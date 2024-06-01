use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprYield;

pub fn handle_yield_expression(_: &ExprYield) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Yield expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprYield;

    #[test]
    fn test_yield_expression() {
        let parsed_expr_yield: ExprYield = syn::parse_str("yield 1").unwrap();
        let result = parse_expression(&syn::Expr::Yield(parsed_expr_yield), None, 0);

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Yield expression not translatable".to_string()
            ))
        );
    }
}
