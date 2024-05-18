use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::pattern::handle_pattern;

pub fn handle_let_expression(let_expr: syn::ExprLet) -> Result<String, NotTranslatableError> {
    // let let_expr_str = parse_expression(&let_expr.expr)?;

    handle_pattern(*(let_expr.pat.clone()))
}

#[cfg(test)]
mod tests {
    use crate::translate::expression::parse_expression;
    use syn;

    mod let_expression {
        use syn::ExprLet;

        use super::*;

        #[test]
        fn test_let_expression_simple_x_equals_1() {
            let parsed_expr_let: ExprLet = syn::parse_str("let x = 1").unwrap();
            let result = parse_expression(&syn::Expr::Let(parsed_expr_let));

            assert_eq!(
                result,
                Ok("{ instruction: assign, input: (1), assign: x }".to_string())
            );
        }

        #[test]
        fn test_let_expression_less_simple_foo_equals_bar() {
            let parsed_expr_let: ExprLet = syn::parse_str("let foo = bar").unwrap();
            let result = parse_expression(&syn::Expr::Let(parsed_expr_let));

            assert_eq!(
                result,
                Ok("{ instruction: assign, input: (bar), assign: foo }".to_string())
            );
        }
    }
}
