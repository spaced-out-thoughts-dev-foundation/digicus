use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_lit;

pub fn handle_lit_expression(lit: &syn::Lit) -> Result<String, NotTranslatableError> {
    parse_lit(lit)
}

#[cfg(test)]
mod tests {
    use syn;

    mod lit_expression {
        use super::*;
        use syn::{Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt};

        use crate::translate::expression::parse_expression;

        #[test]
        fn test_lit_expression_bool() {
            let parsed_lit_bool: LitBool = syn::parse_str("true").unwrap();
            let result = parse_expression(&syn::Expr::Lit(syn::ExprLit {
                attrs: Vec::new(),
                lit: Lit::Bool(parsed_lit_bool),
            }));

            assert_eq!(result, Ok("true".to_string()));
        }

        #[test]
        fn test_lit_expression_byte() {
            let parsed_lit_byte: LitByte = syn::parse_str("b'1'").unwrap();
            let result = parse_expression(&syn::Expr::Lit(syn::ExprLit {
                attrs: Vec::new(),
                lit: Lit::Byte(parsed_lit_byte),
            }));

            assert_eq!(result, Ok("49".to_string()));
        }

        #[test]
        fn test_lit_expression_byte_str() {
            let parsed_lit_byte_str: LitByteStr = syn::parse_str("b\"hello\"").unwrap();
            let result = parse_expression(&syn::Expr::Lit(syn::ExprLit {
                attrs: Vec::new(),
                lit: Lit::ByteStr(parsed_lit_byte_str),
            }));

            assert_eq!(result, Ok("\"hello\"".to_string()));
        }

        #[test]
        fn test_lit_expression_char() {
            let parsed_lit_char: LitChar = syn::parse_str("'a'").unwrap();
            let result = parse_expression(&syn::Expr::Lit(syn::ExprLit {
                attrs: Vec::new(),
                lit: Lit::Char(parsed_lit_char),
            }));

            assert_eq!(result, Ok("'a'".to_string()));
        }

        #[test]
        fn test_lit_expression_float() {
            let parsed_lit_float: LitFloat = syn::parse_str("3.14").unwrap();
            let result = parse_expression(&syn::Expr::Lit(syn::ExprLit {
                attrs: Vec::new(),
                lit: Lit::Float(parsed_lit_float),
            }));

            assert_eq!(result, Ok("3.14".to_string()));
        }

        #[test]
        fn test_lit_expression_int() {
            let parsed_lit_int: LitInt = syn::parse_str("42").unwrap();
            let result = parse_expression(&syn::Expr::Lit(syn::ExprLit {
                attrs: Vec::new(),
                lit: Lit::Int(parsed_lit_int),
            }));

            assert_eq!(result, Ok("42".to_string()));
        }
    }
}
