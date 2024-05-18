use super::parse_expression;
use crate::errors::not_translatable_error::NotTranslatableError;
use syn::ExprReference;

pub fn handle_reference_expression(
    expr_reference: &ExprReference,
) -> Result<String, NotTranslatableError> {
    parse_expression(&expr_reference.expr)
}
