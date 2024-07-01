use core::panic;

use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;

pub fn handle_let_expression(
    let_expr: syn::ExprLet,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let global_uuid = compilation_state.get_global_uuid();
    let input_value_name_for_let = format!("INPUT_VALUE_NAME_FOR_LET_{}", global_uuid);
    let mut preceding_instructions = parse_expression(
        &let_expr.expr,
        &mut compilation_state.with_assignment(Some(input_value_name_for_let.to_string())),
    )?;
    let result = handle_pattern(*(let_expr.pat.clone()));
    let result_instruction: Instruction = Instruction::new(
        compilation_state.get_global_uuid(),
        "assign".to_string(),
        vec![input_value_name_for_let.to_string()],
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or(result.unwrap_or_default()),
        compilation_state.scope(),
    );

    preceding_instructions.push(result_instruction);

    Ok(preceding_instructions)
}

#[cfg(test)]
mod tests {
    use crate::common::compilation_state::CompilationState;
    use crate::instruction::Instruction;
    use crate::translate::expression::parse_expression;
    use syn;

    mod let_expression {
        use syn::ExprLet;

        use super::*;

        #[test]
        fn test_let_expression_simple_x_equals_1() {
            let parsed_expr_let: ExprLet = syn::parse_str("let x = 1").unwrap();
            let result = parse_expression(
                &syn::Expr::Let(parsed_expr_let),
                &mut CompilationState::new(),
            );
            let expected: Vec<Instruction> = vec![
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "INPUT_VALUE_NAME_FOR_LET_0".to_string(),
                    0,
                ),
                Instruction::new(
                    2,
                    "assign".to_string(),
                    vec!["INPUT_VALUE_NAME_FOR_LET_0".to_string()],
                    "x".to_string(),
                    0,
                ),
            ];

            assert_eq!(result, Ok(expected));
        }

        #[test]
        fn test_let_expression_less_simple_foo_equals_bar() {
            let parsed_expr_let: ExprLet = syn::parse_str("let foo = bar").unwrap();
            let result = parse_expression(
                &syn::Expr::Let(parsed_expr_let),
                &mut CompilationState::new(),
            );
            let expected = vec![
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["bar".to_string()],
                    "INPUT_VALUE_NAME_FOR_LET_0".to_string(),
                    0,
                ),
                Instruction::new(
                    2,
                    "assign".to_string(),
                    vec!["INPUT_VALUE_NAME_FOR_LET_0".to_string()],
                    "foo".to_string(),
                    0,
                ),
            ];

            assert_eq!(result, Ok(expected));
        }
    }
}
