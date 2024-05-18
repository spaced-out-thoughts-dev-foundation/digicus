use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::type_name;

fn parse_expression(exp: &syn::Expr) -> Result<String, NotTranslatableError> {
    match exp {
        syn::Expr::Array(_) => Err(NotTranslatableError::Custom(
            "Array expression not translatable".to_string(),
        )),
        // syn::Expr::Assign(_) => {
        //     format!("Assign")
        // }
        syn::Expr::Async(_) => Err(NotTranslatableError::Custom(
            "Async expression not translatable".to_string(),
        )),
        syn::Expr::Await(_) => Err(NotTranslatableError::Custom(
            "Await expression not translatable".to_string(),
        )),
        // syn::Expr::Binary(_) => {
        //     format!("Binary")
        // }
        // syn::Expr::Block(_) => {
        //     format!("Block")
        // }
        // syn::Expr::Break(_) => {
        //     format!("Break")
        // }
        // syn::Expr::Call(_) => {
        //     format!("Call")
        // }
        // syn::Expr::Cast(_) => {
        //     format!("Cast")
        // }
        // syn::Expr::Closure(_) => {
        //     format!("Closure")
        // }
        // syn::Expr::Const(_) => {
        //     format!("Const")
        // }
        // syn::Expr::Continue(_) => {
        //     format!("Continue")
        // }
        // syn::Expr::Field(_) => {
        //     format!("Field")
        // }
        syn::Expr::ForLoop(_) => Err(NotTranslatableError::Custom(
            "ForLoop expression not translatable".to_string(),
        )),
        // syn::Expr::Group(_) => {
        //     format!("Group")
        // }
        syn::Expr::If(_) => Err(NotTranslatableError::Custom(
            "If expression not translatable".to_string(),
        )),
        // syn::Expr::Index(_) => {
        //     format!("Index")
        // }
        // syn::Expr::Infer(_) => {
        //     format!("Infer")
        // }
        syn::Expr::Let(let_expr) => {
            let let_expr_str = parse_expression(&let_expr.expr)?;

            let let_expr_pat: syn::Pat = *(let_expr.pat.clone());

            match &let_expr_pat {
                syn::Pat::Lit(lit_pat) => {
                    let const_pat_str = format!("{:?}", parse_lit(&lit_pat.lit));
                    Ok(format!("let {} = {}", const_pat_str, let_expr_str))
                }
                syn::Pat::Ident(ident_pat) => Ok(format!(
                    "let {} = {}",
                    ident_pat.ident.to_string(),
                    let_expr_str
                )),
                _ => Err(NotTranslatableError::Custom(
                    "Unknown pattern in let expression".to_string(),
                )),
            }
        }
        syn::Expr::Lit(lit_expr) => parse_lit(&lit_expr.lit),
        syn::Expr::Loop(_) => Err(NotTranslatableError::Custom(
            "Loop expression not translatable".to_string(),
        )),
        // syn::Expr::Macro(macro_value) => {
        //     let macro_str = format!("{:?}", macro_value.mac.tokens);
        //     Ok(macro_str)
        // }
        syn::Expr::Match(_) => Err(NotTranslatableError::Custom(
            "Match expression not translatable".to_string(),
        )),
        // syn::Expr::MethodCall(_) => {
        //     format!("MethodCall")
        // }
        // syn::Expr::Paren(_) => {
        //     format!("Paren")
        // }
        // syn::Expr::Path(path) => parse_path(&path.path),
        // syn::Expr::Range(_) => {
        //     format!("Range")
        // }
        // syn::Expr::Reference(_) => {
        //     format!("Reference")
        // }
        syn::Expr::Repeat(_) => Err(NotTranslatableError::Custom(
            "Repeat expression not translatable".to_string(),
        )),
        syn::Expr::Return(return_expr_expr) => {
            let return_expr_box = &return_expr_expr.expr;

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
        // syn::Expr::Struct(_) => {
        //     format!("Struct")
        // }
        syn::Expr::Try(_) => Err(NotTranslatableError::Custom(
            "Try expression not translatable".to_string(),
        )),
        syn::Expr::TryBlock(_) => Err(NotTranslatableError::Custom(
            "TryBlock expression not translatable".to_string(),
        )),
        syn::Expr::Tuple(_) => Err(NotTranslatableError::Custom(
            "Tuple expression not translatable".to_string(),
        )),
        // syn::Expr::Unary(_) => {
        //     format!("Unary")
        // }
        syn::Expr::Unsafe(_) => Err(NotTranslatableError::Custom(
            "Unsafe expression not translatable".to_string(),
        )),
        syn::Expr::While(_) => Err(NotTranslatableError::Custom(
            "While expression not translatable".to_string(),
        )),
        syn::Expr::Yield(_) => Err(NotTranslatableError::Custom(
            "Yield expression not translatable".to_string(),
        )),
        _ => Err(NotTranslatableError::Custom(
            "Unknown expression".to_string(),
        )),
    }
}

// fn parse_macros(mac: &syn::ExprMacro) -> String {
//     let macro_itself: &syn::Macro = &mac.mac;

//     let mut macro_str = String::new();

//     // TODO: do all macros have a bang?
//     macro_str.push_str(&format!("{}!", type_name::parse_path(&macro_itself.path)));

//     macro_str.push_str(format!("{:?}", macro_itself.tokens).as_str());

//     macro_str
// }

// pub fn parse_block_stmt(stmt: &syn::Stmt) -> String {
//     match stmt {
//         syn::Stmt::Local(_local) => {
//             format!("Local")
//         }
//         syn::Stmt::Item(_item) => {
//             format!("Item")
//         }
//         syn::Stmt::Expr(exp, _r) => {
//             format!("{}", parse_expression(exp))
//         }
//         syn::Stmt::Macro(_mac) => {
//             format!("Macro")
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use syn;

    mod array_expression {
        use syn::ExprArray;

        use super::*;

        #[test]
        fn test_array_expression() {
            let parsed_expr_array: ExprArray = syn::parse_str("[1,2,3]").unwrap();
            let result = parse_expression(&syn::Expr::Array(parsed_expr_array));

            assert_eq!(
                result,
                Err(NotTranslatableError::Custom(
                    "Array expression not translatable".to_string()
                ))
            );
        }
    }

    mod asisgn_expression {}

    mod async_expression {
        use syn::ExprAsync;

        use super::*;

        #[test]
        fn test_async_expression() {
            let parsed_expr_async: ExprAsync = syn::parse_str("async { }").unwrap();
            let result = parse_expression(&syn::Expr::Async(parsed_expr_async));

            assert_eq!(
                result,
                Err(NotTranslatableError::Custom(
                    "Async expression not translatable".to_string()
                ))
            );
        }
    }

    mod await_expression {}

    mod binary_expression {}
    mod block_expression {}
    mod break_expression {}
    mod call_expression {}
    mod cast_expression {}
    mod closure_expression {}
    mod const_expression {}
    mod continue_expression {}
    mod field_expression {}

    mod for_loop_expression {
        use syn::ExprForLoop;

        use super::*;

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

    mod group_expression {}

    mod if_expression {
        use syn::ExprIf;

        use super::*;

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

    mod index_expression {}
    mod infer_expression {}

    mod let_expression {
        use syn::ExprLet;

        use super::*;

        #[test]
        fn test_let_expression() {
            let parsed_expr_let: ExprLet = syn::parse_str("let x = 1").unwrap();
            let result = parse_expression(&syn::Expr::Let(parsed_expr_let));

            assert_eq!(result, Ok("let x = 1".to_string()));
        }
    }

    mod lit_expression {
        use syn::{Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr};

        use super::*;

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

    mod loop_expression {
        use syn::ExprLoop;

        use super::*;

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
    mod macro_expression {}
    mod match_expression {
        use syn::ExprMatch;

        use super::*;

        #[test]
        fn test_match_expression() {
            let parsed_expr_match: ExprMatch = syn::parse_str("match 1 { _ => 0 }").unwrap();
            let result = parse_expression(&syn::Expr::Match(parsed_expr_match));

            assert_eq!(
                result,
                Err(NotTranslatableError::Custom(
                    "Match expression not translatable".to_string()
                ))
            );
        }
    }
    mod method_call_expression {}
    mod paren_expression {}
    mod path_expression {}
    mod range_expression {}
    mod reference_expression {}
    mod repeat_expression {
        use syn::ExprRepeat;

        use super::*;

        #[test]
        fn test_repeat_expression() {
            let parsed_expr_repeat: ExprRepeat = syn::parse_str("[1; 3]").unwrap();
            let result = parse_expression(&syn::Expr::Repeat(parsed_expr_repeat));

            assert_eq!(
                result,
                Err(NotTranslatableError::Custom(
                    "Repeat expression not translatable".to_string()
                ))
            );
        }
    }

    mod return_expression {
        use syn::ExprReturn;

        use super::*;

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

    mod struct_expression {}

    mod try_expression {
        use syn::ExprTry;

        use super::*;

        #[test]
        fn test_try_expression() {
            let parsed_expr_try: ExprTry = syn::parse_str("expr?").unwrap();
            let result = parse_expression(&syn::Expr::Try(parsed_expr_try));

            assert_eq!(
                result,
                Err(NotTranslatableError::Custom(
                    "Try expression not translatable".to_string()
                ))
            );
        }
    }

    mod try_block_expression {
        use syn::ExprTryBlock;

        use super::*;

        #[test]
        fn test_try_block_expression() {
            let parsed_expr_try_block: ExprTryBlock = syn::parse_str("try { }").unwrap();
            let result = parse_expression(&syn::Expr::TryBlock(parsed_expr_try_block));

            assert_eq!(
                result,
                Err(NotTranslatableError::Custom(
                    "TryBlock expression not translatable".to_string()
                ))
            );
        }
    }

    mod tuple_expression {
        use syn::ExprTuple;

        use super::*;

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

    mod unary_expression {}

    mod unsafe_expression {
        use syn::ExprUnsafe;

        use super::*;

        #[test]
        fn test_unsafe_expression() {
            let parsed_expr_unsafe: ExprUnsafe = syn::parse_str("unsafe { }").unwrap();
            let result = parse_expression(&syn::Expr::Unsafe(parsed_expr_unsafe));

            assert_eq!(
                result,
                Err(NotTranslatableError::Custom(
                    "Unsafe expression not translatable".to_string()
                ))
            );
        }
    }

    mod while_expression {
        use syn::ExprWhile;

        use super::*;

        #[test]
        fn test_while_expression() {
            let parsed_expr_while: ExprWhile = syn::parse_str("while true { }").unwrap();
            let result = parse_expression(&syn::Expr::While(parsed_expr_while));

            assert_eq!(
                result,
                Err(NotTranslatableError::Custom(
                    "While expression not translatable".to_string()
                ))
            );
        }
    }

    mod yield_expression {
        use syn::ExprYield;

        use super::*;

        #[test]
        fn test_yield_expression() {
            let parsed_expr_yield: ExprYield = syn::parse_str("yield 1").unwrap();
            let result = parse_expression(&syn::Expr::Yield(parsed_expr_yield));

            assert_eq!(
                result,
                Err(NotTranslatableError::Custom(
                    "Yield expression not translatable".to_string()
                ))
            );
        }
    }
}

fn byte_vec_to_string(byte_vec: Vec<u8>) -> String {
    // Convert each byte into a character and collect into a string
    let characters: String = byte_vec.into_iter().map(|byte| byte as char).collect();
    characters
}

fn parse_lit(syn_lit: &syn::Lit) -> Result<String, NotTranslatableError> {
    match &syn_lit {
        syn::Lit::Bool(bool_lit) => Ok(bool_lit.value.to_string()),
        syn::Lit::Byte(byte_lit) => Ok(byte_lit.value().to_string()),
        syn::Lit::ByteStr(byte_str_lit) => {
            Ok(format!("{:?}", byte_vec_to_string(byte_str_lit.value())))
        }
        syn::Lit::Char(char_lit) => Ok(format!("{:?}", char_lit.value())),
        syn::Lit::Float(float_lit) => Ok(float_lit.base10_digits().to_string()),
        syn::Lit::Int(int_lit) => Ok(int_lit.base10_digits().to_string()),
        syn::Lit::Str(str_lit) => Ok(format!("{:?}", str_lit.value())),
        syn::Lit::Verbatim(verbatim_lit) => Err(NotTranslatableError::Custom(format!(
            "Verbatim literal expression: {}",
            verbatim_lit.to_string()
        ))),
        _ => Err(NotTranslatableError::Custom(
            "Unknown literal expression".to_string(),
        )),
    }
}
