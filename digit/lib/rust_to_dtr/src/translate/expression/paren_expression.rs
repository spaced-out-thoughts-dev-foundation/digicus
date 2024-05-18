use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_expression;
use syn::ExprParen;

pub fn handle_paren_expression(expr_paren: &ExprParen) -> Result<String, NotTranslatableError> {
    let paren_expr_str = parse_expression(&expr_paren.expr)?;
    Ok(format!("({})", paren_expr_str))
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::ExprParen;

    #[test]
    fn test_paren_expression() {
        let parsed_expr_paren: ExprParen = syn::parse_str("(1)").unwrap();
        let result = parse_expression(&syn::Expr::Paren(parsed_expr_paren));

        assert_eq!(result, Ok("(1)".to_string()));
    }
}
