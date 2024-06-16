use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprRange;

use super::parse_expression;

pub fn handle_create_range(
    expr: &ExprRange,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    match expr.limits {
        // These are supported
        syn::RangeLimits::HalfOpen(_) => {}

        syn::RangeLimits::Closed(_) => {
            return Err(NotTranslatableError::Custom(
                "Closed ranges are not supported".to_string(),
            ));
        }
    }

    let mut instructions: Vec<Instruction> = vec![];
    let mut inputs: Vec<String> = vec![];

    let range_start: String = format!("RANGE_START_{}", compilation_state.get_global_uuid());
    let range_end: String = format!("RANGE_END_{}", compilation_state.get_global_uuid());

    inputs.push(range_start.clone());
    let start = match expr.start.clone() {
        Some(start) => {
            let start_instructions = parse_expression(
                &start,
                &mut compilation_state.with_assignment(Some(range_start.clone())),
            )?;

            start_instructions
        }
        // ASSUMPTION: if not set, this is 0
        None => vec![Instruction::from_compilation_state(
            "assign".to_string(),
            vec!["0".to_string()],
            &mut compilation_state.with_assignment(Some(range_start.clone())),
        )],
    };

    let end = match expr.end.clone() {
        Some(end) => {
            let end_instructions = parse_expression(
                &end,
                &mut compilation_state.with_assignment(Some(range_end.clone())),
            )?;

            inputs.push(range_end.clone());

            end_instructions
        }
        None => Err(NotTranslatableError::Custom(
            "Undefined end of range is not supported".to_string(),
        ))?,
    };

    instructions.extend(start);
    instructions.extend(end);

    inputs.insert(0, "Range".to_string());

    instructions.push(Instruction::from_compilation_state(
        "instantiate_object".to_string(),
        inputs,
        compilation_state,
    ));

    Ok(instructions)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::compilation_state::CompilationState;
    use crate::instruction::Instruction;
    use syn::{parse_quote, ExprRange};

    #[test]
    fn test_handle_create_full_range() {
        let expr: ExprRange = parse_quote! { 1..10 };

        let mut compilation_state = CompilationState::new();

        let instructions = handle_create_range(&expr, &mut compilation_state).unwrap();

        let expected_instructions = vec![
            Instruction::from_compilation_state(
                "assign".to_string(),
                vec!["1".to_string()],
                &compilation_state.with_assignment(Some("RANGE_START_0".to_string())),
            ),
            Instruction::from_compilation_state(
                "assign".to_string(),
                vec!["10".to_string()],
                &compilation_state.with_assignment(Some("RANGE_END_1".to_string())),
            ),
            Instruction::from_compilation_state(
                "instantiate_object".to_string(),
                vec![
                    "Range".to_string(),
                    "RANGE_START_0".to_string(),
                    "RANGE_END_1".to_string(),
                ],
                &compilation_state,
            ),
        ];

        assert_eq!(instructions, expected_instructions);
    }
}
