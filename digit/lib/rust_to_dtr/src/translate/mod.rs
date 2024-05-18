extern crate syn;

pub mod rust_to_dtr_term;
pub mod type_name;

pub fn parse_return_type(ty: &syn::Type) -> String {
    match type_name::figure_out_type(ty) {
        Ok(val) => format!("\t* Output: {}\n", val),
        Err(_) => format!("\t* Output: Could not figure out type\n"),
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
        syn::Expr::Path(path) => type_name::parse_path(&path.path),
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

fn parse_macros(mac: &syn::ExprMacro) -> String {
    let macro_itself: &syn::Macro = &mac.mac;

    let mut macro_str = String::new();

    // TODO: do all macros have a bang?
    macro_str.push_str(&format!("{}!", type_name::parse_path(&macro_itself.path)));

    macro_str.push_str(format!("{:?}", macro_itself.tokens).as_str());

    macro_str
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
