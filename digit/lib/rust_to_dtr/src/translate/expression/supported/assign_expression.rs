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
    let left_hand_side_name = "ASSIGN_EXPRESSION_LEFT";
    let right_hand_side_name = "ASSIGN_EXPRESSION_RIGHT";

    let mut left_hand_side: Vec<Instruction> = expression::parse_expression(
        &expr.left,
        &mut compilation_state.with_assignment(Some(left_hand_side_name.to_string())),
    )?;

    let right_hand_side: Vec<Instruction> = expression::parse_expression(
        &expr.right,
        &mut compilation_state.with_assignment(Some(right_hand_side_name.to_string())),
    )?;

    let binary_instruction = Instruction::new(
        "assign".to_string(),
        vec![
            left_hand_side_name.to_string(),
            right_hand_side_name.to_string(),
        ],
        // TODO: this is incorrect!
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or_default(),
        compilation_state.scope,
    );

    // add all instructions to one vec
    left_hand_side.extend(right_hand_side);
    left_hand_side.push(binary_instruction);

    Ok(left_hand_side)
}
