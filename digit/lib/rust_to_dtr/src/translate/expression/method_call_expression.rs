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
    compilation_state.clone().debug_state();

    let mut argument_names: Vec<String> = Vec::new();
    let mut index = 1;

    let mut expressions: Vec<Instruction> = Vec::new();
    expr.args.iter().for_each(|arg| {
        let unique_uuid = compilation_state.get_global_uuid();
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

    let unique_uuid = compilation_state.get_global_uuid();
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
        compilation_state.get_global_uuid(),
        "evaluate".to_string(),
        argument_names,
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or("".to_string()),
        compilation_state.scope(),
    ));

    Ok(receiver)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::compilation_state::CompilationState;
    use crate::instruction::Instruction;
    use syn::parse_quote;

    #[test]
    fn test_handle_method_call_expression() {
        let expr = parse_quote! { "hello".to_string().to_uppercase(10, 11) };
        let mut compilation_state = CompilationState::new();

        let result = handle_method_call_expression(&expr, &mut compilation_state);

        assert_eq!(result.is_ok(), true);

        let instructions = result.unwrap();

        assert_eq!(instructions.len(), 5);

        assert_eq!(
            instructions[0],
            Instruction::new(
                6,
                "assign".to_string(),
                vec!["\"hello\"".to_string(),],
                "METHOD_CALL_EXPRESSION_5".to_string(),
                0
            )
        );

        assert_eq!(
            instructions[1],
            Instruction::new(
                7,
                "evaluate".to_string(),
                vec!["METHOD_CALL_EXPRESSION_5.to_string".to_string()],
                "METHOD_CALL_EXPRESSION_4".to_string(),
                0
            )
        );

        assert_eq!(
            instructions[2],
            Instruction::new(
                1,
                "assign".to_string(),
                vec!["10".to_string()],
                "METHOD_CALL_ARG_1_0".to_string(),
                0
            )
        );

        assert_eq!(
            instructions[3],
            Instruction::new(
                3,
                "assign".to_string(),
                vec!["11".to_string()],
                "METHOD_CALL_ARG_2_2".to_string(),
                0
            )
        );

        assert_eq!(
            instructions[4],
            Instruction::new(
                8,
                "evaluate".to_string(),
                vec![
                    "METHOD_CALL_EXPRESSION_4.to_uppercase".to_string(),
                    "METHOD_CALL_ARG_1_0".to_string(),
                    "METHOD_CALL_ARG_2_2".to_string()
                ],
                "".to_string(),
                0
            )
        );
    }
}
