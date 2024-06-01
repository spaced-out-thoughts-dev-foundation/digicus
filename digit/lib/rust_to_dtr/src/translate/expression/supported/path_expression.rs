use std::vec;

use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::type_name::parse_path;

pub fn handle_path_expression(
    path_expr: &syn::Path,
    assignment: Option<String>,
    scope: u32,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let result_instruction: Vec<Instruction> = vec![Instruction::new(
        "assign".to_string(),
        vec![parse_path(path_expr)],
        assignment.unwrap_or_default(),
        scope,
    )];

    Ok(result_instruction)
}
