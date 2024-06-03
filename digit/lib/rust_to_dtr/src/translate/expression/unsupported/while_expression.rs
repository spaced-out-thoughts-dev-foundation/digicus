use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprWhile;

pub fn handle_while_expression(_: &ExprWhile) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "While expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::compilation_state::CompilationState;
    use crate::translate::expression::parse_expression;
    use syn::ExprWhile;

    #[test]
    fn test_while_expression() {
        let parsed_expr_while: ExprWhile = syn::parse_str("while true { }").unwrap();
        let result = parse_expression(
            &syn::Expr::While(parsed_expr_while),
            &mut CompilationState::new(),
        );

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "While expression not translatable".to_string()
            ))
        );
    }
}
