use crate::Instruction;
use std::collections::HashMap;

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut instruction_hash_table: HashMap<String, Vec<String>> = HashMap::new();
    let mut optimized_instructions: Vec<Instruction> = Vec::new();

    instructions
        .clone()
        .into_iter()
        .for_each(|mut instruction| {
            let mut keys_to_remove: HashMap<String, bool> = HashMap::new();

            let mut new_inputs: Vec<String> = Vec::new();
            instruction.input.clone().into_iter().for_each(|input| {
                if instruction_hash_table.contains_key(&input) {
                    keys_to_remove.insert(input.clone(), true);
                    new_inputs.extend(instruction_hash_table.get(&input).unwrap().clone());
                } else {
                    let splitted_input_string: Vec<&str> = input.split('.').collect();

                    if splitted_input_string.len() > 0 {
                        let base_object = splitted_input_string[0];
                        if instruction_hash_table.contains_key(base_object) {
                            keys_to_remove.insert(input.clone(), true);
                            let mut new_input =
                                instruction_hash_table.get(base_object).unwrap().clone();
                            new_input.push(splitted_input_string[1].to_string());
                            new_inputs.push(new_input.join("."));
                        } else {
                            new_inputs.push(input);
                        }
                    } else {
                        new_inputs.push(input);
                    }
                }
            });

            let new_assign = if (instruction.assign != ""
                && instruction.assign.to_uppercase() == instruction.assign)
                && instruction_hash_table.contains_key(&instruction.assign)
            {
                keys_to_remove.insert(instruction.assign.clone(), true);
                // TODO: make sure this is correct
                instruction_hash_table
                    .get(&instruction.assign)
                    .unwrap()
                    .join(".")
            } else {
                instruction.assign.clone()
            };

            // rewrite fields as assigns
            if instruction.name == "field" {
                new_inputs = vec![new_inputs.join(".")];

                instruction = Instruction::new(
                    instruction.id,
                    "assign".to_string(),
                    new_inputs.clone(),
                    new_assign.clone(),
                    instruction.scope,
                );
            }

            keys_to_remove.into_iter().for_each(|(key, _)| {
                instruction_hash_table.remove(&key);
            });

            if instruction.name == "assign" {
                instruction_hash_table
                    .insert(instruction.assign.clone(), instruction.input.clone());
            }

            if instruction.name != "assign" {
                // if you had an assign but then you have an instruction that re-assigns (not in an assign instruction), remove the assign
                if instruction_hash_table.contains_key(&instruction.assign) {
                    instruction_hash_table.remove_entry(&instruction.assign);
                }
            }

            let optimized_instruction = Instruction::new(
                instruction.id,
                instruction.name.clone(),
                new_inputs,
                new_assign.clone(),
                instruction.scope,
            );

            optimized_instructions.push(optimized_instruction);
        });

    optimized_instructions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimize::create_instruction;

    #[test]
    fn constant_propagation_only_propagates_if_assign() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["a", "2"], "b"),
            create_instruction("add", vec!["b", "3"], "c"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["1", "2"], "b"),
            create_instruction("add", vec!["b", "3"], "c"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_latest_value() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "a"),
            create_instruction("add", vec!["a", "2"], "b"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "a"),
            create_instruction("add", vec!["5", "2"], "b"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_multiple_values() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "b"),
            create_instruction("add", vec!["a", "b"], "c"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "b"),
            create_instruction("add", vec!["1", "5"], "c"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_multiple_values_with_multiple_assigns() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "b"),
            create_instruction("add", vec!["a", "b"], "c"),
            create_instruction("assign", vec!["7"], "b"),
            create_instruction("add", vec!["a", "b"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "b"),
            create_instruction("add", vec!["1", "5"], "c"),
            create_instruction("assign", vec!["7"], "b"),
            create_instruction("add", vec!["a", "7"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_same_value_multiple_times() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["a", "b"], "c"),
            create_instruction("add", vec!["c", "a"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["1", "b"], "c"),
            create_instruction("add", vec!["c", "a"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_does_not_propagate_after_evaluate_reassignment() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["a", "b"], "c"),
            create_instruction("evaluate", vec!["5", "a"], "a"),
            create_instruction("add", vec!["a", "b"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["1", "b"], "c"),
            create_instruction("evaluate", vec!["5", "a"], "a"),
            create_instruction("add", vec!["a", "b"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_works_for_method_call_base_object() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["foo"], "a"),
            create_instruction("evaluate", vec!["a.bar"], "b"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["foo"], "a"),
            create_instruction("evaluate", vec!["foo.bar"], "b"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }
}
