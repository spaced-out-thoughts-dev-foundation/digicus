use syn::ExprAwait;

use crate::errors::not_translatable_error::NotTranslatableError;

pub fn handle_await_expression(_: &ExprAwait) -> Result<String, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Await expression not translatable".to_string(),
    ))
}
