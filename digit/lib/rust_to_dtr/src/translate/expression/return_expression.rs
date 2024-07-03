use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use syn::ExprReturn;

pub fn handle_return_expression(
    expr_return: &ExprReturn,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let return_expr_box = &expr_return.expr;

    match return_expr_box {
        Some(return_expr) => {
            let global_uuid = compilation_state.get_global_uuid();
            let return_label: &str = &format!("RETURN_VALUE_LABEL_{}", global_uuid);

            let original_assignment = compilation_state.next_assignment.clone();

            let mut precedning_instructions = parse_expression(
                return_expr,
                &mut compilation_state.with_assignment(Some(return_label.to_string())),
            )?;

            compilation_state.with_assignment(original_assignment);

            let return_instruction = Instruction::new(
                compilation_state.get_global_uuid(),
                "return".to_string(),
                vec![return_label.to_string()],
                "".to_string(),
                compilation_state.scope(),
            );

            precedning_instructions.push(return_instruction);

            Ok(precedning_instructions)
        }
        None => Err(NotTranslatableError::Custom(
            "Return expression not translatable".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::compilation_state::CompilationState;
    use syn::ExprReturn;

    // use crate::common::get_random_string;

    #[test]
    fn test_return_expression_int() {
        // let mut mock = Mockget_random_string::new();
        // mock.return_string("10");

        let parsed_expr_return: ExprReturn = syn::parse_str("return 1").unwrap();
        let result = parse_expression(
            &syn::Expr::Return(parsed_expr_return),
            &mut CompilationState::new(),
        );
        let expected: Vec<Instruction> = vec![
            Instruction::new(
                1,
                "assign".to_string(),
                vec!["1".to_string()],
                "RETURN_VALUE_LABEL_0".to_string(),
                0,
            ),
            Instruction::new(
                2,
                "return".to_string(),
                vec!["RETURN_VALUE_LABEL_0".to_string()],
                "".to_string(),
                0,
            ),
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_return_expression_bool() {
        let parsed_expr_return: ExprReturn = syn::parse_str("return true").unwrap();
        let result = parse_expression(
            &syn::Expr::Return(parsed_expr_return),
            &mut CompilationState::new(),
        );

        let expected: Vec<Instruction> = vec![
            Instruction::new(
                1,
                "assign".to_string(),
                vec!["true".to_string()],
                "RETURN_VALUE_LABEL_0".to_string(),
                0,
            ),
            Instruction::new(
                2,
                "return".to_string(),
                vec!["RETURN_VALUE_LABEL_0".to_string()],
                "".to_string(),
                0,
            ),
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_return_expression_no_expr() {
        let parsed_expr_return: ExprReturn = syn::parse_str("return").unwrap();
        let result = parse_expression(
            &syn::Expr::Return(parsed_expr_return),
            &mut CompilationState::new(),
        );

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Return expression not translatable".to_string()
            ))
        );
    }
}
