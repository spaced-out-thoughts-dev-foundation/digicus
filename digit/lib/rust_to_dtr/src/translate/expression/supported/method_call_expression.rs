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
        let arg_name = format!("{}_METHOD_CALL_ARG", index);
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

    let mut receiver: Vec<Instruction> = parse_expression(
        &expr.receiver,
        &mut compilation_state.with_assignment(Some("METHOD_CALL_EXPRESSION".to_string())),
    )?;

    receiver.extend(expressions);

    argument_names.insert(
        0,
        format!("METHOD_CALL_EXPRESSION.{}", expr.method.to_string()),
    );

    receiver.push(Instruction::new(
        "evaluate".to_string(),
        argument_names,
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or("METHOD_CALL_RESULT".to_string()),
        compilation_state.scope,
    ));

    Ok(receiver)
}

// fn translate_env_method_call_expressions(method_name: &str) -> String {
//     match method_name {
//         "env.storage.instance.set" => "set_state".to_string(),
//         "env.storage.instance.extend_ttl" => "extend_ttl".to_string(),
//         "env.storage.instance.get" => "fetch_state".to_string(),
//         _ => "TF I KNOW".to_string(),
//     }
// }

// fn get_method_names(expr: &Expr) -> String {
//     if let syn::Expr::MethodCall(expr_method_call) = expr {
//         // Recursively handle the receiver
//         let result: String = get_method_names(&expr_method_call.receiver);

//         // Get the method name
//         let method_name = expr_method_call.method.to_string();
//         return format!("{}.{}", result, method_name);
//     } else {
//         return parse_expression(&expr, None).unwrap();
//     }
// }
