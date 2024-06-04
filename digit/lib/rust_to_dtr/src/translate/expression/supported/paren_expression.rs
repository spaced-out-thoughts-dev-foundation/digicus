use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use syn::ExprParen;

pub fn handle_paren_expression(
    expr_paren: &ExprParen,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    Ok(parse_expression(&expr_paren.expr, compilation_state)?)
}
