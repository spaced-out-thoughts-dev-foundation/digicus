use crate::{common::compilation_state::ScopeNaryTree, Instruction};
use std::collections::HashMap;

pub fn apply(instructions: Vec<Instruction>, scope_tree: ScopeNaryTree) -> Vec<Instruction> {
    let mut instruction_hash_table: HashMap<String, Vec<(u128, Vec<String>)>> = HashMap::new();
    let mut optimized_instructions: Vec<Instruction> = Vec::new();

    let mut scope_stack: Vec<u128> = vec![0];

    let mut current_scope: u128 = instructions[0].scope;
    instructions
        .clone()
        .into_iter()
        .for_each(|mut instruction| {
            current_scope = instruction.scope;
            let mut keys_to_remove: HashMap<String, bool> = HashMap::new();

            let new_inputs = handle_inputs(
                instruction.clone(),
                &instruction_hash_table,
                &mut keys_to_remove,
                current_scope,
                scope_tree.clone(),
            );

            let new_assign = if (instruction.assign != ""
                && instruction.assign.to_uppercase() == instruction.assign)
                && instruction_hash_table.contains_key(&instruction.assign)
            {
                handle_assigns(
                    instruction.clone(),
                    &instruction_hash_table,
                    &mut keys_to_remove,
                    current_scope,
                    scope_tree.clone(),
                )
            } else {
                instruction.assign.clone()
            };

            // rewrite fields as assigns
            if instruction.name == "field" {
                instruction = rewrite_field_instruction_as_assign(
                    instruction,
                    new_inputs.clone(),
                    new_assign.clone(),
                );
            }

            keys_to_remove.into_iter().for_each(|(key, _)| {
                remove_key_and_scope_from_instruction_hash(
                    &mut instruction_hash_table,
                    key,
                    instruction.scope,
                )
            });

            if current_scope != scope_stack[scope_stack.len() - 1] {
                if scope_stack.len() <= 1 {
                    scope_stack.push(current_scope);
                } else {
                    if scope_stack[scope_stack.len() - 2] != current_scope {
                        scope_stack.push(current_scope);
                    } else {
                        let scope_to_remove = scope_stack.pop().unwrap();

                        instruction_hash_table = instruction_hash_table
                            .clone()
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    key,
                                    value
                                        .into_iter()
                                        .filter(|(s, _)| *s != scope_to_remove)
                                        .collect(),
                                )
                            })
                            .collect();

                        instruction_hash_table.retain(|_key, value| value.len() > 0);
                    }
                }
            }

            if instruction.name == "assign" {
                if instruction_hash_table.contains_key(&instruction.assign) {
                    let mut scope_values: Vec<(u128, Vec<String>)> = instruction_hash_table
                        .get(&instruction.assign)
                        .unwrap()
                        .clone();
                    scope_values.retain(|(s, _)| *s != instruction.scope);
                    scope_values.push((instruction.scope, instruction.input.clone()));
                    instruction_hash_table.insert(instruction.assign.clone(), scope_values.clone());
                } else {
                    instruction_hash_table.insert(
                        instruction.assign.clone(),
                        vec![(instruction.scope, instruction.input.clone())],
                    );
                }
            } else {
                // if you had an assign but then you have an instruction that re-assigns (not in an assign instruction), remove the assign
                remove_key_and_scope_from_instruction_hash(
                    &mut instruction_hash_table,
                    instruction.assign.clone(),
                    instruction.scope,
                )
            }

            optimized_instructions.push(Instruction::new(
                instruction.id,
                instruction.name.clone(),
                new_inputs.clone(),
                new_assign.clone(),
                instruction.scope,
            ));
        });

    optimized_instructions
}

fn remove_key_and_scope_from_instruction_hash(
    instruction_hash_table: &mut HashMap<String, Vec<(u128, Vec<String>)>>,
    key: String,
    scope: u128,
) {
    if instruction_hash_table.contains_key(&key) {
        let mut scope_values = instruction_hash_table.get(&key).unwrap().clone();
        scope_values.retain(|(s, _)| *s != scope);

        if scope_values.len() > 0 {
            instruction_hash_table.insert(key.clone(), scope_values);
        } else {
            instruction_hash_table.remove(&key);
        }
    }
}

fn rewrite_field_instruction_as_assign(
    instruction: Instruction,
    new_inputs: Vec<String>,
    new_assign: String,
) -> Instruction {
    Instruction::new(
        instruction.id,
        "assign".to_string(),
        vec![new_inputs.join(".").clone()],
        new_assign.clone(),
        instruction.scope,
    )
}

fn handle_assigns(
    instruction: Instruction,
    instruction_hash_table: &HashMap<String, Vec<(u128, Vec<String>)>>,
    keys_to_remove: &mut HashMap<String, bool>,
    current_scope: u128,
    scope_tree: ScopeNaryTree,
) -> String {
    return if (instruction.assign != "" && instruction.assign.to_uppercase() == instruction.assign)
        && instruction_hash_table.contains_key(&instruction.assign)
    {
        keys_to_remove.insert(instruction.assign.clone(), true);
        // TODO: make sure this is correct
        let foo_bar = instruction_hash_table.get(&instruction.assign).unwrap();

        let mut index: i32 = foo_bar.len() as i32 - 1;
        while index >= 0 {
            let scope = foo_bar[index as usize].0;
            if scope == current_scope || scope_tree.is_child_of(current_scope, scope) {
                let new_input: Vec<String> = foo_bar[foo_bar.len() - 1].1.clone();
                return new_input.join(".");
            }

            index -= 1;
        }

        instruction.assign.clone()
    } else {
        instruction.assign.clone()
    };
}

fn handle_inputs(
    instruction: Instruction,
    instruction_hash_table: &HashMap<String, Vec<(u128, Vec<String>)>>,
    keys_to_remove: &mut HashMap<String, bool>,
    current_scope: u128,
    scope_tree: ScopeNaryTree,
) -> Vec<String> {
    instruction
        .input
        .clone()
        .into_iter()
        .map(|input| {
            handle_single_input(
                input,
                instruction_hash_table,
                keys_to_remove,
                current_scope,
                scope_tree.clone(),
            )
        })
        .flatten()
        .collect::<Vec<String>>()
}

fn handle_single_input(
    input: String,
    instruction_hash_table: &HashMap<String, Vec<(u128, Vec<String>)>>,
    keys_to_remove: &mut HashMap<String, bool>,
    current_scope: u128,
    scope_tree: ScopeNaryTree,
) -> Vec<String> {
    if instruction_hash_table.contains_key(&input) {
        keys_to_remove.insert(input.clone(), true);

        let foo_bar = instruction_hash_table.get(&input).unwrap().clone();

        let mut index: i32 = foo_bar.len() as i32 - 1;
        while index >= 0 {
            let scope = foo_bar[index as usize].0;
            if scope == current_scope || scope_tree.is_child_of(current_scope, scope) {
                return foo_bar[foo_bar.len() - 1].1.clone();
            }

            index -= 1;
        }

        return vec![input];
    } else {
        let splitted_input_string: Vec<&str> = input.split('.').collect();

        if splitted_input_string.len() > 0 {
            let base_object = splitted_input_string[0];
            if instruction_hash_table.contains_key(base_object) {
                keys_to_remove.insert(input.clone(), true);
                let foo_bar = instruction_hash_table.get(base_object).unwrap().clone();

                let mut index: i32 = foo_bar.len() as i32 - 1;
                while index >= 0 {
                    let scope = foo_bar[index as usize].0;
                    if scope == current_scope || scope_tree.is_child_of(current_scope, scope) {
                        let mut new_input: Vec<String> = foo_bar[foo_bar.len() - 1].1.clone();
                        new_input.push(splitted_input_string[1].to_string());
                        return vec![new_input.join(".")];
                    }

                    index -= 1;
                }

                return vec![input];
            } else {
                return vec![input];
            }
        } else {
            return vec![input];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimize::create_instruction_with_scope;

    #[test]
    fn constant_propagation_only_propagates_if_assign() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("add", vec!["a", "2"], "b", 0),
            create_instruction_with_scope("add", vec!["b", "3"], "c", 0),
            create_instruction_with_scope("add", vec!["c", "4"], "d", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("add", vec!["1", "2"], "b", 0),
            create_instruction_with_scope("add", vec!["b", "3"], "c", 0),
            create_instruction_with_scope("add", vec!["c", "4"], "d", 0),
        ];

        assert_eq!(
            apply(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_latest_value() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("assign", vec!["5"], "a", 0),
            create_instruction_with_scope("add", vec!["a", "2"], "b", 0),
            create_instruction_with_scope("add", vec!["c", "4"], "d", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("assign", vec!["5"], "a", 0),
            create_instruction_with_scope("add", vec!["5", "2"], "b", 0),
            create_instruction_with_scope("add", vec!["c", "4"], "d", 0),
        ];

        assert_eq!(
            apply(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_multiple_values() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("assign", vec!["5"], "b", 0),
            create_instruction_with_scope("add", vec!["a", "b"], "c", 0),
            create_instruction_with_scope("add", vec!["c", "4"], "d", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("assign", vec!["5"], "b", 0),
            create_instruction_with_scope("add", vec!["1", "5"], "c", 0),
            create_instruction_with_scope("add", vec!["c", "4"], "d", 0),
        ];

        assert_eq!(
            apply(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_multiple_values_with_multiple_assigns() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("assign", vec!["5"], "b", 0),
            create_instruction_with_scope("add", vec!["a", "b"], "c", 0),
            create_instruction_with_scope("assign", vec!["7"], "b", 0),
            create_instruction_with_scope("add", vec!["a", "b"], "d", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("assign", vec!["5"], "b", 0),
            create_instruction_with_scope("add", vec!["1", "5"], "c", 0),
            create_instruction_with_scope("assign", vec!["7"], "b", 0),
            create_instruction_with_scope("add", vec!["a", "7"], "d", 0),
        ];

        assert_eq!(
            apply(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_same_value_multiple_times() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("add", vec!["a", "b"], "c", 0),
            create_instruction_with_scope("add", vec!["c", "a"], "d", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("add", vec!["1", "b"], "c", 0),
            create_instruction_with_scope("add", vec!["c", "a"], "d", 0),
        ];

        assert_eq!(
            apply(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_does_not_propagate_after_evaluate_reassignment() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("add", vec!["a", "b"], "c", 0),
            create_instruction_with_scope("evaluate", vec!["5", "a"], "a", 0),
            create_instruction_with_scope("add", vec!["a", "b"], "d", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "a", 0),
            create_instruction_with_scope("add", vec!["1", "b"], "c", 0),
            create_instruction_with_scope("evaluate", vec!["5", "a"], "a", 0),
            create_instruction_with_scope("add", vec!["a", "b"], "d", 0),
        ];

        assert_eq!(
            apply(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_works_for_method_call_base_object() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["foo"], "a", 0),
            create_instruction_with_scope("evaluate", vec!["a.bar"], "b", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["foo"], "a", 0),
            create_instruction_with_scope("evaluate", vec!["foo.bar"], "b", 0),
        ];

        assert_eq!(
            apply(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]
    fn rewrite_field_instruction_as_assign_works() {
        let instruction = create_instruction_with_scope("field", vec!["a", "b"], "c", 0);
        let new_assign = "new_assign".to_string();

        let expected_instruction =
            create_instruction_with_scope("assign", vec!["a.b"], &new_assign, 0);

        assert_eq!(
            rewrite_field_instruction_as_assign(
                instruction.clone(),
                instruction.input.clone(),
                new_assign,
            ),
            expected_instruction
        );
    }

    #[test]
    fn constant_propagation_does_not_violate_scope() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "AAA", 0),
            create_instruction_with_scope("assign", vec!["AAA"], "BBB", 2),
            create_instruction_with_scope("assign", vec!["3"], "BBB", 3),
            create_instruction_with_scope("evaluate", vec!["BBB"], "To_Return", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "AAA", 0),
            create_instruction_with_scope("assign", vec!["1"], "BBB", 2),
            create_instruction_with_scope("assign", vec!["3"], "BBB", 3),
            create_instruction_with_scope("evaluate", vec!["BBB"], "To_Return", 0),
        ];

        //   0
        //  / \
        // 2   3
        let mut scope_tree = ScopeNaryTree::new('a');
        scope_tree.push(0, 2);
        scope_tree.push(0, 3);

        assert_eq!(
            apply(unoptimized_instructions, scope_tree.clone()),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_uses_value_from_lowest_scope_less_than_or_equal_to_self() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "AAA", 0),
            create_instruction_with_scope("assign", vec!["AAA"], "bbb", 2),
            create_instruction_with_scope("assign", vec!["3"], "bbb", 3),
            create_instruction_with_scope("assign", vec!["4"], "bbb", 4),
            create_instruction_with_scope("assign", vec!["5"], "CCC", 4),
            create_instruction_with_scope("evaluate", vec!["bbb", "CCC"], "DDD", 5),
            create_instruction_with_scope("assign", vec!["bbb"], "To_Return", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "AAA", 0),
            create_instruction_with_scope("assign", vec!["1"], "bbb", 2),
            create_instruction_with_scope("assign", vec!["3"], "bbb", 3),
            create_instruction_with_scope("assign", vec!["4"], "bbb", 4),
            create_instruction_with_scope("assign", vec!["5"], "CCC", 4),
            create_instruction_with_scope("evaluate", vec!["4", "5"], "DDD", 5),
            create_instruction_with_scope("assign", vec!["bbb"], "To_Return", 0),
        ];

        //          0
        //         / \
        //        3   4
        //       /
        //      4
        //     /
        //    5
        let mut scope_tree = ScopeNaryTree::new('a');
        scope_tree.push(0, 2);
        scope_tree.push(0, 3);
        scope_tree.push(3, 4);
        scope_tree.push(4, 5);

        assert_eq!(
            apply(unoptimized_instructions, scope_tree.clone()),
            expected_optimized_instructions
        );
    }
}
