use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprTry;

pub fn handle_try_expression(_: &ExprTry) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Try expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::compilation_state::CompilationState;
    use crate::translate::expression::parse_expression;
    use syn::ExprTry;

    #[test]
    fn test_try_expression() {
        let parsed_expr_try: ExprTry = syn::parse_str("expr?").unwrap();
        let result = parse_expression(
            &syn::Expr::Try(parsed_expr_try),
            &mut CompilationState::new(),
        );

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Try expression not translatable".to_string()
            ))
        );
    }
}
