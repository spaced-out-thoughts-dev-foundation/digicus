use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprField;

pub fn handle_field_expression(
    expr: &ExprField,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let member = match &expr.member {
        syn::Member::Named(ident) => ident.to_string(),
        syn::Member::Unnamed(index) => index.index.to_string(),
    };

    let field_base_name: String = format!("FIELD_BASE_{}", compilation_state.get_global_uuid());

    let original_assignment = compilation_state.next_assignment.clone();
    let mut base = expression::parse_expression(
        &*expr.base,
        &mut compilation_state.with_assignment(Some(field_base_name.clone())),
    )?;
    compilation_state.with_assignment(original_assignment);

    base.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "field".to_string(),
        vec![field_base_name.clone(), member],
        compilation_state.next_assignment.clone().unwrap_or(format!(
            "FIELD_RESULT_{}",
            compilation_state.get_global_uuid()
        )),
        compilation_state.scope(),
    ));

    Ok(base)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_handle_field_expression() {
        let expr: ExprField = syn::parse_str("a.b").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_field_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "FIELD_BASE_0".to_string(),
                    0,
                ),
                Instruction::new(
                    2,
                    "field".to_string(),
                    vec!["FIELD_BASE_0".to_string(), "b".to_string()],
                    "FIELD_RESULT_3".to_string(),
                    0,
                ),
            ]
        );
    }

    #[test]
    fn test_handle_field_expression_index() {
        let expr: ExprField = syn::parse_str("a.0").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_field_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "FIELD_BASE_0".to_string(),
                    0,
                ),
                Instruction::new(
                    2,
                    "field".to_string(),
                    vec!["FIELD_BASE_0".to_string(), "0".to_string()],
                    "FIELD_RESULT_3".to_string(),
                    0,
                ),
            ]
        );
    }
}
