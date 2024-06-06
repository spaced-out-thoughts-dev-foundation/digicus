use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use syn::ExprParen;

pub fn handle_paren_expression(
    expr_paren: &ExprParen,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    Ok(parse_expression(&expr_paren.expr, compilation_state)?)
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::{
        common::compilation_state::CompilationState,
        translate::expression::paren_expression::handle_paren_expression,
    };
    use syn::{parse_quote, ExprParen};

    #[test]
    fn test_handle_paren_expression() {
        let compilation_state = CompilationState::new();
        let expr: ExprParen = parse_quote! { (Struct) };
        let instructions = handle_paren_expression(
            &expr,
            &mut compilation_state.with_assignment(Some("SomeAssignment".to_string())),
        )
        .unwrap();
        assert_eq!(
            instructions,
            vec![Instruction::new(
                "assign".to_string(),
                vec!["Struct".to_string()],
                "SomeAssignment".to_string(),
                0
            ),]
        );
    }
}
