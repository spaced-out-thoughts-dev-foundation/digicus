use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprCall;

pub fn handle_call_expression(
    expr: &ExprCall,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut argument_names: Vec<String> = Vec::new();
    let mut index = 1;

    let mut expressions: Vec<Instruction> = Vec::new();
    expr.args.iter().for_each(|arg| {
        let arg_name = format!("{} METHOD_CALL_ARG", index);
        let expressions_parsed: Vec<Instruction> =
            match parse_expression(&arg, Some(arg_name.clone())) {
                Ok(expressions) => expressions,
                Err(_) => Vec::new(),
            };

        expressions.extend(expressions_parsed);

        argument_names.push(arg_name);

        index += 1;
    });

    let mut func: Vec<Instruction> =
        parse_expression(&expr.func, Some("CALL_EXPRESSION_FUNCTION".to_string()))?;

    argument_names.insert(0, "CALL_EXPRESSION_FUNCTION".to_string());

    func.extend(expressions);
    func.push(Instruction::new(
        "evaluate".to_string(),
        argument_names,
        "CALL_EXPRESSION_RESULT".to_string(),
    ));

    Ok(func)
}
