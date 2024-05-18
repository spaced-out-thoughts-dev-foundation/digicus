use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_expression;
use syn::ExprReturn;

pub fn handle_return_expression(expr_return: &ExprReturn) -> Result<String, NotTranslatableError> {
    let return_expr_box = &expr_return.expr;

    match return_expr_box {
        Some(return_expr) => {
            let return_expr_str = parse_expression(return_expr)?;
            Ok(format!("return {}", return_expr_str))
        }
        None => Err(NotTranslatableError::Custom(
            "Return expression not translatable".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::ExprReturn;

    #[test]
    fn test_return_expression_int() {
        let parsed_expr_return: ExprReturn = syn::parse_str("return 1").unwrap();
        let result = parse_expression(&syn::Expr::Return(parsed_expr_return));

        assert_eq!(result, Ok("return 1".to_string()));
    }

    #[test]
    fn test_return_expression_bool() {
        let parsed_expr_return: ExprReturn = syn::parse_str("return true").unwrap();
        let result = parse_expression(&syn::Expr::Return(parsed_expr_return));

        assert_eq!(result, Ok("return true".to_string()));
    }

    #[test]
    fn test_return_expression_no_expr() {
        let parsed_expr_return: ExprReturn = syn::parse_str("return").unwrap();
        let result = parse_expression(&syn::Expr::Return(parsed_expr_return));

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Return expression not translatable".to_string()
            ))
        );
    }
}
