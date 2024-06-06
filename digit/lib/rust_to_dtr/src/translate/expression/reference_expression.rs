use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, translate::expression::parse_expression,
};
use syn::ExprReference;

pub fn handle_reference_expression(
    expr_reference: &ExprReference,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    parse_expression(&expr_reference.expr, compilation_state)
}
