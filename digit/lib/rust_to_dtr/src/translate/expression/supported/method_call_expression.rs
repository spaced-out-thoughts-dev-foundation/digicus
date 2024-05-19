use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, translate::expression::parse_expression,
};
use syn::Expr;

pub fn handle_method_call_expression(
    expr: &Expr,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    // if let syn::Expr::MethodCall(expr_method_call) = expr {
    //     let method_name = get_method_names(expr);

    //     let mut input_strings: Vec<String> = Vec::new();

    //     let index = 1;
    //     expr_method_call.args.iter().for_each(|arg| {
    //         let arg_name = match parse_expression(&arg, Some(format!("arg-{}-method_call", index)))
    //         {
    //             Ok(arg_name) => arg_name,
    //             Err(_) => "ERR".to_string(),
    //         };

    //         input_strings.push(arg_name);
    //         index += 1;
    //     });
    // }

    // panic!("Impossible to get here in a method call expression.");

    Ok(vec![Instruction::new(
        "assign".to_string(),
        vec!["INPUT_VALUE_NAME_FOR_METHOD_CALL".to_string()],
        "METHOD_CALL_RESULT".to_string(),
    )])
}

fn translate_env_method_call_expressions(method_name: &str) -> String {
    match method_name {
        "env.storage.instance.set" => "set_state".to_string(),
        "env.storage.instance.extend_ttl" => "extend_ttl".to_string(),
        "env.storage.instance.get" => "fetch_state".to_string(),
        _ => "TF I KNOW".to_string(),
    }
}

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
