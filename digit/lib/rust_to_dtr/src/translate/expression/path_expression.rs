use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::type_name::parse_path;

pub fn handle_path_expression(path_expr: &syn::Path) -> Result<String, NotTranslatableError> {
    Ok(parse_path(path_expr))
}
