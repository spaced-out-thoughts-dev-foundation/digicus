use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::is_conditional_comparative_operator;
use crate::translate::expression::parse_binary_op;
use crate::translate::expression::parse_expression;
use syn::ExprBinary;

pub fn handle_binary_expression(
    expr_binary: &ExprBinary,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut global_uuid = compilation_state.get_global_uuid();
    let left_hand_side_name = format!("BINARY_EXPRESSION_LEFT_{}", global_uuid);
    global_uuid = compilation_state.get_global_uuid();
    let right_hand_side_name = format!("BINARY_EXPRESSION_RIGHT_{}", global_uuid);

    let mut left_hand_side: Vec<Instruction> = parse_expression(
        &expr_binary.left,
        &mut compilation_state.with_assignment(Some(left_hand_side_name.to_string())),
    )?;
    let right_hand_side: Vec<Instruction> = parse_expression(
        &expr_binary.right,
        &mut compilation_state.with_assignment(Some(right_hand_side_name.to_string())),
    )?;
    let operator: String = parse_binary_op(&expr_binary.op)?;

    let binary_instruction = if is_conditional_comparative_operator(&expr_binary.op) {
        Instruction::new(
            "evaluate".to_string(),
            vec![
                operator,
                left_hand_side_name.to_string(),
                right_hand_side_name.to_string(),
            ],
            compilation_state
                .next_assignment
                .clone()
                .unwrap_or_default(),
            compilation_state.scope,
        )
    } else {
        Instruction::new(
            operator,
            vec![
                left_hand_side_name.to_string(),
                right_hand_side_name.to_string(),
            ],
            // TODO: this is incorrect!
            compilation_state
                .next_assignment
                .clone()
                .unwrap_or_default(),
            compilation_state.scope,
        )
    };

    // add all instructions to one vec
    left_hand_side.extend(right_hand_side);
    left_hand_side.push(binary_instruction);

    Ok(left_hand_side)
}

#[cfg(test)]
mod tests {
    use crate::common::compilation_state::CompilationState;
    use crate::instruction::Instruction;
    use crate::translate::expression::parse_expression;
    use syn::ExprBinary;

    #[test]
    fn test_binary_expression_simple_addition() {
        let parsed_expr_binary: ExprBinary = syn::parse_str("1 + 2").unwrap();
        let result = parse_expression(
            &syn::Expr::Binary(parsed_expr_binary),
            &mut CompilationState::new(),
        );

        assert_eq!(
            result,
            Ok(vec![
                Instruction::new(
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "BINARY_EXPRESSION_LEFT_0".to_string(),
                    0,
                ),
                Instruction::new(
                    "assign".to_string(),
                    vec!["2".to_string()],
                    "BINARY_EXPRESSION_RIGHT_1".to_string(),
                    0,
                ),
                Instruction::new(
                    "add".to_string(),
                    vec![
                        "BINARY_EXPRESSION_LEFT_0".to_string(),
                        "BINARY_EXPRESSION_RIGHT_1".to_string(),
                    ],
                    "".to_string(),
                    0
                ),
            ])
        );
    }

    #[test]
    fn test_binary_expression_simple_subtraction_and_assignment() {
        let parsed_expr_binary: ExprBinary = syn::parse_str("foo -= 2").unwrap();
        let result = parse_expression(
            &syn::Expr::Binary(parsed_expr_binary),
            &mut CompilationState::new(),
        );

        let expected = Ok(vec![
            Instruction::new(
                "assign".to_string(),
                vec!["foo".to_string()],
                "BINARY_EXPRESSION_LEFT_0".to_string(),
                0,
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["2".to_string()],
                "BINARY_EXPRESSION_RIGHT_1".to_string(),
                0,
            ),
            Instruction::new(
                "subtract_and_assign".to_string(),
                vec![
                    "BINARY_EXPRESSION_LEFT_0".to_string(),
                    "BINARY_EXPRESSION_RIGHT_1".to_string(),
                ],
                "".to_string(),
                0,
            ),
        ]);

        assert_eq!(result, expected);
    }
}
