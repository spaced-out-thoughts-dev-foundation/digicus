use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprRange;

pub fn handle_range_expression(
    expr: &ExprRange,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut start_label: Vec<Instruction> = match expr.clone().start {
        Some(start) => parse_expression(&start, compilation_state)?,
        None => vec![],
    };

    let end_label: Vec<Instruction> = match expr.clone().end {
        Some(end) => parse_expression(&end, compilation_state)?,
        None => vec![],
    };

    start_label.extend(end_label);

    Ok(start_label)
}
