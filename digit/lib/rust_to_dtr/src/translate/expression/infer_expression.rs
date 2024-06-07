use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprInfer;

pub fn handle_infer_expression(
    expr: &ExprInfer,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    // TODO: seems this doesn't warrant an instruction? Is this correct?
    Ok(vec![])
}
