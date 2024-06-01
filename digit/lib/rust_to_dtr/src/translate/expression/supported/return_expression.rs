use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use syn::ExprReturn;

pub fn handle_return_expression(
    expr_return: &ExprReturn,
    assignment: Option<String>,
    scope: u32,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let return_expr_box = &expr_return.expr;

    match return_expr_box {
        Some(return_expr) => {
            let return_label: &str = "RETURN_VALUE_LABEL";

            let mut precedning_instructions =
                parse_expression(return_expr, Some(return_label.to_string()), scope)?;

            let return_instruction = Instruction::new(
                "return".to_string(),
                vec![return_label.to_string()],
                assignment.unwrap_or_default(),
                scope,
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
    use syn::ExprReturn;

    // use crate::common::get_random_string;

    #[test]
    fn test_return_expression_int() {
        // let mut mock = Mockget_random_string::new();
        // mock.return_string("10");

        let parsed_expr_return: ExprReturn = syn::parse_str("return 1").unwrap();
        let result = parse_expression(&syn::Expr::Return(parsed_expr_return), None, 0);
        let expected: Vec<Instruction> = vec![
            Instruction::new(
                "assign".to_string(),
                vec!["1".to_string()],
                "RETURN_VALUE_LABEL".to_string(),
                0,
            ),
            Instruction::new(
                "return".to_string(),
                vec!["RETURN_VALUE_LABEL".to_string()],
                "".to_string(),
                0,
            ),
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_return_expression_bool() {
        let parsed_expr_return: ExprReturn = syn::parse_str("return true").unwrap();
        let result = parse_expression(&syn::Expr::Return(parsed_expr_return), None, 0);

        let expected: Vec<Instruction> = vec![
            Instruction::new(
                "assign".to_string(),
                vec!["true".to_string()],
                "RETURN_VALUE_LABEL".to_string(),
                0,
            ),
            Instruction::new(
                "return".to_string(),
                vec!["RETURN_VALUE_LABEL".to_string()],
                "".to_string(),
                0,
            ),
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_return_expression_no_expr() {
        let parsed_expr_return: ExprReturn = syn::parse_str("return").unwrap();
        let result = parse_expression(&syn::Expr::Return(parsed_expr_return), None, 0);

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Return expression not translatable".to_string()
            ))
        );
    }
}
