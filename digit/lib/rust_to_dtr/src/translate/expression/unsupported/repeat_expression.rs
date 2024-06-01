use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprRepeat;

pub fn handle_repeat_expression(_: &ExprRepeat) -> Result<Vec<Instruction>, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Repeat expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprRepeat;

    #[test]
    fn test_repeat_expression() {
        let parsed_expr_repeat: ExprRepeat = syn::parse_str("[1; 3]").unwrap();
        let result = parse_expression(&syn::Expr::Repeat(parsed_expr_repeat), None, 0);

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Repeat expression not translatable".to_string()
            ))
        );
    }
}
