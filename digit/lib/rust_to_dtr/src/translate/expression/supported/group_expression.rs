use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprGroup;

pub fn handle_group_expression(
    _expr: &ExprGroup,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    Ok(vec![Instruction::new(
        "group".to_string(),
        vec!["DO_A_GROUP".to_string()],
        "DID_A_GROUP".to_string(),
        compilation_state.scope,
    )])
}
