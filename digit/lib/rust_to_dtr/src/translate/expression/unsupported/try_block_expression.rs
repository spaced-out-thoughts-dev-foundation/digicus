use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprTryBlock;

pub fn handle_try_block_expression(
    _: &ExprTryBlock,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "TryBlock expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::compilation_state::CompilationState;
    use crate::translate::expression::parse_expression;
    use syn::ExprTryBlock;

    #[test]
    fn test_try_block_expression() {
        let parsed_expr_try_block: ExprTryBlock = syn::parse_str("try { }").unwrap();
        let result = parse_expression(
            &syn::Expr::TryBlock(parsed_expr_try_block),
            &mut CompilationState::new(),
        );

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "TryBlock expression not translatable".to_string()
            ))
        );
    }
}
