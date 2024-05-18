use crate::errors::not_translatable_error::NotTranslatableError;
use syn::ExprLoop;

pub fn handle_loop_expression(_: &ExprLoop) -> Result<String, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Loop expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprLoop;

    #[test]
    fn test_loop_expression() {
        let parsed_expr_loop: ExprLoop = syn::parse_str("loop { }").unwrap();
        let result = parse_expression(&syn::Expr::Loop(parsed_expr_loop));

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Loop expression not translatable".to_string()
            ))
        );
    }
}
