use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprTuple;

pub fn handle_tuple_expression(
    expr: &ExprTuple,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut instructions_to_return: Vec<Instruction> = Vec::new();

    let mut index = 1;
    let mut arguments: Vec<String> = Vec::new();
    expr.elems.iter().for_each(|arg| {
        let arg_name = format!("{}_TUPLE_ARG", index);

        arguments.push(arg_name.clone());

        let instructions: Vec<Instruction> =
            parse_expression(arg, &mut compilation_state.with_assignment(Some(arg_name))).unwrap();
        instructions_to_return.extend(instructions);

        index += 1;
    });

    instructions_to_return.push(Instruction::new(
        "create_tuple".to_string(),
        arguments,
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or("TUPLE_RESULT".to_string()),
        compilation_state.scope,
    ));

    Ok(instructions_to_return)
}
