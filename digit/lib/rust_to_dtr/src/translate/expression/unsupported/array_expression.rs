use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprArray;

pub fn handle_array_expression(_: &ExprArray) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Array expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::compilation_state::CompilationState;
    use crate::translate::expression::parse_expression;
    use syn::ExprArray;

    #[test]
    fn test_array_expression() {
        let parsed_expr_array: ExprArray = syn::parse_str("[1,2,3]").unwrap();
        let result = parse_expression(
            &syn::Expr::Array(parsed_expr_array),
            &mut CompilationState::new(),
        );

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Array expression not translatable".to_string()
            ))
        );
    }
}
