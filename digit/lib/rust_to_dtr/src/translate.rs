extern crate syn;

pub fn parse_return_type(ty: &syn::Type) -> String {
    format!("\t* Output: {}\n", figure_out_type(ty))
}

pub fn figure_out_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => {
            format!("{}", parse_path(&type_path.path))
        }
        _ => format!("idk"),
    }
}

fn parse_path(path: &syn::Path) -> String {
    let segments = &path.segments;
    let segment = &segments[0];

    match &segment.arguments {
        syn::PathArguments::None => {
            return format!("{}", segment.ident);
        }
        syn::PathArguments::AngleBracketed(args) => {
            let mut path_str = String::new();
            path_str.push_str(&format!("{}", segment.ident));
            path_str.push_str("<");
            // for arg in args.args {
                // path_str.push_str(&format!("{}", parse_path_arguments(arg)));
            // }
            path_str.push_str(">");
            return path_str;
        }
        _ => {}
    }
    if segments.len() > 1 {
        let mut path_str = String::new();
        for segment in segments {
            path_str.push_str(&format!("{}::", segment.ident));
        }
        return path_str;
    }
    

    format!("{}", segment.ident)
}

pub fn parse_block_stmt(stmt: &syn::Stmt) -> String {
    match stmt {
        syn::Stmt::Local(_local) => {
            format!("Local")
        }
        syn::Stmt::Item(_item) => {
            format!("Item")
        }
        syn::Stmt::Expr(exp, _r) => {
            format!("{}", parse_expression(exp))
        }
        syn::Stmt::Macro(_mac) => {
            format!("Macro")
        }
    }
}

fn parse_expression(exp: &syn::Expr) -> String {
    match exp {
        syn::Expr::Array(_) => {
            format!("Array")
        }
        syn::Expr::Assign(_) => {
            format!("Assign")
        }
        syn::Expr::Async(_) => {
            format!("Async")
        }
        syn::Expr::Await(_) => {
            format!("Await")
        }
        syn::Expr::Binary(_) => {
            format!("Binary")
        }
        syn::Expr::Block(_) => {
            format!("Block")
        }
        syn::Expr::Break(_) => {
            format!("Break")
        }
        syn::Expr::Call(_) => {
            format!("Call")
        }
        syn::Expr::Cast(_) => {
            format!("Cast")
        }
        syn::Expr::Closure(_) => {
            format!("Closure")
        }
        syn::Expr::Const(_) => {
            format!("Const")
        }
        syn::Expr::Continue(_) => {
            format!("Continue")
        }
        syn::Expr::Field(_) => {
            format!("Field")
        }
        syn::Expr::ForLoop(_) => {
            format!("ForLoop")
        }
        syn::Expr::Group(_) => {
            format!("Group")
        }
        syn::Expr::If(_) => {
            format!("If")
        }
        syn::Expr::Index(_) => {
            format!("Index")
        }
        syn::Expr::Infer(_) => {
            format!("Infer")
        }
        syn::Expr::Let(_) => {
            format!("Let")
        }
        syn::Expr::Lit(_) => {
            format!("Lit")
        }
        syn::Expr::Loop(_) => {
            format!("Loop")
        }
        syn::Expr::Macro(macro_value) => parse_macros(macro_value),
        syn::Expr::Match(_) => {
            format!("Match")
        }
        syn::Expr::MethodCall(_) => {
            format!("MethodCall")
        }
        syn::Expr::Paren(_) => {
            format!("Paren")
        }
        syn::Expr::Path(_) => {
            format!("Path")
        }
        syn::Expr::Range(_) => {
            format!("Range")
        }
        syn::Expr::Reference(_) => {
            format!("Reference")
        }
        syn::Expr::Repeat(_) => {
            format!("Repeat")
        }
        syn::Expr::Return(_) => {
            format!("Return")
        }
        syn::Expr::Struct(_) => {
            format!("Struct")
        }
        syn::Expr::Try(_) => {
            format!("Try")
        }
        syn::Expr::TryBlock(_) => {
            format!("TryBlock")
        }
        syn::Expr::Tuple(_) => {
            format!("Tuple")
        }
        syn::Expr::Unary(_) => {
            format!("Unary")
        }
        syn::Expr::Unsafe(_) => {
            format!("Unsafe")
        }
        syn::Expr::Verbatim(_) => {
            format!("Verbatim")
        }
        syn::Expr::While(_) => {
            format!("While")
        }
        syn::Expr::Yield(_) => {
            format!("Yield")
        }
        _ => {
            format!("idk")
        }
    }
}

fn parse_macros(_mac: &syn::ExprMacro) -> String {
    format!("Macro")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_figure_out_type_primitive() {
        let ty = syn::parse_str("i32").unwrap();
        assert_eq!(super::figure_out_type(&ty), "i32");
    }

    #[test]
    fn test_figure_out_type_vec() {
        let ty = syn::parse_str("Vec<i32>").unwrap();
        assert_eq!(super::figure_out_type(&ty), "Vec<i32>");
    }

    #[test]
    fn test_figure_out_type_enum() {
        let ty = syn::parse_str("Option<i32>").unwrap();
        assert_eq!(super::figure_out_type(&ty), "Option<i32>");
    }

    #[test]
    fn test_figure_out_type_hash_map() {
        let ty = syn::parse_str("HashMap<i32, i32>").unwrap();
        assert_eq!(super::figure_out_type(&ty), "HashMap<i32, i32>");
    }

    #[test]
    fn test_figure_out_type_pointer() {
        let ty = syn::parse_str("*const i32").unwrap();
        assert_eq!(super::figure_out_type(&ty), "*const i32");
    }

    #[test]
    fn test_figure_out_type_function() {
        let ty = syn::parse_str("fn(i32) -> i32").unwrap();
        assert_eq!(super::figure_out_type(&ty), "fn(i32) -> i32");
    }

    #[test]
    fn test_figure_out_type_trait() {
        let ty = syn::parse_str("dyn Fn(i32) -> i32").unwrap();
        assert_eq!(super::figure_out_type(&ty), "dyn Fn(i32) -> i32");
    }

    #[test]
    fn test_figure_out_type_never() {
        let ty = syn::parse_str("!").unwrap();
        assert_eq!(super::figure_out_type(&ty), "!");
    }
}
