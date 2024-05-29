use crate::common::handle_macro;
// use super::pattern::handle_pattern;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::StmtMacro;

pub fn handle_macro_statement(
    mac: &StmtMacro,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    handle_macro(&mac.mac, assignment)
}
