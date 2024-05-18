use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_binary_op;
use crate::translate::expression::parse_expression;

use syn::ExprBinary;

pub fn handle_binary_expression(expr_binary: &ExprBinary) -> Result<String, NotTranslatableError> {
    let left_hand_side: String = parse_expression(&expr_binary.left)?;
    let right_hand_side: String = parse_expression(&expr_binary.right)?;
    let operator: String = parse_binary_op(&expr_binary.op)?;

    Ok(format!(
        "{{ instruction: {}, input: ({}, {}), assign: IDK_YET }}",
        operator, left_hand_side, right_hand_side
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::expression::parse_expression;
    use syn::ExprBinary;

    #[test]
    fn test_binary_expression_simple_addition() {
        let parsed_expr_binary: ExprBinary = syn::parse_str("1 + 2").unwrap();
        let result = parse_expression(&syn::Expr::Binary(parsed_expr_binary));

        assert_eq!(
            result,
            Ok("{ instruction: add, input: (1, 2), assign: IDK_YET }".to_string())
        );
    }

    #[test]
    fn test_binary_expression_simple_subtraction_and_assignment() {
        let parsed_expr_binary: ExprBinary = syn::parse_str("foo -= 2").unwrap();
        let result = parse_expression(&syn::Expr::Binary(parsed_expr_binary));

        assert_eq!(
            result,
            Ok(
                "{ instruction: subtract_and_assign, input: (foo, 2), assign: IDK_YET }"
                    .to_string()
            )
        );
    }
}
