use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprMatch;

pub fn handle_match_expression(_: &ExprMatch) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Match expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprMatch;

    #[test]
    fn test_match_expression() {
        let parsed_expr_match: ExprMatch = syn::parse_str("match 1 { _ => 0 }").unwrap();
        let result = parse_expression(&syn::Expr::Match(parsed_expr_match), None, 0);

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Match expression not translatable".to_string()
            ))
        );
    }
}
