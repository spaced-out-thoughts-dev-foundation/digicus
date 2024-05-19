use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprUnsafe;

pub fn handle_unsafe_expression(_: &ExprUnsafe) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Unsafe expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprUnsafe;

    #[test]
    fn test_unsafe_expression() {
        let parsed_expr_unsafe: ExprUnsafe = syn::parse_str("unsafe { }").unwrap();
        let result = parse_expression(&syn::Expr::Unsafe(parsed_expr_unsafe), None);

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Unsafe expression not translatable".to_string()
            ))
        );
    }
}
