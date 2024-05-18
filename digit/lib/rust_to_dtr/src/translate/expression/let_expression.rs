use std::path;

use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_expression;
use crate::translate::expression::parse_lit;
use crate::translate::type_name::figure_out_type;
use crate::translate::type_name::parse_path;

pub fn handle_let_expression(let_expr: syn::ExprLet) -> Result<String, NotTranslatableError> {
    let let_expr_str = parse_expression(&let_expr.expr)?;

    let let_expr_pat: syn::Pat = *(let_expr.pat.clone());

    match &let_expr_pat {
        syn::Pat::Ident(ident_pat) => Ok(format!(
            "{{ instruction: assign, input: ({}), assign: {} }}",
            let_expr_str,
            ident_pat.ident.to_string()
        )),
        syn::Pat::Const(_) => Ok(format!("Const")),
        syn::Pat::Lit(lit_pat) => {
            let const_pat_str = format!("{:?}", parse_lit(&lit_pat.lit));
            Ok(format!(
                "{{ instruction: assign, input: ({}), assign: {} }}",
                let_expr_str, const_pat_str
            ))
        }
        syn::Pat::Macro(_) => Ok(format!("Macro")),
        syn::Pat::Or(_) => Ok(format!("Or")),
        syn::Pat::Paren(_) => Ok(format!("Paren")),
        syn::Pat::Path(path_pat) => Ok(parse_path(&path_pat.path)),
        syn::Pat::Range(_) => Ok(format!("Range")),
        syn::Pat::Reference(_) => Ok(format!("Reference")),
        syn::Pat::Rest(_) => Ok(format!("Rest")),
        syn::Pat::Slice(_) => Ok(format!("Slice")),
        syn::Pat::Struct(_) => Ok(format!("Struct")),
        syn::Pat::Tuple(_) => Ok(format!("Tuple")),
        syn::Pat::TupleStruct(_) => Ok(format!("TupleStruct")),
        syn::Pat::Type(type_pat) => figure_out_type(&type_pat.ty),
        syn::Pat::Verbatim(_) => Ok(format!("Verbatim")),
        syn::Pat::Wild(_) => Ok(format!("Wild")),
        _ => Err(NotTranslatableError::Custom(
            "Unknown pattern in let pat".to_string(),
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
