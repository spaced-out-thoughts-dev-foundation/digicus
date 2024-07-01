use crate::common::compilation_state::CompilationState;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::pattern::handle_pattern;
use syn::ExprMatch;

use super::parse_expression;

pub fn handle_match_expression(
    expr: &ExprMatch,
    compilation_state: &mut CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let thing_to_compare_against: String = "Thing_to_compare_against".to_string();
    let mut thing_to_compare_against_instructions = parse_expression(
        &*expr.expr,
        &mut compilation_state.with_assignment(Some(thing_to_compare_against.clone())),
    )?;

    let mut match_conditional_evaluation_instructions: Vec<Instruction> = vec![];
    let mut match_body_instructions: Vec<Instruction> = vec![];

    expr.arms.iter().for_each(|arm| {
        let arm_path: String = handle_pattern(arm.clone().pat.clone()).unwrap();
        let conditional_jump_check = format!(
            "CONDITIONAL_JUMP_CHECK_{}",
            compilation_state.get_global_uuid()
        );

        match_conditional_evaluation_instructions.push(Instruction::new(
            compilation_state.get_global_uuid(),
            format!("evaluate"),
            vec![
                "equal_to".to_string(),
                thing_to_compare_against.clone(),
                arm_path.clone(),
            ],
            conditional_jump_check.clone(),
            compilation_state.scope(),
        ));

        let mut prev_scope = compilation_state.scope();
        compilation_state.enter_new_scope();

        match_conditional_evaluation_instructions.push(Instruction::new(
            compilation_state.get_global_uuid(),
            format!("jump"),
            vec![
                conditional_jump_check.clone(),
                compilation_state.scope().to_string(),
            ],
            "".to_string(),
            prev_scope,
        ));

        let arm_instructions =
            parse_expression(&*arm.clone().body, &mut compilation_state.clone()).unwrap();
        match_body_instructions.extend(arm_instructions);

        prev_scope = compilation_state.scope();
        compilation_state.exit_scope();

        match_body_instructions.push(Instruction::new(
            compilation_state.get_global_uuid(),
            format!("jump"),
            vec![compilation_state.scope().to_string()],
            "".to_string(),
            prev_scope,
        ));
    });

    thing_to_compare_against_instructions.extend(match_conditional_evaluation_instructions);
    thing_to_compare_against_instructions.extend(match_body_instructions);

    Ok(thing_to_compare_against_instructions)
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::{
        common::compilation_state::CompilationState,
        translate::expression::match_expression::handle_match_expression,
    };
    use syn::{parse_quote, ExprMatch};

    #[test]
    fn test_handle_match_expression() {
        let mut compilation_state = CompilationState::new();
        let expr: ExprMatch = parse_quote! { match instance_of_struct {
            Struct::Variant1 => log!("Variant1"),
            Struct::Variant2 => log!("Variant2"),
        } };
        let instructions = handle_match_expression(&expr, &mut compilation_state).unwrap();

        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    0,
                    "assign".to_string(),
                    vec!["instance_of_struct".to_string()],
                    "Thing_to_compare_against".to_string(),
                    0
                ),
                Instruction::new(
                    2,
                    "evaluate".to_string(),
                    vec![
                        "equal_to".to_string(),
                        "Thing_to_compare_against".to_string(),
                        "Struct::Variant1".to_string()
                    ],
                    "CONDITIONAL_JUMP_CHECK_1".to_string(),
                    0
                ),
                Instruction::new(
                    4,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_CHECK_1".to_string(), "3".to_string()],
                    "".to_string(),
                    0
                ),
                Instruction::new(
                    8,
                    "evaluate".to_string(),
                    vec![
                        "equal_to".to_string(),
                        "Thing_to_compare_against".to_string(),
                        "Struct::Variant2".to_string()
                    ],
                    "CONDITIONAL_JUMP_CHECK_7".to_string(),
                    0
                ),
                Instruction::new(
                    10,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_CHECK_7".to_string(), "9".to_string()],
                    "".to_string(),
                    0
                ),
                Instruction::new(
                    5,
                    "print".to_string(),
                    vec!["\"Variant1\"".to_string()],
                    "".to_string(),
                    3
                ),
                Instruction::new(
                    6,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    3
                ),
                Instruction::new(
                    11,
                    "print".to_string(),
                    vec!["\"Variant2\"".to_string()],
                    "".to_string(),
                    9
                ),
                Instruction::new(
                    12,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    9
                ),
            ]
        );
    }
}
