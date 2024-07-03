use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprBreak;

pub fn handle_break_expression(
    _expr: &ExprBreak,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let prev_scope = compilation_state.scope();
    compilation_state.exit_scope();

    Ok(vec![Instruction::new(
        1,
        "jump".to_string(),
        vec![compilation_state.scope().to_string()],
        "".to_string(),
        prev_scope,
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
                "jump".to_string(),
                vec![compilation_state.scope().to_string()],
                "".to_string(),
                0,
            ),]
        );
    }
}
