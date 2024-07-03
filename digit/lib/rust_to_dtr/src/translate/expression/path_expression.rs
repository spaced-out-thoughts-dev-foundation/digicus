use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::type_name::parse_path;
use std::vec;

pub fn handle_path_expression(
    path_expr: &syn::ExprPath,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let path_string_value = parse_path(&path_expr.path.clone());

    let result_instruction: Vec<Instruction> = vec![Instruction::new(
        compilation_state.get_global_uuid(),
        "assign".to_string(),
        vec![path_string_value],
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or_default(),
        compilation_state.scope(),
    )];

    Ok(result_instruction)
}

#[cfg(test)]
mod tests {
    use crate::common::compilation_state::CompilationState;
    use crate::instruction::Instruction;
    use crate::translate::expression::path_expression::handle_path_expression;
    use syn::{parse_quote, ExprPath};

    #[test]
    fn test_handle_path_expression() {
        let mut compilation_state = CompilationState::new();
        let expr: ExprPath = parse_quote! { Struct };
        let instructions = handle_path_expression(
            &expr,
            &mut compilation_state.with_assignment(Some("SomeAssignment".to_string())),
        )
        .unwrap();
        assert_eq!(
            instructions,
            vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["Struct".to_string()],
                "SomeAssignment".to_string(),
                0
            ),]
        );
    }
}
