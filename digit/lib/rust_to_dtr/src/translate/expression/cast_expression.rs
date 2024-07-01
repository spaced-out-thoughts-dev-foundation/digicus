use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprCast;

pub fn handle_cast_expression(
    expr: &ExprCast,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    // TODO: don't skip this cast in the future
    parse_expression(&expr.expr, compilation_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_handle_cast_expression() {
        let expr: ExprCast = syn::parse_str("a as i32").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_cast_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["a".to_string()],
                "".to_string(),
                0,
            ),]
        );
    }
}
