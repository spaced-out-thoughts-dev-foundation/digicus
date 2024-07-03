use crate::Instruction;

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let filtered_instructions: Vec<Instruction> = instructions
        .clone()
        .into_iter()
        .filter(|instruction| {
            !(instruction.input.len() == 1 && instruction.assign == instruction.input[0])
        })
        .collect();

    remove_unused_returns(filtered_instructions)
}

pub fn remove_unused_returns(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut scope_stack: Vec<(u128, bool)> = vec![];

    instructions
        .into_iter()
        .filter(|instruction| {
            // changing scope
            if scope_stack.is_empty() {
                scope_stack.push((
                    instruction.scope,
                    instruction.name == "return" || instruction.name == "return",
                ));
                return true;
            }

            let mut last_element_index = scope_stack.len() - 1;
            if scope_stack[last_element_index].0 != instruction.scope {
                // Going down to a deeper scope
                if scope_stack[last_element_index].0 < instruction.scope {
                    scope_stack.push((
                        instruction.scope,
                        instruction.name == "return" || instruction.name == "return",
                    ));
                    return true;
                } else {
                    scope_stack.pop();
                    last_element_index -= 1;
                }
                // At same scope
            }

            if scope_stack[last_element_index].1 {
                return false;
            }

            if instruction.name == "return" || instruction.name == "return" {
                scope_stack[last_element_index].1 = true;
            }
            return true;
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimize::{create_instruction, create_instruction_with_scope};

    #[test]
    fn remove_multi_return_at_same_scope_lowercase_return_single_scope() {
        let unoptimized_instructions = vec![
            create_instruction("return", vec!["1"], ""),
            create_instruction("return", vec!["1", "b"], "a"),
            create_instruction("return", vec!["a", "b"], "d"),
        ];

        let expected_optimized_instructions = vec![create_instruction("return", vec!["1"], "")];

        assert_eq!(
            remove_unused_returns(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn remove_multi_return_at_same_scope_uppercase_return_single_scope() {
        let unoptimized_instructions = vec![
            create_instruction("return", vec!["1"], ""),
            create_instruction("return", vec!["1", "b"], "a"),
            create_instruction("return", vec!["a", "b"], "d"),
        ];

        let expected_optimized_instructions = vec![create_instruction("return", vec!["1"], "")];

        assert_eq!(
            remove_unused_returns(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn remove_multi_return_at_same_scope_uppercase_return_multi_scope() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope("return", vec![], "a", 1),
            create_instruction_with_scope("return", vec![], "b", 2),
            create_instruction_with_scope("return", vec![], "c", 1),
            create_instruction_with_scope("return", vec![], "d", 2),
            create_instruction_with_scope("return", vec![], "e", 2),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope("return", vec![], "a", 1),
            create_instruction_with_scope("return", vec![], "b", 2),
            create_instruction_with_scope("return", vec![], "d", 2),
        ];

        assert_eq!(
            remove_unused_returns(unoptimized_instructions),
            expected_optimized_instructions
        );
    }
}
