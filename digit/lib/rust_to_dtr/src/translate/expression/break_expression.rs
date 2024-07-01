use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprBreak;

pub fn handle_break_expression(
    _expr: &ExprBreak,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    compilation_state.exit_scope();

    Ok(vec![Instruction::from_compilation_state(
        "goto".to_string(),
        vec![compilation_state.scope().to_string()],
        compilation_state,
    )])
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_handle_break_expression() {
        let expr: ExprBreak = syn::parse_str("break").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();

        compilation_state.enter_new_scope();

        let instructions = handle_break_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![Instruction::new(
                1,
                "goto".to_string(),
                vec!["0".to_string()],
                "".to_string(),
                0,
            ),]
        );
    }
}
