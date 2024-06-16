use crate::Instruction;

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    instructions
        .into_iter()
        .map(|instruction| {
            if instruction.name.ends_with("_and_assign") {
                transform_and_assign_to_normal_op_with_assign(&instruction)
            } else {
                instruction
            }
        })
        .collect()
}

pub fn transform_and_assign_to_normal_op_with_assign(instruction: &Instruction) -> Instruction {
    let operator = instruction.name.clone();
    let left_hand_side = instruction.input[0].clone();
    let right_hand_side = instruction.input[1].clone();

    Instruction::new(
        operator.replace("_and_assign", ""),
        vec![left_hand_side.clone(), right_hand_side],
        left_hand_side,
        instruction.scope,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimize::create_instruction;

    #[test]
    fn transform_and_assign_to_normal_op_with_assign_transforms_add_and_assign() {
        let instruction = create_instruction("add_and_assign", vec!["a", "b"], "a");

        let result = transform_and_assign_to_normal_op_with_assign(&instruction);

        assert_eq!(result.name, "add");
        assert_eq!(result.input, vec!["a", "b"]);
        assert_eq!(result.assign, "a");
    }

    #[test]
    fn transform_and_assign_to_normal_op_with_assign_does_not_change_normal_op() {
        let instruction = create_instruction("add", vec!["a", "b"], "a");

        let result = transform_and_assign_to_normal_op_with_assign(&instruction);

        assert_eq!(result.name, "add");
        assert_eq!(result.input, vec!["a", "b"]);
        assert_eq!(result.assign, "a");
    }

    #[test]
    fn transform_and_assign_to_normal_op_with_assign_transforms_subtract_and_assign() {
        let instruction = create_instruction("subtract_and_assign", vec!["a", "b"], "a");

        let result = transform_and_assign_to_normal_op_with_assign(&instruction);

        assert_eq!(result.name, "subtract");
        assert_eq!(result.input, vec!["a", "b"]);
        assert_eq!(result.assign, "a");
    }
}
