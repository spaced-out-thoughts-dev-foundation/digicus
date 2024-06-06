use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprBreak;

pub fn handle_break_expression(
    _expr: &ExprBreak,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    Ok(vec![Instruction::from_compilation_state(
        "break".to_string(),
        vec![],
        compilation_state,
    )])
}
