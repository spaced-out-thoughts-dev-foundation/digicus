use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprAssign;

pub fn handle_assign_expression(
    expr: &ExprAssign,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let left_hand_side_name = format!(
        "ASSIGN_EXPRESSION_LEFT_{}",
        compilation_state.get_global_uuid()
    );
    let right_hand_side_name = format!(
        "ASSIGN_EXPRESSION_RIGHT_{}",
        compilation_state.get_global_uuid()
    );

    let mut left_hand_side: Vec<Instruction> = expression::parse_expression(
        &expr.left,
        &mut compilation_state.with_assignment(Some(left_hand_side_name.to_string())),
    )?;

    let right_hand_side: Vec<Instruction> = expression::parse_expression(
        &expr.right,
        &mut compilation_state.with_assignment(Some(right_hand_side_name.to_string())),
    )?;

    let binary_instruction = Instruction::new(
        compilation_state.get_global_uuid(),
        "assign".to_string(),
        vec![right_hand_side_name.to_string()],
        // TODO: this is incorrect!
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or(left_hand_side_name.to_string()),
        compilation_state.scope(),
    );

    // add all instructions to one vec
    left_hand_side.extend(right_hand_side);
    left_hand_side.push(binary_instruction);

    Ok(left_hand_side)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_handle_assign_expression() {
        let expr: ExprAssign = syn::parse_str("a = 1").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_assign_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    2,
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "ASSIGN_EXPRESSION_LEFT_0".to_string(),
                    0,
                ),
                Instruction::new(
                    3,
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "ASSIGN_EXPRESSION_RIGHT_1".to_string(),
                    0,
                ),
                Instruction::new(
                    4,
                    "assign".to_string(),
                    vec!["ASSIGN_EXPRESSION_RIGHT_1".to_string()],
                    "ASSIGN_EXPRESSION_LEFT_0".to_string(),
                    0,
                ),
            ]
        );
    }
}
