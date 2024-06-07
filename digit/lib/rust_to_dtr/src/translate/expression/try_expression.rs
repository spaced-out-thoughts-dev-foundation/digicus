use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprTry;

pub fn handle_try_expression(
    expr: &ExprTry,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    // TODO: probably should actually handle this in some way.
    parse_expression(&expr.expr, compilation_state)
}
