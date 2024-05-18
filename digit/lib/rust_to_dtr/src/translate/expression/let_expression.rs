use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_expression;
use crate::translate::expression::parse_lit;

pub fn handle_let_expression(let_expr: syn::ExprLet) -> Result<String, NotTranslatableError> {
    let let_expr_str = parse_expression(&let_expr.expr)?;

    let let_expr_pat: syn::Pat = *(let_expr.pat.clone());

    match &let_expr_pat {
        syn::Pat::Lit(lit_pat) => {
            let const_pat_str = format!("{:?}", parse_lit(&lit_pat.lit));
            Ok(format!(
                "{{ instruction: assign, input: ({}), assign: {} }}",
                let_expr_str, const_pat_str
            ))
        }
        syn::Pat::Ident(ident_pat) => Ok(format!(
            "{{ instruction: assign, input: ({}), assign: {} }}",
            let_expr_str,
            ident_pat.ident.to_string()
        )),
        _ => Err(NotTranslatableError::Custom(
            "Unknown pattern in let expression".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
