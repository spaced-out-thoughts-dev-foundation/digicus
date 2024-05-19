use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprTuple;

pub fn handle_tuple_expression(_: &ExprTuple) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Tuple expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprTuple;

    #[test]
    fn test_tuple_expression() {
        let parsed_expr_tuple: ExprTuple = syn::parse_str("(1, 2, 3)").unwrap();
        let result = parse_expression(&syn::Expr::Tuple(parsed_expr_tuple), None);

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Tuple expression not translatable".to_string()
            ))
        );
    }
}
