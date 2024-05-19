use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;

pub fn handle_let_expression(
    let_expr: syn::ExprLet,
    _assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let input_value_name_for_let = "INPUT_VALUE_NAME_FOR_LET";
    let mut preceding_instructions =
        parse_expression(&let_expr.expr, Some(input_value_name_for_let.to_string()))?;
    let result = handle_pattern(*(let_expr.pat.clone()));
    let result_instruction: Instruction = Instruction::new(
        "assign".to_string(),
        vec![input_value_name_for_let.to_string()],
        result.unwrap_or_default(),
    );

    preceding_instructions.push(result_instruction);

    Ok(preceding_instructions)
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::translate::expression::parse_expression;
    use syn;

    mod let_expression {
        use syn::ExprLet;

        use super::*;

        #[test]
        fn test_let_expression_simple_x_equals_1() {
            let parsed_expr_let: ExprLet = syn::parse_str("let x = 1").unwrap();
            let result = parse_expression(&syn::Expr::Let(parsed_expr_let), None);
            let expected: Vec<Instruction> = vec![
                Instruction::new(
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "INPUT_VALUE_NAME_FOR_LET".to_string(),
                ),
                Instruction::new(
                    "assign".to_string(),
                    vec!["INPUT_VALUE_NAME_FOR_LET".to_string()],
                    "x".to_string(),
                ),
            ];

            assert_eq!(result, Ok(expected));
        }

        #[test]
        fn test_let_expression_less_simple_foo_equals_bar() {
            let parsed_expr_let: ExprLet = syn::parse_str("let foo = bar").unwrap();
            let result = parse_expression(&syn::Expr::Let(parsed_expr_let), None);
            let expected = vec![
                Instruction::new(
                    "assign".to_string(),
                    vec!["bar".to_string()],
                    "INPUT_VALUE_NAME_FOR_LET".to_string(),
                ),
                Instruction::new(
                    "assign".to_string(),
                    vec!["INPUT_VALUE_NAME_FOR_LET".to_string()],
                    "foo".to_string(),
                ),
            ];

            assert_eq!(result, Ok(expected));
        }
    }
}
