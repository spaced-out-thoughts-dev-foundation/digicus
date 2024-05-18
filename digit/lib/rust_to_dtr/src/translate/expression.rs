use crate::errors::not_translatable_error::NotTranslatableError;

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
        // syn::Expr::ForLoop(_) => {
        //     format!("ForLoop")
        // }
        // syn::Expr::Group(_) => {
        //     format!("Group")
        // }
        // syn::Expr::If(_) => {
        //     format!("If")
        // }
        // syn::Expr::Index(_) => {
        //     format!("Index")
        // }
        // syn::Expr::Infer(_) => {
        //     format!("Infer")
        // }
        // syn::Expr::Let(_) => {
        //     format!("Let")
        // }
        // syn::Expr::Lit(_) => {
        //     format!("Lit")
        // }
        // syn::Expr::Loop(_) => {
        //     format!("Loop")
        // }
        // syn::Expr::Macro(macro_value) => parse_macros(macro_value),
        // syn::Expr::Match(_) => {
        //     format!("Match")
        // }
        // syn::Expr::MethodCall(_) => {
        //     format!("MethodCall")
        // }
        // syn::Expr::Paren(_) => {
        //     format!("Paren")
        // }
        // syn::Expr::Path(path) => type_name::parse_path(&path.path),
        // syn::Expr::Range(_) => {
        //     format!("Range")
        // }
        // syn::Expr::Reference(_) => {
        //     format!("Reference")
        // }
        // syn::Expr::Repeat(_) => {
        //     format!("Repeat")
        // }
        // syn::Expr::Return(_) => {
        //     format!("Return")
        // }
        // syn::Expr::Struct(_) => {
        //     format!("Struct")
        // }
        // syn::Expr::Try(_) => {
        //     format!("Try")
        // }
        // syn::Expr::TryBlock(_) => {
        //     format!("TryBlock")
        // }
        // syn::Expr::Tuple(_) => {
        //     format!("Tuple")
        // }
        // syn::Expr::Unary(_) => {
        //     format!("Unary")
        // }
        syn::Expr::Unsafe(_) => Err(NotTranslatableError::Custom(
            "Unsafe expression not translatable".to_string(),
        )),
        // syn::Expr::Verbatim(_) => {
        //     format!("Verbatim")
        // }
        // syn::Expr::While(_) => {
        //     format!("While")
        // }
        // syn::Expr::Yield(_) => {
        //     format!("Yield")
        // }
        _ => Ok(format!("idk")),
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
    mod for_loop_expression {}
    mod group_expression {}
    mod if_expression {}
    mod index_expression {}
    mod infer_expression {}
    mod let_expression {}
    mod lit_expression {}
    mod loop_expression {}
    mod macro_expression {}
    mod match_expression {}
    mod method_call_expression {}
    mod paren_expression {}
    mod path_expression {}
    mod range_expression {}
    mod reference_expression {}
    mod repeat_expression {}
    mod return_expression {}
    mod struct_expression {}
    mod try_expression {}
    mod try_block_expression {}
    mod tuple_expression {}
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

    mod verbatim_expression {}
    mod while_expression {}
    mod yield_expression {}
}
