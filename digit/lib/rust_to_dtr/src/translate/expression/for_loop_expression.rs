use crate::errors::not_translatable_error::NotTranslatableError;
use syn::ExprForLoop;

pub fn handle_for_loop_expression(_: &ExprForLoop) -> Result<String, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "ForLoop expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprForLoop;

    #[test]
    fn test_for_loop_expression() {
        let parsed_expr_for_loop: ExprForLoop = syn::parse_str("for i in 0..10 { }").unwrap();
        let result = parse_expression(&syn::Expr::ForLoop(parsed_expr_for_loop));

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "ForLoop expression not translatable".to_string()
            ))
        );
    }
}
