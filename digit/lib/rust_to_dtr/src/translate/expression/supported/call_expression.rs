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
        let arg_name = format!("{}_CALL_EXPRESSION_ARG", index);
        let expressions_parsed: Vec<Instruction> =
            match parse_expression(&arg, Some(arg_name.clone())) {
                Ok(expressions) => expressions,
                Err(_) => panic!("Error parsing call expression"),
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
        assignment.unwrap_or("CALL_EXPRESSION_RESULT".to_string()),
    ));

    Ok(func)
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::translate::expression::parse_expression;
    use syn::ExprCall;

    #[test]
    fn test_handle_call_expression() {
        let parsed_expr_let: ExprCall = syn::parse_str("foo(bar, baz, 10)").unwrap();
        let result = parse_expression(&syn::Expr::Call(parsed_expr_let), None);
        let expected: Vec<Instruction> = vec![
            Instruction::new(
                "assign".to_string(),
                vec!["foo".to_string()],
                "CALL_EXPRESSION_FUNCTION".to_string(),
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["bar".to_string()],
                "1_CALL_EXPRESSION_ARG".to_string(),
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["baz".to_string()],
                "2_CALL_EXPRESSION_ARG".to_string(),
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["10".to_string()],
                "3_CALL_EXPRESSION_ARG".to_string(),
            ),
            Instruction::new(
                "evaluate".to_string(),
                vec![
                    "CALL_EXPRESSION_FUNCTION".to_string(),
                    "1_CALL_EXPRESSION_ARG".to_string(),
                    "2_CALL_EXPRESSION_ARG".to_string(),
                    "3_CALL_EXPRESSION_ARG".to_string(),
                ],
                "CALL_EXPRESSION_RESULT".to_string(),
            ),
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_handle_call_expression_with_path() {
        let parsed_expr_let: ExprCall = syn::parse_str("Self::foo(bar, baz, 10)").unwrap();
        let result = parse_expression(&syn::Expr::Call(parsed_expr_let), None);
        let expected: Vec<Instruction> = vec![
            Instruction::new(
                "assign".to_string(),
                vec!["foo".to_string()],
                "CALL_EXPRESSION_FUNCTION".to_string(),
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["bar".to_string()],
                "1_CALL_EXPRESSION_ARG".to_string(),
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["baz".to_string()],
                "2_CALL_EXPRESSION_ARG".to_string(),
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["10".to_string()],
                "3_CALL_EXPRESSION_ARG".to_string(),
            ),
            Instruction::new(
                "evaluate".to_string(),
                vec![
                    "CALL_EXPRESSION_FUNCTION".to_string(),
                    "1_CALL_EXPRESSION_ARG".to_string(),
                    "2_CALL_EXPRESSION_ARG".to_string(),
                    "3_CALL_EXPRESSION_ARG".to_string(),
                ],
                "CALL_EXPRESSION_RESULT".to_string(),
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
        let result = parse_expression(&syn::Expr::Call(parsed_expr_let), None);
        let expected: Vec<Instruction> = vec![
            Instruction::new(
                "assign".to_string(),
                vec!["unwrap_or".to_string()],
                "CALL_EXPRESSION_FUNCTION".to_string(),
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["0".to_string()],
                "count".to_string(),
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["0".to_string()],
                "last_incr".to_string(),
            ),
            Instruction::new(
                "initialize_udt".to_string(),
                vec![
                    "State".to_string(),
                    "count".to_string(),
                    "last_incr".to_string(),
                ],
                "1_CALL_EXPRESSION_ARG".to_string(),
            ),
            Instruction::new(
                "evaluate".to_string(),
                vec![
                    "CALL_EXPRESSION_FUNCTION".to_string(),
                    "1_CALL_EXPRESSION_ARG".to_string(),
                ],
                "CALL_EXPRESSION_RESULT".to_string(),
            ),
        ];

        match &result {
            Ok(result_thing) => {
                // assert_eq!(result.len(), expected.len());
                for (index, instruction) in result_thing.iter().enumerate() {
                    println!("{:?}", instruction);
                }
            }
            Err(_) => assert!(false),
        }

        println!("\n");

        assert_eq!(result, Ok(expected));
    }
}
