use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, translate::expression::parse_expression,
};
use syn::ExprReference;

pub fn handle_reference_expression(
    expr_reference: &ExprReference,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    parse_expression(&expr_reference.expr, assignment)
}
