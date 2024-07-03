use crate::{common::compilation_state::ScopeNaryTree, Instruction};
use std::collections::HashMap;

pub fn apply(instructions: Vec<Instruction>, scope_tree: ScopeNaryTree) -> Vec<Instruction> {
    let instructions_sans_unused_assigns = remove_unused_assigns(instructions, scope_tree);

    instructions_sans_unused_assigns
}

#[derive(Debug, Clone, PartialEq)]
struct AssignHashValue {
    index: usize,
    is_used: bool,
}

pub fn remove_unused_assigns(
    instructions: Vec<Instruction>,
    scope_tree: ScopeNaryTree,
) -> Vec<Instruction> {
    let mut tagged_instructions: HashMap<usize, bool> = HashMap::new();
    let mut assign_hash: HashMap<String, AssignHashValue> = HashMap::new();

    let get_rid_off = true;
    let should_keep = false;

    instructions
        .clone()
        .into_iter()
        .enumerate()
        .for_each(|(index, instruction)| {
            tagged_instructions.insert(index, get_rid_off);

            let assigned_value =
                &key_for_assign_hash(instruction.scope, instruction.assign.clone());

            if instruction.name == "assign"
                && instruction.assign != ""
                && !instruction.assign.contains(".")
            // if instruction.name == "assign" && instruction.assign != ""
            // && instruction.assign.to_uppercase() == instruction.assign
            {
                if assign_hash.contains_key(assigned_value) {
                    let the_value: &AssignHashValue = assign_hash.get(assigned_value).unwrap();

                    tagged_instructions.entry(the_value.index).and_modify(|e| {
                        *e = if the_value.is_used {
                            should_keep
                        } else {
                            get_rid_off
                        }
                    });

                    assign_hash.remove(assigned_value);
                } else {
                    let input = instruction.assign.clone();
                    assign_hash.clone().iter().for_each(|(key, _value)| {
                        let key_parts: Vec<&str> = key.split("|").collect();
                        let scope = key_parts[0].parse::<u128>().unwrap();
                        let name = key_parts[1].to_string();

                        if input == name
                            && (scope_tree.is_child_of(scope, instruction.scope)
                                || scope == instruction.scope)
                        {
                            assign_hash.entry(key.to_string()).and_modify(|e| {
                                e.is_used = true;
                            });
                        }
                    });
                }

                assign_hash.insert(
                    assigned_value.to_string(),
                    AssignHashValue {
                        index: index,
                        is_used: false,
                    },
                );

                // }
            } else {
                // if you had an assign but then you have an instruction that re-assigns (not in an assign instruction,0 ), remove the assign
                if assign_hash.contains_key(assigned_value) {
                    let the_value: &AssignHashValue = assign_hash.get(assigned_value).unwrap();

                    tagged_instructions.entry(the_value.index).and_modify(|e| {
                        *e = if the_value.is_used {
                            should_keep
                        } else {
                            get_rid_off
                        }
                    });

                    assign_hash.remove(assigned_value);
                }

                tagged_instructions
                    .entry(index)
                    .and_modify(|e| *e = should_keep);

                instruction.input.clone().into_iter().for_each(|input| {
                    assign_hash.clone().iter().for_each(|(key, _value)| {
                        let key_parts: Vec<&str> = key.split("|").collect();
                        let scope = key_parts[0].parse::<u128>().unwrap();
                        let name = key_parts[1].to_string();

                        if input == name
                            && (scope_tree.is_child_of(scope, instruction.scope)
                                || scope == instruction.scope)
                        {
                            assign_hash.entry(key.to_string()).and_modify(|e| {
                                e.is_used = true;
                            });
                        }
                    });
                });
            }
        });

    apply_assign_hash_removal_to_tagged_instructions(&assign_hash, &mut tagged_instructions);

    instructions
        .into_iter()
        .enumerate()
        .filter(|(index, _)| tagged_instructions.get(index).unwrap() == &should_keep)
        .map(|(_, instruction)| instruction)
        .collect()
}

fn apply_assign_hash_removal_to_tagged_instructions(
    assign_hash: &HashMap<String, AssignHashValue>,
    tagged_instructions: &mut HashMap<usize, bool>,
) {
    assign_hash.into_iter().for_each(|(_key, value)| {
        tagged_instructions
            .entry(value.index)
            .and_modify(|e| *e = if value.is_used { false } else { true });
    });
}

fn key_for_assign_hash(scope: u128, name: String) -> String {
    format!("{}|{}", scope, name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimize::create_instruction_with_scope;

    #[test]
    fn remove_unused_assigns_removes_unused_assigns_simple() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "AA", 0),
            create_instruction_with_scope("assign", vec!["2"], "BB", 0),
            create_instruction_with_scope("add", vec!["5", "3"], "CC", 0),
            create_instruction_with_scope("add", vec!["CC", "AA"], "DD", 0),
            create_instruction_with_scope("add", vec!["CC", "10"], "EE", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "AA", 0),
            create_instruction_with_scope("add", vec!["5", "3"], "CC", 0),
            create_instruction_with_scope("add", vec!["CC", "AA"], "DD", 0),
            create_instruction_with_scope("add", vec!["CC", "10"], "EE", 0),
        ];

        assert_eq!(
            remove_unused_assigns(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]
    fn remove_unused_assigns_removes_unused_assigns_complex() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "AA", 0),
            create_instruction_with_scope("assign", vec!["2"], "BB", 0),
            create_instruction_with_scope("add", vec!["5", "3"], "CC", 0),
            create_instruction_with_scope("assign", vec!["11"], "AA", 0),
            create_instruction_with_scope("add", vec!["CC", "AA"], "DD", 0),
            create_instruction_with_scope("assign", vec!["10"], "AA", 0),
            create_instruction_with_scope("add", vec!["CC", "10"], "EE", 0),
            create_instruction_with_scope("assign", vec!["DD"], "FF", 0),
            create_instruction_with_scope("add", vec!["FF", "EE"], "GG", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("add", vec!["5", "3"], "CC", 0),
            create_instruction_with_scope("assign", vec!["11"], "AA", 0),
            create_instruction_with_scope("add", vec!["CC", "AA"], "DD", 0),
            create_instruction_with_scope("add", vec!["CC", "10"], "EE", 0),
            create_instruction_with_scope("assign", vec!["DD"], "FF", 0),
            create_instruction_with_scope("add", vec!["FF", "EE"], "GG", 0),
        ];

        assert_eq!(
            remove_unused_assigns(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]
    fn remove_unused_assigns_due_to_eval_after() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("assign", vec!["1"], "AA", 0),
            create_instruction_with_scope("evaluate", vec!["1", "BB"], "AA", 0),
            create_instruction_with_scope("add", vec!["AA", "BB"], "DD", 0),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("evaluate", vec!["1", "BB"], "AA", 0),
            create_instruction_with_scope("add", vec!["AA", "BB"], "DD", 0),
        ];

        assert_eq!(
            remove_unused_assigns(unoptimized_instructions, ScopeNaryTree::new('a')),
            expected_optimized_instructions
        );
    }

    #[test]

    fn does_not_remove_assigns_when_defined_in_other_scope_and_used_later() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope(
                "assign",
                vec!["is_answer_to_life"],
                "CONDITIONAL_JUMP_ASSIGNMENT_1",
                0,
            ),
            create_instruction_with_scope(
                "jump",
                vec!["CONDITIONAL_JUMP_ASSIGNMENT_1", "7"],
                "",
                0,
            ),
            create_instruction_with_scope("assign", vec!["42"], "RETURN_VALUE_LABEL_0", 3),
            create_instruction_with_scope("jump", vec!["0"], "", 3),
            create_instruction_with_scope("assign", vec!["40"], "RETURN_VALUE_LABEL_0", 7),
            create_instruction_with_scope("jump", vec!["0"], "", 7),
            create_instruction_with_scope(
                "return",
                vec!["RETURN_VALUE_LABEL_0"],
                "Thing_to_return",
                0,
            ),
            create_instruction_with_scope("return", vec!["Thing_to_return"], "", 0),
        ];

        //    0
        //    |
        //    3
        //    |
        //    7
        let mut scope_tree = ScopeNaryTree::new('a');
        scope_tree.push(0, 3);
        scope_tree.push(3, 7);

        assert_eq!(
            remove_unused_assigns(unoptimized_instructions.clone(), scope_tree),
            unoptimized_instructions
        );
    }

    #[test]
    fn do_a_thing() {
        let unoptimized_instructions = [
            create_instruction_with_scope(
                "evaluate",
                vec!["env.storage"],
                "METHOD_CALL_EXPRESSION_6",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_6.instance"],
                "METHOD_CALL_EXPRESSION_5",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_5.get", "COUNTER"],
                "METHOD_CALL_EXPRESSION_2",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_2.unwrap_or", "0"],
                "count|||Integer",
                0,
            ),
            create_instruction_with_scope("print", vec!["env", "count: {}", "count"], "", 0),
            create_instruction_with_scope("add", vec!["count", "1"], "count", 0),
            create_instruction_with_scope(
                "evaluate",
                vec!["env.storage"],
                "METHOD_CALL_EXPRESSION_24",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_24.instance"],
                "METHOD_CALL_EXPRESSION_23",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_23.set", "COUNTER", "count"],
                "",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["env.storage"],
                "METHOD_CALL_EXPRESSION_35",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_35.instance"],
                "METHOD_CALL_EXPRESSION_34",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_34.extend_ttl", "50", "100"],
                "",
                0,
            ),
            create_instruction_with_scope("assign", vec!["count"], "Thing_to_return", 0),
            create_instruction_with_scope("return", vec!["count"], "", 0),
        ];

        let expected_instructions = [
            create_instruction_with_scope(
                "evaluate",
                vec!["env.storage"],
                "METHOD_CALL_EXPRESSION_6",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_6.instance"],
                "METHOD_CALL_EXPRESSION_5",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_5.get", "COUNTER"],
                "METHOD_CALL_EXPRESSION_2",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_2.unwrap_or", "0"],
                "count|||Integer",
                0,
            ),
            create_instruction_with_scope("print", vec!["env", "count: {}", "count"], "", 0),
            create_instruction_with_scope("add", vec!["count", "1"], "count", 0),
            create_instruction_with_scope(
                "evaluate",
                vec!["env.storage"],
                "METHOD_CALL_EXPRESSION_24",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_24.instance"],
                "METHOD_CALL_EXPRESSION_23",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_23.set", "COUNTER", "count"],
                "",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["env.storage"],
                "METHOD_CALL_EXPRESSION_35",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_35.instance"],
                "METHOD_CALL_EXPRESSION_34",
                0,
            ),
            create_instruction_with_scope(
                "evaluate",
                vec!["METHOD_CALL_EXPRESSION_34.extend_ttl", "50", "100"],
                "",
                0,
            ),
            create_instruction_with_scope("return", vec!["count"], "", 0),
        ];

        assert_eq!(
            remove_unused_assigns(unoptimized_instructions.to_vec(), ScopeNaryTree::new('a')),
            expected_instructions
        );
    }
}
