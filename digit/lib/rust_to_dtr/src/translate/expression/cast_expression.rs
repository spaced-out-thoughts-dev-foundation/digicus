use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprCast;

pub fn handle_cast_expression(
    expr: &ExprCast,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    // TODO: don't skip this cast in the future
    parse_expression(&expr.expr, compilation_state)
}
