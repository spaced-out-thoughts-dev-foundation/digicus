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
    let thing_being_index_name = format!("THING_BEING_INDEXED_{}", global_uuid);

    let original_assignment = compilation_state.next_assignment.clone();
    let mut instructions = parse_expression(
        &expr.expr,
        &mut compilation_state.with_assignment(Some(thing_being_index_name.clone())),
    )?;

    global_uuid = compilation_state.get_global_uuid();
    let index_name = format!("INDEX_NAME_{}", global_uuid);

    let index_instructions = parse_expression(
        &expr.index,
        &mut compilation_state.with_assignment(Some(index_name.clone())),
    )?;
    compilation_state.with_assignment(original_assignment);

    instructions.extend(index_instructions);

    instructions.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "evaluate".to_string(),
        vec!["index".to_string(), thing_being_index_name, index_name],
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or_default(),
        compilation_state.scope(),
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
                    1,
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "THING_BEING_INDEXED_0".to_string(),
                    compilation_state.scope()
                ),
                Instruction::new(
                    3,
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "INDEX_NAME_2".to_string(),
                    compilation_state.scope()
                ),
                Instruction::new(
                    4,
                    "evaluate".to_string(),
                    vec![
                        "index".to_string(),
                        "THING_BEING_INDEXED_0".to_string(),
                        "INDEX_NAME_2".to_string()
                    ],
                    "".to_string(),
                    compilation_state.scope()
                )
            ]
        );
    }

    #[test]
    fn test_handle_index_expression_with_nested_index() {
        let expr: ExprIndex = syn::parse_str("a[1][2]").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_index_expression(
            &expr,
            compilation_state.with_assignment(Some("final_indexed_thing".to_string())),
        )
        .unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    2,
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "THING_BEING_INDEXED_1".to_string(),
                    compilation_state.scope()
                ),
                Instruction::new(
                    4,
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "INDEX_NAME_3".to_string(),
                    compilation_state.scope()
                ),
                Instruction::new(
                    5,
                    "evaluate".to_string(),
                    vec![
                        "index".to_string(),
                        "THING_BEING_INDEXED_1".to_string(),
                        "INDEX_NAME_3".to_string()
                    ],
                    "THING_BEING_INDEXED_0".to_string(),
                    compilation_state.scope()
                ),
                Instruction::new(
                    7,
                    "assign".to_string(),
                    vec!["2".to_string()],
                    "INDEX_NAME_6".to_string(),
                    compilation_state.scope()
                ),
                Instruction::new(
                    8,
                    "evaluate".to_string(),
                    vec![
                        "index".to_string(),
                        "THING_BEING_INDEXED_0".to_string(),
                        "INDEX_NAME_6".to_string()
                    ],
                    "final_indexed_thing".to_string(),
                    compilation_state.scope()
                )
            ]
        );
    }
}
