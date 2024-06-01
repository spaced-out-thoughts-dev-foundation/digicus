// use super::pattern::handle_pattern;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::block::handle_block;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;
use crate::translate::statement::macro_statement::handle_macro_statement;
use syn::ExprBlock;

// A block is a collection of statements
pub fn handle_block_expression(
    expr_block: &ExprBlock,
    _assignment: Option<String>,
    scope: u32,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    Ok(handle_block(&expr_block.block, scope))
}

pub fn parse_block_stmt(
    stmt: &syn::Stmt,
    assignment: Option<String>,
    scope: u32,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    match stmt {
        syn::Stmt::Local(local) => {
            let pattern_as_string = handle_pattern(local.pat.clone()).unwrap();
            match &local.init {
                Some(local_init) => {
                    parse_expression(&local_init.expr, Some(pattern_as_string), scope)
                }
                None => Ok(vec![Instruction::new(
                    "assign".to_string(),
                    vec![pattern_as_string],
                    "".to_string(),
                    scope,
                )]),
            }
        }
        syn::Stmt::Item(_item) => Err(NotTranslatableError::Custom(
            "Item statement not translatable".to_string(),
        )),
        syn::Stmt::Expr(exp, _r) => parse_expression(exp, assignment, scope),
        syn::Stmt::Macro(stmt_mac) => handle_macro_statement(stmt_mac, assignment, scope),
    }
}
