use crate::common::compilation_state;
use crate::common::handle_macro;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprMacro;

pub fn handle_macro_expression(
    expr: &ExprMacro,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    handle_macro(
        &expr.mac,
        compilation_state.next_assignment.clone(),
        compilation_state.clone(),
    )
}
