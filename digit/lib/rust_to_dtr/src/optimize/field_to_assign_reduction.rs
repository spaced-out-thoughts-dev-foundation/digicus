use crate::Instruction;
use std::collections::HashMap;

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let instructions_sans_unused_assigns = translate_fields_into_assigns(instructions);

    instructions_sans_unused_assigns
}

fn translate_fields_into_assigns(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut new_instructions = vec![];

    for instruction in instructions {
        if instruction.name == "field" {
            let assign_instruction = Instruction::new(
                "assign".to_string(),
                vec![format!(
                    "{}.{}",
                    instruction.input[0].clone(),
                    instruction.input[1].clone()
                )],
                instruction.assign.clone(),
                instruction.scope,
            );
            new_instructions.push(assign_instruction);
        } else {
            new_instructions.push(instruction);
        }
    }
    new_instructions
}
