use crate::common::handle_macro;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprMacro;

pub fn handle_macro_expression(
    expr: &ExprMacro,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    handle_macro(&expr.mac, assignment)
}
