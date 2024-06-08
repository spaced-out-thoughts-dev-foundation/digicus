use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprIndex;

use super::parse_expression;

pub fn handle_index_expression(
    expr: &ExprIndex,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut global_uuid = compilation_state.get_global_uuid();
    let thing_being_index_name = format!("thing_being_indexed_{}", global_uuid);

    let mut instructions = parse_expression(
        &expr.expr,
        &mut compilation_state.with_assignment(Some(thing_being_index_name.clone())),
    )?;

    global_uuid = compilation_state.get_global_uuid();
    let index_name = format!("index_name_{}", global_uuid);

    let index_instructions = parse_expression(
        &expr.index,
        &mut compilation_state.with_assignment(Some(index_name.clone())),
    )?;

    instructions.extend(index_instructions);

    instructions.push(Instruction::new(
        "index".to_string(),
        vec![thing_being_index_name, index_name],
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or_default(),
        compilation_state.scope,
    ));

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_handle_index_expression() {
        let expr: ExprIndex = syn::parse_str("a[1]").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_index_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "thing_being_indexed_0".to_string(),
                    compilation_state.scope
                ),
                Instruction::new(
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "index_name_1".to_string(),
                    compilation_state.scope
                ),
                Instruction::new(
                    "index".to_string(),
                    vec![
                        "thing_being_indexed_0".to_string(),
                        "index_name_1".to_string()
                    ],
                    "".to_string(),
                    compilation_state.scope
                )
            ]
        );
    }

    #[test]
    fn test_handle_index_expression_with_nested_index() {
        let expr: ExprIndex = syn::parse_str("a[1][2]").unwrap();
        let compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_index_expression(
            &expr,
            &mut compilation_state.with_assignment(Some("final_indexed_thing".to_string())),
        )
        .unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "thing_being_indexed_1".to_string(),
                    compilation_state.scope
                ),
                Instruction::new(
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "index_name_2".to_string(),
                    compilation_state.scope
                ),
                Instruction::new(
                    "index".to_string(),
                    vec![
                        "thing_being_indexed_1".to_string(),
                        "index_name_2".to_string()
                    ],
                    "thing_being_indexed_0".to_string(),
                    compilation_state.scope
                ),
                Instruction::new(
                    "assign".to_string(),
                    vec!["2".to_string()],
                    "index_name_3".to_string(),
                    compilation_state.scope
                ),
                Instruction::new(
                    "index".to_string(),
                    vec![
                        "thing_being_indexed_0".to_string(),
                        "index_name_3".to_string()
                    ],
                    "final_indexed_thing".to_string(),
                    compilation_state.scope
                )
            ]
        );
    }
}
