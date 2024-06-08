use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprMethodCall;

pub fn handle_method_call_expression(
    expr: &ExprMethodCall,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut argument_names: Vec<String> = Vec::new();
    let mut index = 1;

    let mut expressions: Vec<Instruction> = Vec::new();
    expr.args.iter().for_each(|arg| {
        let unique_uuid = compilation_state.global_uuid;
        compilation_state.increment_global_uuid();
        let arg_name = format!("METHOD_CALL_ARG_{}_{}", index, unique_uuid);
        let expressions_parsed: Vec<Instruction> = match parse_expression(
            &arg,
            &mut compilation_state.with_assignment(Some(arg_name.clone())),
        ) {
            Ok(expressions) => expressions,
            Err(e) => panic!("Error parsing expression: {:?}", e),
        };

        expressions.extend(expressions_parsed);

        argument_names.push(arg_name);

        index += 1;
    });

    let unique_uuid = compilation_state.global_uuid;
    compilation_state.increment_global_uuid();
    let mut receiver: Vec<Instruction> = parse_expression(
        &expr.receiver,
        &mut compilation_state.with_assignment(Some(
            format!("METHOD_CALL_EXPRESSION_{}", unique_uuid).to_string(),
        )),
    )?;

    receiver.extend(expressions);

    argument_names.insert(
        0,
        format!(
            "METHOD_CALL_EXPRESSION_{}.{}",
            unique_uuid,
            expr.method.to_string()
        ),
    );

    receiver.push(Instruction::new(
        "evaluate".to_string(),
        argument_names,
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or("".to_string()),
        compilation_state.scope,
    ));

    Ok(receiver)
}
