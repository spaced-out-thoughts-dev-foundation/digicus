use std::vec;

use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprClosure;

pub fn handle_closure_expression(
    expr: &ExprClosure,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut input_argument_names: Vec<String> = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    expr.inputs.iter().for_each(|input| {
        input_argument_names.push(handle_pattern(input.clone()).unwrap());
    });

    instructions.extend(parse_expression(&expr.body, compilation_state)?);

    Ok(instructions)
}
