use crate::Instruction;
use std::collections::HashMap;

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let instructions_sans_unused_assigns = remove_unused_assigns(instructions);

    instructions_sans_unused_assigns
}

#[derive(Debug)]
struct AssignHashValue {
    index: usize,
    is_used: bool,
}

pub fn remove_unused_assigns(instructions: Vec<Instruction>) -> Vec<Instruction> {
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

            if instruction.name == "assign" {
                let assigned_value = instruction.assign.as_str();

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

                assign_hash.insert(
                    assigned_value.to_string(),
                    AssignHashValue {
                        index: index,
                        is_used: false,
                    },
                );

                // }
            } else {
                tagged_instructions
                    .entry(index)
                    .and_modify(|e| *e = should_keep);

                instruction.input.clone().into_iter().for_each(|input| {
                    if assign_hash.contains_key(&input) {
                        assign_hash.entry(input).and_modify(|e| {
                            e.is_used = true;
                        });
                    }
                });
            }
        });

    assign_hash.into_iter().for_each(|(_key, value)| {
        tagged_instructions.entry(value.index).and_modify(|e| {
            *e = if value.is_used {
                should_keep
            } else {
                get_rid_off
            }
        });
    });

    instructions
        .into_iter()
        .enumerate()
        .filter(|(index, _)| tagged_instructions.get(index).unwrap() == &should_keep)
        .map(|(_, instruction)| instruction)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimize::create_instruction;

    #[test]
    fn remove_unused_assigns_removes_unused_assigns_simple() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["2"], "b"),
            create_instruction("add", vec!["5", "3"], "c"),
            create_instruction("add", vec!["c", "a"], "d"),
            create_instruction("add", vec!["c", "10"], "e"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["5", "3"], "c"),
            create_instruction("add", vec!["c", "a"], "d"),
            create_instruction("add", vec!["c", "10"], "e"),
        ];

        assert_eq!(
            remove_unused_assigns(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn remove_unused_assigns_removes_unused_assigns_complex() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["2"], "b"),
            create_instruction("add", vec!["5", "3"], "c"),
            create_instruction("assign", vec!["11"], "a"),
            create_instruction("add", vec!["c", "a"], "d"),
            create_instruction("assign", vec!["10"], "a"),
            create_instruction("add", vec!["c", "10"], "e"),
            create_instruction("assign", vec!["d"], "f"),
            create_instruction("add", vec!["f", "e"], "g"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("add", vec!["5", "3"], "c"),
            create_instruction("assign", vec!["11"], "a"),
            create_instruction("add", vec!["c", "a"], "d"),
            create_instruction("add", vec!["c", "10"], "e"),
            create_instruction("assign", vec!["d"], "f"),
            create_instruction("add", vec!["f", "e"], "g"),
        ];

        assert_eq!(
            remove_unused_assigns(unoptimized_instructions),
            expected_optimized_instructions
        );
    }
}
