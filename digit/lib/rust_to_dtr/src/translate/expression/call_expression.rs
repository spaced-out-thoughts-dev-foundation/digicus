use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprCall;

pub fn handle_call_expression(
    expr: &ExprCall,
    mut compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut argument_names: Vec<String> = Vec::new();
    let mut index = 1;

    let mut expressions: Vec<Instruction> = Vec::new();
    let original_assignment = compilation_state.next_assignment.clone();

    expr.args.iter().for_each(|arg| {
        let arg_name = format!(
            "CALL_EXPRESSION_ARG_{}_{}",
            index,
            compilation_state.get_global_uuid()
        );

        let expressions_parsed: Vec<Instruction> = match parse_expression(
            &arg,
            &mut compilation_state.with_assignment(Some(arg_name.clone())),
        ) {
            Ok(expressions) => expressions,
            Err(_) => panic!("Error parsing call expression"),
        };

        expressions.extend(expressions_parsed);

        argument_names.push(arg_name);

        index += 1;
    });
    compilation_state = compilation_state.with_assignment(original_assignment.clone());

    let unique_uuid = compilation_state.get_global_uuid();
    let mut func: Vec<Instruction> = parse_expression(
        &expr.func,
        &mut compilation_state.with_assignment(Some(
            format!("CALL_EXPRESSION_FUNCTION_{}", unique_uuid).to_string(),
        )),
    )?;

    compilation_state = compilation_state.with_assignment(original_assignment.clone());

    argument_names.insert(
        0,
        format!("CALL_EXPRESSION_FUNCTION_{}", unique_uuid).to_string(),
    );

    func.extend(expressions);
    func.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "evaluate".to_string(),
        argument_names,
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or("".to_string()),
        compilation_state.scope(),
    ));

    Ok(func)
}

#[cfg(test)]
mod tests {
    use crate::common::compilation_state::CompilationState;
    use crate::instruction::Instruction;
    use crate::translate::expression::parse_expression;
    use syn::ExprCall;

    #[test]
    fn test_handle_call_expression() {
        let parsed_expr_let: ExprCall = syn::parse_str("foo(bar, baz, 10)").unwrap();
        let result = parse_expression(
            &syn::Expr::Call(parsed_expr_let),
            &mut CompilationState::new(),
        );
        let expected: Vec<Instruction> = vec![
            Instruction::new(
                7,
                "assign".to_string(),
                vec!["foo".to_string()],
                "CALL_EXPRESSION_FUNCTION_6".to_string(),
                0,
            ),
            Instruction::new(
                1,
                "assign".to_string(),
                vec!["bar".to_string()],
                "CALL_EXPRESSION_ARG_1_0".to_string(),
                0,
            ),
            Instruction::new(
                3,
                "assign".to_string(),
                vec!["baz".to_string()],
                "CALL_EXPRESSION_ARG_2_2".to_string(),
                0,
            ),
            Instruction::new(
                5,
                "assign".to_string(),
                vec!["10".to_string()],
                "CALL_EXPRESSION_ARG_3_4".to_string(),
                0,
            ),
            Instruction::new(
                8,
                "evaluate".to_string(),
                vec![
                    "CALL_EXPRESSION_FUNCTION_6".to_string(),
                    "CALL_EXPRESSION_ARG_1_0".to_string(),
                    "CALL_EXPRESSION_ARG_2_2".to_string(),
                    "CALL_EXPRESSION_ARG_3_4".to_string(),
                ],
                "".to_string(),
                0,
            ),
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_handle_call_expression_with_path() {
        let parsed_expr_let: ExprCall = syn::parse_str("Self::foo(bar, baz, 10)").unwrap();
        let result = parse_expression(
            &syn::Expr::Call(parsed_expr_let),
            &mut CompilationState::new(),
        );
        let expected: Vec<Instruction> = vec![
            Instruction::new(
                7,
                "assign".to_string(),
                vec!["foo".to_string()],
                "CALL_EXPRESSION_FUNCTION_6".to_string(),
                0,
            ),
            Instruction::new(
                1,
                "assign".to_string(),
                vec!["bar".to_string()],
                "CALL_EXPRESSION_ARG_1_0".to_string(),
                0,
            ),
            Instruction::new(
                3,
                "assign".to_string(),
                vec!["baz".to_string()],
                "CALL_EXPRESSION_ARG_2_2".to_string(),
                0,
            ),
            Instruction::new(
                5,
                "assign".to_string(),
                vec!["10".to_string()],
                "CALL_EXPRESSION_ARG_3_4".to_string(),
                0,
            ),
            Instruction::new(
                8,
                "evaluate".to_string(),
                vec![
                    "CALL_EXPRESSION_FUNCTION_6".to_string(),
                    "CALL_EXPRESSION_ARG_1_0".to_string(),
                    "CALL_EXPRESSION_ARG_2_2".to_string(),
                    "CALL_EXPRESSION_ARG_3_4".to_string(),
                ],
                "".to_string(),
                0,
            ),
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_handle_call_expression_with_struct_init_as_input() {
        let parsed_expr_let: ExprCall = syn::parse_str(
            "unwrap_or(State {
            count: 0,
            last_incr: 0,
        })",
        )
        .unwrap();
        let result = parse_expression(
            &syn::Expr::Call(parsed_expr_let),
            &mut CompilationState::new(),
        );

        let expected: Vec<Instruction> = vec![
            Instruction::new(
                6,
                "assign".to_string(),
                vec!["unwrap_or".to_string()],
                "CALL_EXPRESSION_FUNCTION_5".to_string(),
                0,
            ),
            Instruction::new(
                1,
                "assign".to_string(),
                vec!["0".to_string()],
                "count".to_string(),
                0,
            ),
            Instruction::new(
                2,
                "assign".to_string(),
                vec!["0".to_string()],
                "last_incr".to_string(),
                0,
            ),
            Instruction::new(
                3,
                "instantiate_object".to_string(),
                vec![
                    "UDT".to_string(),
                    "State".to_string(),
                    "count".to_string(),
                    "last_incr".to_string(),
                ],
                "CALL_EXPRESSION_ARG_1_0".to_string(),
                0,
            ),
            Instruction::new(
                7,
                "evaluate".to_string(),
                vec![
                    "CALL_EXPRESSION_FUNCTION_5".to_string(),
                    "CALL_EXPRESSION_ARG_1_0".to_string(),
                ],
                "".to_string(),
                0,
            ),
        ];

        assert_eq!(result, Ok(expected));
    }
}
