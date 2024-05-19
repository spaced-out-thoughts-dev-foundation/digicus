use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprAwait;

pub fn handle_await_expression(_: &ExprAwait) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Await expression not translatable".to_string(),
    ))
}
