use std::vec;

use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprClosure;

pub fn handle_closure_expression(
    expr: &ExprClosure,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut input_argument_names: Vec<String> = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    expr.inputs.iter().for_each(|input| {
        input_argument_names.push(handle_pattern(input.clone()).unwrap());
    });

    instructions.extend(parse_expression(&expr.body, compilation_state)?);

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_handle_closure_expression() {
        let expr: ExprClosure = syn::parse_str("|a, b| { a + b }").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_closure_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    2,
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "BINARY_EXPRESSION_LEFT_0".to_string(),
                    0,
                ),
                Instruction::new(
                    3,
                    "assign".to_string(),
                    vec!["b".to_string()],
                    "BINARY_EXPRESSION_RIGHT_1".to_string(),
                    0,
                ),
                Instruction::new(
                    4,
                    "add".to_string(),
                    vec![
                        "BINARY_EXPRESSION_LEFT_0".to_string(),
                        "BINARY_EXPRESSION_RIGHT_1".to_string()
                    ],
                    "".to_string(),
                    0,
                ),
            ]
        );
    }
}
