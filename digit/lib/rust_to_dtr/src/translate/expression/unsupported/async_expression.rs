use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprAsync;

pub fn handle_async_expression(_: &ExprAsync) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Async expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::compilation_state::CompilationState;
    use crate::translate::expression::parse_expression;
    use syn::ExprAsync;

    #[test]
    fn test_async_expression() {
        let parsed_expr_async: ExprAsync = syn::parse_str("async { }").unwrap();
        let result = parse_expression(
            &syn::Expr::Async(parsed_expr_async),
            &mut CompilationState::new(),
        );

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Async expression not translatable".to_string()
            ))
        );
    }
}
