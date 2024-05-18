use crate::errors::not_translatable_error::NotTranslatableError;
use syn::ExprIf;

pub fn handle_if_expression(_: &ExprIf) -> Result<String, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "If expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprIf;

    #[test]
    fn test_if_expression() {
        let parsed_expr_if: ExprIf = syn::parse_str("if true { }").unwrap();
        let result = parse_expression(&syn::Expr::If(parsed_expr_if));

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "If expression not translatable".to_string()
            ))
        );
    }
}
