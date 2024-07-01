use crate::common::compilation_state::CompilationState;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;
use syn::ExprMatch;

pub fn handle_match_expression(
    expr: &ExprMatch,
    compilation_state: &mut CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    // let thing_to_compare_against: String = "Thing_to_compare_against".to_string();
    // let mut thing_to_compare_against_instructions = parse_expression(
    //     &*expr.expr,
    //     &mut compilation_state.with_assignment(Some(thing_to_compare_against.clone())),
    // )?;

    // let mut index = 1;
    // expr.arms.iter().for_each(|arm| {
    //     let arm_path: String = handle_pattern(arm.clone().pat.clone()).unwrap();
    //     let new_scope = compilation_state.scope.clone() + 100 * index;

    //     thing_to_compare_against_instructions.push(Instruction::new(
    //         compilation_state.get_global_uuid(),
    //         format!("evaluate"),
    //         vec![
    //             "equal_to".to_string(),
    //             thing_to_compare_against.clone(),
    //             arm_path.clone(),
    //         ],
    //         format!("CONDITIONAL_JUMP_CHECK_{}", new_scope),
    //         compilation_state.scope(),
    //     ));

    //     thing_to_compare_against_instructions.push(Instruction::new(
    //         compilation_state.get_global_uuid(),
    //         format!("jump"),
    //         vec![
    //             format!("CONDITIONAL_JUMP_CHECK_{}", new_scope),
    //             (new_scope.to_string()),
    //         ],
    //         "".to_string(),
    //         compilation_state.scope(),
    //     ));

    //     let arm_instructions = parse_expression(
    //         &*arm.clone().body,
    //         &mut compilation_state.with_scope_jump(100 * index),
    //     )
    //     .unwrap();
    //     thing_to_compare_against_instructions.extend(arm_instructions);

    //     index += 1;
    // });

    Ok(vec![])
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
        let expr: ExprMatch = parse_quote! { match Struct {
            Struct::Variant1 => Struct::Variant1,
            Struct::Variant2 => Struct::Variant2,
        } };
        let instructions = handle_match_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    0,
                    "assign".to_string(),
                    vec!["Struct".to_string()],
                    "Thing_to_compare_against".to_string(),
                    0
                ),
                Instruction::new(
                    0,
                    "evaluate".to_string(),
                    vec![
                        "equal_to".to_string(),
                        "Thing_to_compare_against".to_string(),
                        "Struct::Variant1".to_string()
                    ],
                    "CONDITIONAL_JUMP_CHECK_100".to_string(),
                    0
                ),
                Instruction::new(
                    0,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_CHECK_100".to_string(), "100".to_string()],
                    "".to_string(),
                    0
                ),
                Instruction::new(
                    0,
                    "assign".to_string(),
                    vec!["Struct::Variant1".to_string()],
                    "".to_string(),
                    100
                ),
                Instruction::new(
                    0,
                    "evaluate".to_string(),
                    vec![
                        "equal_to".to_string(),
                        "Thing_to_compare_against".to_string(),
                        "Struct::Variant2".to_string()
                    ],
                    "CONDITIONAL_JUMP_CHECK_200".to_string(),
                    0
                ),
                Instruction::new(
                    0,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_CHECK_200".to_string(), "200".to_string()],
                    "".to_string(),
                    0
                ),
                Instruction::new(
                    0,
                    "assign".to_string(),
                    vec!["Struct::Variant2".to_string()],
                    "".to_string(),
                    200
                )
            ]
        );
    }
}
