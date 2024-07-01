use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprRepeat;

pub fn handle_repeat_expression(
    expr: &ExprRepeat,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut instructions: Vec<Instruction> = vec![];
    let repeat_expression_name = format!(
        "REPEAT_EXPRESSION_VALUE_{}",
        compilation_state.get_global_uuid()
    );
    let repeat_expression_length = format!(
        "REPEAT_EXPRESSION_LENGTH_{}",
        compilation_state.get_global_uuid()
    );

    instructions.extend(parse_expression(
        &expr.expr,
        &mut compilation_state.with_assignment(Some(repeat_expression_name.clone())),
    )?);

    instructions.extend(parse_expression(
        &expr.len,
        &mut compilation_state.with_assignment(Some(repeat_expression_length.clone())),
    )?);

    instructions.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "evaluate".to_string(),
        vec![
            "repeat".to_string(),
            repeat_expression_name,
            repeat_expression_length,
        ],
        compilation_state.next_assignment.clone().unwrap(),
        0,
    ));

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use crate::common::compilation_state::CompilationState;
    use crate::instruction::Instruction;
    use crate::translate::expression::repeat_expression::handle_repeat_expression;

    #[test]
    fn test_handle_repeat_expression() {
        let expr: syn::ExprRepeat = parse_quote! { [hello; 3] };
        let compilation_state = CompilationState::new();
        let instructions = handle_repeat_expression(
            &expr,
            &mut compilation_state.with_assignment(Some("foobar".to_string())),
        )
        .unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    2,
                    "assign".to_string(),
                    vec!["hello".to_string()],
                    "REPEAT_EXPRESSION_VALUE_0".to_string(),
                    0,
                ),
                Instruction::new(
                    3,
                    "assign".to_string(),
                    vec!["3".to_string()],
                    "REPEAT_EXPRESSION_LENGTH_1".to_string(),
                    0,
                ),
                Instruction::new(
                    4,
                    "evaluate".to_string(),
                    vec![
                        "repeat".to_string(),
                        "REPEAT_EXPRESSION_VALUE_0".to_string(),
                        "REPEAT_EXPRESSION_LENGTH_1".to_string()
                    ],
                    "foobar".to_string(),
                    0,
                ),
            ]
        );
    }
}
