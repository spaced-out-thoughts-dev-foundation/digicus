use crate::errors::not_translatable_error::NotTranslatableError;
use syn::ExprTuple;

pub fn handle_tuple_expression(_: &ExprTuple) -> Result<String, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Tuple expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprTuple;

    #[test]
    fn test_tuple_expression() {
        let parsed_expr_tuple: ExprTuple = syn::parse_str("(1, 2, 3)").unwrap();
        let result = parse_expression(&syn::Expr::Tuple(parsed_expr_tuple));

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Tuple expression not translatable".to_string()
            ))
        );
    }
}
