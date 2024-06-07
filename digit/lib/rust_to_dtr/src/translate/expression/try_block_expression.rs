use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprTryBlock;

pub fn handle_try_block_expression(
    _expr: &ExprTryBlock,
    _compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    Ok(vec![Instruction::new(
        "try_block_expression".to_string(),
        vec![],
        "".to_string(),
        0,
    )])
}
