use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_binary_op;
use crate::translate::expression::parse_expression;

use syn::ExprBinary;

pub fn handle_binary_expression(
    expr_binary: &ExprBinary,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let left_hand_side_name = "BINARY_EXPRESSION_LEFT";
    let right_hand_side_name = "BINARY_EXPRESSION_RIGHT";

    let mut left_hand_side: Vec<Instruction> =
        parse_expression(&expr_binary.left, Some(left_hand_side_name.to_string()))?;
    let right_hand_side: Vec<Instruction> =
        parse_expression(&expr_binary.right, Some(right_hand_side_name.to_string()))?;
    let operator: String = parse_binary_op(&expr_binary.op)?;

    let binary_instruction = Instruction::new(
        operator,
        vec![
            left_hand_side_name.to_string(),
            right_hand_side_name.to_string(),
        ],
        // TODO: this is incorrect!
        assignment.unwrap_or_default(),
    );

    // add all instructions to one vec
    left_hand_side.extend(right_hand_side);
    left_hand_side.push(binary_instruction);

    Ok(left_hand_side)
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::translate::expression::parse_expression;
    use syn::ExprBinary;

    #[test]
    fn test_binary_expression_simple_addition() {
        let parsed_expr_binary: ExprBinary = syn::parse_str("1 + 2").unwrap();
        let result = parse_expression(&syn::Expr::Binary(parsed_expr_binary), None);

        assert_eq!(
            result,
            Ok(vec![
                Instruction::new(
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "BINARY_EXPRESSION_LEFT".to_string()
                ),
                Instruction::new(
                    "assign".to_string(),
                    vec!["2".to_string()],
                    "BINARY_EXPRESSION_RIGHT".to_string()
                ),
                Instruction::new(
                    "add".to_string(),
                    vec![
                        "BINARY_EXPRESSION_LEFT".to_string(),
                        "BINARY_EXPRESSION_RIGHT".to_string()
                    ],
                    "".to_string()
                ),
            ])
        );
    }

    #[test]
    fn test_binary_expression_simple_subtraction_and_assignment() {
        let parsed_expr_binary: ExprBinary = syn::parse_str("foo -= 2").unwrap();
        let result = parse_expression(&syn::Expr::Binary(parsed_expr_binary), None);

        let expected = Ok(vec![
            Instruction::new(
                "assign".to_string(),
                vec!["foo".to_string()],
                "BINARY_EXPRESSION_LEFT".to_string(),
            ),
            Instruction::new(
                "assign".to_string(),
                vec!["2".to_string()],
                "BINARY_EXPRESSION_RIGHT".to_string(),
            ),
            Instruction::new(
                "subtract_and_assign".to_string(),
                vec![
                    "BINARY_EXPRESSION_LEFT".to_string(),
                    "BINARY_EXPRESSION_RIGHT".to_string(),
                ],
                "".to_string(),
            ),
        ]);

        assert_eq!(result, expected);
    }
}
