use crate::errors::not_translatable_error::NotTranslatableError;
use syn::Expr;

use super::parse_expression;

pub fn handle_method_call_expression(expr: &Expr) -> Result<String, NotTranslatableError> {
    if let syn::Expr::MethodCall(expr_method_call) = expr {
        let method_name = get_method_names(expr);
        // let starting_name = parse_expression(&expr_method_call.receiver)?;

        let mut input_strings: Vec<String> = Vec::new();

        expr_method_call.args.iter().for_each(|arg| {
            let arg_name = match parse_expression(&arg) {
                Ok(arg_name) => arg_name,
                Err(_) => "ERR".to_string(),
            };

            input_strings.push(arg_name);
        });

        return Ok(format!(
            "{{ instruction: {}, input: ({})}}",
            translate_env_method_call_expressions(&format!("{}", method_name)),
            input_strings.join(", ")
        ));
    }

    Ok(format!("idk man"))
}

fn translate_env_method_call_expressions(method_name: &str) -> String {
    match method_name {
        "env.storage.instance.set" => "set_state".to_string(),
        "env.storage.instance.extend_ttl" => "extend_ttl".to_string(),
        "env.storage.instance.get" => "fetch_state".to_string(),
        _ => "TF I KNOW".to_string(),
    }
}

fn get_method_names(expr: &Expr) -> String {
    if let syn::Expr::MethodCall(expr_method_call) = expr {
        // Recursively handle the receiver
        let result: String = get_method_names(&expr_method_call.receiver);

        // Get the method name
        let method_name = expr_method_call.method.to_string();
        return format!("{}.{}", result, method_name);
    } else {
        return parse_expression(&expr).unwrap();
    }
}
