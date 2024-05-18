// use super::pattern::handle_pattern;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;
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
            // let let_expr_str = match &local.init {
            //     Some((local_init)) => parse_expression(&local_init.expr)?,
            //     None => {
            //         return Err(NotTranslatableError::Custom(
            //             "No expression in let".to_string(),
            //         ))
            //     }
            // };

            handle_pattern(local.pat.clone())
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
