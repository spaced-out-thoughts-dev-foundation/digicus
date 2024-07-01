use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprTry;

use super::parse_expression;

pub fn handle_try_expression(
    expr: &ExprTry,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    parse_expression(&expr.expr, compilation_state)
}
