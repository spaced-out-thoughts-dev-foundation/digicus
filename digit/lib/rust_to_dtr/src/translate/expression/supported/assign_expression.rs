use crate::instruction::Instruction;
use crate::translate::expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprAssign;

pub fn handle_assign_expression(
    expr: &ExprAssign,
    assignment: Option<String>,
    scope: u32,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let left_hand_side_name = "ASSIGN_EXPRESSION_LEFT";
    let right_hand_side_name = "ASSIGN_EXPRESSION_RIGHT";

    let mut left_hand_side: Vec<Instruction> =
        expression::parse_expression(&expr.left, Some(left_hand_side_name.to_string()), scope)?;
    let right_hand_side: Vec<Instruction> =
        expression::parse_expression(&expr.right, Some(right_hand_side_name.to_string()), scope)?;

    let binary_instruction = Instruction::new(
        "assign".to_string(),
        vec![
            left_hand_side_name.to_string(),
            right_hand_side_name.to_string(),
        ],
        // TODO: this is incorrect!
        assignment.unwrap_or_default(),
        scope,
    );

    // add all instructions to one vec
    left_hand_side.extend(right_hand_side);
    left_hand_side.push(binary_instruction);

    Ok(left_hand_side)
}
