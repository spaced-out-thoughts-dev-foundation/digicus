use std::vec;

use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::type_name::parse_path;

pub fn handle_path_expression(
    path_expr: &syn::Path,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let result_instruction: Vec<Instruction> = vec![Instruction::new(
        "assign".to_string(),
        vec![parse_path(path_expr)],
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or_default(),
        compilation_state.scope,
    )];

    Ok(result_instruction)
}
