use std::collections::HashMap;

use crate::instruction::{self, Instruction};

struct UnreachableCodeElimination {
    instructions: Vec<Instruction>,
    index: usize,
    // <scope, visited>
    visited: HashMap<u128, bool>,
}

impl UnreachableCodeElimination {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            index: 0,
            visited: HashMap::new(),
        }
    }

    fn visit(&mut self) {
        let mut visited: HashMap<u128, bool> = HashMap::new();
        // <index, scope>
        let mut stack: Vec<(usize, u128)> = Vec::new();
        let mut memoize: HashMap<String, bool> = HashMap::new();

        stack.push((0, 0));

        while !stack.is_empty() {
            let top_element = stack.pop().unwrap();

            let current_index = top_element.0;
            let current_scope = top_element.1;

            if memoize.contains_key(&format!("{}_{}", current_index, current_scope)) {
                continue;
            } else {
                memoize.insert(format!("{}_{}", current_index, current_scope), true);
            }

            let current_instruction = self.instructions[current_index].clone();

            let next_index = current_index + 1;

            // we are at a different scope and thus we skip this instruction, however we still continue
            // down the sequence
            if current_instruction.scope != current_scope {
                if next_index >= self.instructions.len() {
                    continue;
                }

                stack.push((next_index, current_scope));
                continue;
            };

            visited.insert(current_instruction.id, true);
            match current_instruction.name.as_str() {
                "jump" => {
                    let is_conditional_jump = current_instruction.input.len() > 1;

                    if is_conditional_jump {
                        let jump_scope = current_instruction.input[1].parse::<u128>().unwrap();

                        if next_index < self.instructions.len() {
                            // whether conditional or unconditional, we will have _at least_ the possibility of going to the jump scope
                            stack.push((next_index, jump_scope));
                            // the condition might not hold, thus we might continue in the current scope
                            stack.push((next_index, current_scope));
                        }
                    } else {
                        let jump_scope = current_instruction.input[0].parse::<u128>().unwrap();
                        // whether conditional or unconditional, we will have _at least_ the possibility of going to the jump scope

                        if next_index < self.instructions.len() {
                            stack.push((next_index, jump_scope));
                        }
                    }
                }
                "goto" => {
                    // a goto is always unconditional
                    let goto_instruction_id = current_instruction.input[0].parse::<u128>().unwrap();

                    let mut index = 0;
                    for instruction in self.instructions.iter() {
                        if instruction.id == goto_instruction_id {
                            stack.push((index, instruction.scope));
                            break;
                        }

                        index += 1;
                    }
                }
                "return" => {
                    // we are at the end of the function, no need to continue
                }
                _ => {
                    if next_index < self.instructions.len() {
                        stack.push((next_index, current_scope));
                    }
                }
            }
        }

        self.visited = visited;
    }
}

#[cfg(test)]
mod visited_tests {
    use std::collections::HashMap;

    use crate::instruction::{self, Instruction};
    use crate::optimize::unreachable_code_elimination::UnreachableCodeElimination;

    #[test]
    fn simple_sequential() {
        let instructions = vec![
            Instruction::new(
                1,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                2,
                "sub".to_string(),
                vec!["c".to_string(), "1".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                3,
                "return".to_string(),
                vec!["c".to_string()],
                "".to_string(),
                0,
            ),
        ];

        let mut unreachable_code_elimination = UnreachableCodeElimination::new(instructions);
        unreachable_code_elimination.visit();

        println!("{:?}", unreachable_code_elimination.visited);

        assert_eq!(unreachable_code_elimination.visited.len(), 3);
    }

    #[test]
    fn simple_one_scope_dead_code_after_return() {
        let instructions = vec![
            Instruction::new(
                1,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                2,
                "sub".to_string(),
                vec!["c".to_string(), "1".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                3,
                "return".to_string(),
                vec!["c".to_string()],
                "".to_string(),
                0,
            ),
            Instruction::new(
                4,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                0,
            ),
        ];

        let mut unreachable_code_elimination = UnreachableCodeElimination::new(instructions);
        unreachable_code_elimination.visit();

        assert_eq!(unreachable_code_elimination.visited.len(), 3);
        assert_eq!(unreachable_code_elimination.visited.contains_key(&4), false);
    }

    #[test]
    fn simple_one_scope_dead_code_after_jump() {
        let instructions = vec![
            Instruction::new(
                1,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                2,
                "sub".to_string(),
                vec!["c".to_string(), "1".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                3,
                "jump".to_string(),
                vec!["5".to_string()],
                "".to_string(),
                0,
            ),
            Instruction::new(
                4,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                5,
                "sub".to_string(),
                vec!["c".to_string(), "1".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                6,
                "return".to_string(),
                vec!["c".to_string()],
                "".to_string(),
                0,
            ),
        ];

        let mut unreachable_code_elimination = UnreachableCodeElimination::new(instructions);
        unreachable_code_elimination.visit();

        assert_eq!(unreachable_code_elimination.visited.len(), 3);
        assert_eq!(unreachable_code_elimination.visited.contains_key(&4), false);
    }

    #[test]
    fn simple_one_scope_goto_skips_instruction() {
        let instructions = vec![
            Instruction::new(
                1,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                2,
                "sub".to_string(),
                vec!["c".to_string(), "1".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                3,
                "goto".to_string(),
                vec!["5".to_string()],
                "".to_string(),
                0,
            ),
            Instruction::new(
                4,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                5,
                "sub".to_string(),
                vec!["c".to_string(), "1".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                6,
                "return".to_string(),
                vec!["c".to_string()],
                "".to_string(),
                0,
            ),
        ];

        let mut unreachable_code_elimination = UnreachableCodeElimination::new(instructions);
        unreachable_code_elimination.visit();

        assert_eq!(unreachable_code_elimination.visited.len(), 5);
        assert_eq!(unreachable_code_elimination.visited.contains_key(&4), false);
    }

    #[test]
    fn conditional_jumps() {
        let instructions = vec![
            Instruction::new(
                1,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                2,
                "jump".to_string(),
                vec!["some_condition".to_string(), "1".to_string()],
                "".to_string(),
                0,
            ),
            Instruction::new(
                3,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                1,
            ),
            Instruction::new(
                4,
                "jump".to_string(),
                vec!["some_condition".to_string(), "2".to_string()],
                "".to_string(),
                1,
            ),
            Instruction::new(
                5,
                "print".to_string(),
                vec!["2".to_string()],
                "".to_string(),
                2,
            ),
            Instruction::new(
                6,
                "jump".to_string(),
                vec!["0".to_string()],
                "".to_string(),
                2,
            ),
            Instruction::new(
                7,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                1,
            ),
            Instruction::new(
                8,
                "return".to_string(),
                vec!["c".to_string()],
                "".to_string(),
                1,
            ),
            Instruction::new(
                9,
                "return".to_string(),
                vec!["d".to_string()],
                "".to_string(),
                1,
            ),
            Instruction::new(
                10,
                "sub".to_string(),
                vec!["c".to_string(), "1".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                11,
                "return".to_string(),
                vec!["c".to_string()],
                "".to_string(),
                0,
            ),
            Instruction::new(
                12,
                "assign".to_string(),
                vec!["1".to_string()],
                "c".to_string(),
                0,
            ),
        ];

        let expected_visited: HashMap<u128, bool> = HashMap::from_iter(vec![
            (1, true),
            (2, true),
            (3, true),
            (4, true),
            (5, true),
            (6, true),
            (7, true),
            (8, true),
            (10, true),
            (11, true),
        ]);

        let mut unreachable_code_elimination = UnreachableCodeElimination::new(instructions);

        unreachable_code_elimination.visit();

        assert_eq!(unreachable_code_elimination.visited, expected_visited);
    }

    #[test]
    fn handles_simple_for_loop() {
        let instructions: Vec<Instruction> = vec![
            Instruction::new(
                3,
                "assign".to_string(),
                vec!["xs".to_string()],
                "METHOD_CALL_EXPRESSION_2".to_string(),
                0,
            ),
            Instruction::new(
                4,
                "evaluate".to_string(),
                vec!["METHOD_CALL_EXPRESSION_2.iter".to_string()],
                "ITERATOR_0".to_string(),
                0,
            ),
            Instruction::new(
                5,
                "evaluate".to_string(),
                vec!["start".to_string(), "ITERATOR_0".to_string()],
                "x".to_string(),
                0,
            ),
            Instruction::new(
                6,
                "end_of_iteration_check".to_string(),
                vec!["x".to_string(), "ITERATOR_0".to_string()],
                "CHECK_CONDITION_ASSIGNMENT_1".to_string(),
                0,
            ),
            Instruction::new(
                8,
                "jump".to_string(),
                vec!["CHECK_CONDITION_ASSIGNMENT_1".to_string(), "7".to_string()],
                "".to_string(),
                0,
            ),
            Instruction::new(
                9,
                "print".to_string(),
                vec!["\"{}\"".to_string(), "x".to_string()],
                "".to_string(),
                7,
            ),
            Instruction::new(
                10,
                "increment".to_string(),
                vec!["x".to_string()],
                "".to_string(),
                7,
            ),
            Instruction::new(
                11,
                "goto".to_string(),
                vec!["6".to_string()],
                "".to_string(),
                7,
            ),
        ];

        let expected_visited: HashMap<u128, bool> = HashMap::from_iter(vec![
            (3, true),
            (4, true),
            (5, true),
            (6, true),
            (8, true),
            (9, true),
            (10, true),
            (11, true),
        ]);

        let mut unreachable_code_elimination = UnreachableCodeElimination::new(instructions);

        unreachable_code_elimination.visit();

        assert_eq!(unreachable_code_elimination.visited, expected_visited);
    }

    #[test]
    fn handles_goto() {
        let instructions = vec![
            Instruction::new(
                1,
                "jump".to_string(),
                vec!["1".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                2,
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                3,
                "sub".to_string(),
                vec!["c".to_string(), "1".to_string()],
                "c".to_string(),
                0,
            ),
            Instruction::new(
                4,
                "assign".to_string(),
                vec!["1".to_string()],
                "c".to_string(),
                1,
            ),
            Instruction::new(
                5,
                "goto".to_string(),
                vec!["2".to_string()],
                "c".to_string(),
                1,
            ),
            Instruction::new(
                6,
                "return".to_string(),
                vec!["2".to_string()],
                "c".to_string(),
                0,
            ),
        ];

        let expected_visited: HashMap<u128, bool> = HashMap::from_iter(vec![
            (1, true),
            (2, true),
            (3, true),
            (4, true),
            (5, true),
            (6, true),
        ]);

        let mut unreachable_code_elimination = UnreachableCodeElimination::new(instructions);

        unreachable_code_elimination.visit();

        assert_eq!(unreachable_code_elimination.visited, expected_visited);
    }
}

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let filtered_instructions: Vec<Instruction> = instructions
        .clone()
        .into_iter()
        .filter(|instruction| {
            !(instruction.input.len() == 1 && instruction.assign == instruction.input[0])
        })
        .collect();

    let mut optimized_instructions: Vec<Instruction> = vec![];

    let mut unreachable_code_elimination = UnreachableCodeElimination::new(filtered_instructions);
    unreachable_code_elimination.visit();

    let visited = unreachable_code_elimination.visited;

    instructions.into_iter().for_each(|instruction| {
        if visited.contains_key(&instruction.id) {
            optimized_instructions.push(instruction);
        }
    });

    optimized_instructions
}

#[cfg(test)]
mod apply_tests {
    use super::*;
    use crate::instruction::Instruction;
    use crate::optimize::{create_instruction, create_instruction_with_scope_and_id};

    #[test]
    fn test_unreachable_jump_elimination() {
        // 0 -> 1 -> 2 -> 3 -> 1 -> 0
        //           |         |
        //           4        10
        let instructions = vec![
            Instruction::unconditional_jump(0, 1, 1),
            Instruction::unconditional_jump(1, 2, 2),
            Instruction::unconditional_jump(2, 3, 3),
            Instruction::unconditional_jump(2, 4, 4),
            Instruction::unconditional_jump(3, 1, 5),
            Instruction::unconditional_jump(1, 0, 6),
            Instruction::unconditional_jump(1, 10, 7),
        ];

        // 0 -> 1 -> 2 -> 3 -> 1 -> 0
        let expected = vec![
            Instruction::unconditional_jump(0, 1, 1),
            Instruction::unconditional_jump(1, 2, 2),
            Instruction::unconditional_jump(2, 3, 3),
            Instruction::unconditional_jump(3, 1, 5),
            Instruction::unconditional_jump(1, 0, 6),
        ];

        apply(instructions.clone())
            .into_iter()
            .for_each(|x| println!("{:?}", x));

        assert_eq!(apply(instructions), expected);
    }

    #[test]
    fn single_scope_unreachable_return_elimination() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope_and_id("return", vec!["1"], "", 0, 1),
            create_instruction_with_scope_and_id("return", vec!["1", "b"], "a", 0, 2),
            create_instruction_with_scope_and_id("return", vec!["a", "b"], "d", 0, 3),
        ];

        let expected_optimized_instructions = vec![create_instruction_with_scope_and_id(
            "return",
            vec!["1"],
            "",
            0,
            1,
        )];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn multi_scope_unreachable_return_elimination() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope_and_id("return", vec![], "a", 0, 1),
            create_instruction_with_scope_and_id("return", vec![], "b", 2, 2),
            create_instruction_with_scope_and_id("return", vec![], "c", 1, 3),
            create_instruction_with_scope_and_id("return", vec![], "d", 2, 4),
            create_instruction_with_scope_and_id("return", vec![], "e", 2, 5),
        ];

        let expected_optimized_instructions = vec![create_instruction_with_scope_and_id(
            "return",
            vec![],
            "a",
            0,
            1,
        )];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn single_scope_unreachable_goto_elimination() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope_and_id("goto", vec!["4"], "", 0, 1),
            create_instruction_with_scope_and_id("goto", vec!["2"], "", 0, 2),
            create_instruction_with_scope_and_id("goto", vec!["3"], "", 0, 3),
        ];

        let expected_optimized_instructions = vec![create_instruction_with_scope_and_id(
            "goto",
            vec!["4"],
            "",
            0,
            1,
        )];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn multi_scope_unreachable_goto_elimination() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope_and_id("goto", vec!["2"], "", 0, 1),
            create_instruction_with_scope_and_id("goto", vec!["3"], "", 2, 2),
            create_instruction_with_scope_and_id("goto", vec!["4"], "", 1, 3),
            create_instruction_with_scope_and_id("goto", vec!["5"], "", 2, 4),
            create_instruction_with_scope_and_id("goto", vec!["6"], "", 2, 5),
            create_instruction_with_scope_and_id("goto", vec!["7"], "", 3, 6),
            create_instruction_with_scope_and_id("goto", vec!["8"], "", 1, 7),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope_and_id("goto", vec!["2"], "", 0, 1),
            create_instruction_with_scope_and_id("goto", vec!["3"], "", 2, 2),
            create_instruction_with_scope_and_id("goto", vec!["4"], "", 1, 3),
            create_instruction_with_scope_and_id("goto", vec!["5"], "", 2, 4),
            create_instruction_with_scope_and_id("goto", vec!["6"], "", 2, 5),
            create_instruction_with_scope_and_id("goto", vec!["7"], "", 3, 6),
            create_instruction_with_scope_and_id("goto", vec!["8"], "", 1, 7),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn single_scope_unreachable_jump_elimination() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope_and_id("jump", vec!["1"], "", 0, 1),
            create_instruction_with_scope_and_id("jump", vec!["2"], "", 0, 2),
            create_instruction_with_scope_and_id("jump", vec!["3"], "", 0, 3),
        ];

        let expected_optimized_instructions = vec![create_instruction_with_scope_and_id(
            "jump",
            vec!["1"],
            "",
            0,
            1,
        )];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn single_scope_jump_goto_and_return() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope_and_id("goto", vec!["3"], "", 0, 1),
            create_instruction_with_scope_and_id("jump", vec!["some_condition", "2"], "", 0, 2),
            create_instruction_with_scope_and_id("return", vec!["3"], "", 0, 3),
        ];

        let expected_optimized_instructions = vec![
            create_instruction_with_scope_and_id("goto", vec!["3"], "", 0, 1),
            create_instruction_with_scope_and_id("return", vec!["3"], "", 0, 3),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn single_scope_jump_goto_return_and_assigns() {
        let unoptimized_instructions = vec![
            create_instruction_with_scope_and_id("jump", vec!["1"], "", 0, 1),
            create_instruction_with_scope_and_id("assign", vec!["foo"], "bar", 0, 2),
            create_instruction_with_scope_and_id("goto", vec!["2"], "", 0, 3),
            create_instruction_with_scope_and_id("assign", vec!["baz"], "far", 0, 4),
            create_instruction_with_scope_and_id("jump", vec!["some_condition", "2"], "", 0, 5),
            create_instruction_with_scope_and_id("return", vec!["3"], "", 0, 6),
        ];

        let expected_optimized_instructions = vec![create_instruction_with_scope_and_id(
            "jump",
            vec!["1"],
            "",
            0,
            1,
        )];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }
}
