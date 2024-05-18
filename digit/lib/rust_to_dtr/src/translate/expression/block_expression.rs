use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_expression;
use crate::translate::expression::parse_lit;
use crate::translate::type_name::figure_out_type;
use crate::translate::type_name::parse_path;
use syn::ExprBlock;

// A block is a collection of statements
pub fn handle_block_expression(expr_block: &ExprBlock) -> Result<String, NotTranslatableError> {
    let the_block = expr_block.block.clone();
    let mut block_str: Vec<String> = Vec::new();

    for stmt in the_block.stmts.iter() {
        // let stmt_str = parse_block_stmt(stmt)?;
        // block_str.push_str(&stmt_str);

        match &parse_block_stmt(stmt) {
            Ok(stmt_str) => block_str.push(stmt_str.to_string()),
            Err(e) => return Err(e.clone()),
        }
    }

    Ok(block_str.join("\n"))
}

pub fn parse_block_stmt(stmt: &syn::Stmt) -> Result<String, NotTranslatableError> {
    match stmt {
        syn::Stmt::Local(local) => {
            let let_expr_str = match &local.init {
                Some((local_init)) => parse_expression(&local_init.expr)?,
                None => {
                    return Err(NotTranslatableError::Custom(
                        "No expression in let".to_string(),
                    ))
                }
            };

            match &local.pat {
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
                syn::Pat::Type(type_pat) => Ok(format!(
                    "{{ instruction: assign, input: ({}), assign: {} }}",
                    let_expr_str,
                    figure_out_type(&type_pat.ty)?
                )),
                syn::Pat::Verbatim(_) => Ok(format!("Verbatim")),
                syn::Pat::Wild(_) => Ok(format!("Wild")),
                _ => Err(NotTranslatableError::Custom(
                    "Unknown pattern in block pat".to_string(),
                )),
            }
        }
        syn::Stmt::Item(_item) => Err(NotTranslatableError::Custom(
            "Item statement not translatable".to_string(),
        )),
        syn::Stmt::Expr(exp, _r) => parse_expression(exp),
        syn::Stmt::Macro(_mac) => Err(NotTranslatableError::Custom(
            "Macro statement not translatable".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::translate::expression::parse_expression;
    use syn::ExprBlock;

    #[test]
    fn test_block_expression() {
        let parsed_expr_block: ExprBlock = syn::parse_str("{ let x = 1; let foo = bar; }").unwrap();
        let result = parse_expression(&syn::Expr::Block(parsed_expr_block));

        let expected = "{ instruction: assign, input: (1), assign: x }
{ instruction: assign, input: (bar), assign: foo }";

        assert_eq!(result, Ok(expected.to_string()));
    }
}
