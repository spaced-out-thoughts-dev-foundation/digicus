use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::block::handle_block;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprIf;

pub fn handle_if_expression(
    expr: &ExprIf,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let global_uuid = compilation_state.get_global_uuid();
    let conditional_jump_assignment_label = format!("CONDITIONAL_JUMP_ASSIGNMENT_{}", global_uuid);

    // let mut instructions_to_return: Vec<Instruction> = vec![];

    let original_assignment = compilation_state.next_assignment.clone();
    let mut condition_instructions: Vec<Instruction> = parse_expression(
        &expr.cond,
        &mut compilation_state.with_assignment(Some(conditional_jump_assignment_label.to_string())),
    )?;
    compilation_state.with_assignment(original_assignment);

    let mut prev_scope = compilation_state.scope();
    compilation_state.enter_new_scope(false);
    let mut scope_snapshot = compilation_state.copy_out_current_scope_stack();

    let conditional_jump_instruction = Instruction::new(
        compilation_state.get_global_uuid(),
        "jump".to_string(),
        vec![
            conditional_jump_assignment_label.to_string(),
            (compilation_state.scope()).to_string(),
        ],
        "".to_string(),
        prev_scope,
    );

    condition_instructions.push(conditional_jump_instruction);

    let mut then_branch = handle_block(&expr.then_branch, compilation_state);

    compilation_state.set_scope_stack(scope_snapshot);

    println!("\n[DEBUG] setting back the if scope to {:?}", prev_scope);

    while compilation_state.scope() != prev_scope {
        println!("\n[DEBUG] exiting scope {:?}", compilation_state.scope());
        let temp_prev_scope = compilation_state.scope();
        compilation_state.exit_scope();
        then_branch.push(Instruction::new(
            compilation_state.get_global_uuid(),
            "jump".to_string(),
            vec![(compilation_state.scope()).to_string()],
            "".to_string(),
            temp_prev_scope,
        ));
    }

    let else_branch = match &expr.else_branch {
        Some(else_branch) => {
            prev_scope = compilation_state.scope();
            compilation_state.enter_new_scope(false);
            condition_instructions.push(Instruction::new(
                compilation_state.get_global_uuid(),
                "jump".to_string(),
                vec![(compilation_state.scope()).to_string()],
                "".to_string(),
                prev_scope,
            ));

            scope_snapshot = compilation_state.copy_out_current_scope_stack();

            let mut else_branch_instructions =
                parse_expression(&else_branch.1, &mut compilation_state.clone())?;

            while compilation_state.scope() != prev_scope {
                let temp_prev_scope = compilation_state.scope();
                compilation_state.exit_scope();
                else_branch_instructions.push(Instruction::new(
                    compilation_state.get_global_uuid(),
                    "jump".to_string(),
                    vec![(compilation_state.scope()).to_string()],
                    "".to_string(),
                    temp_prev_scope,
                ));
            }

            else_branch_instructions
        }
        None => vec![],
    };

    condition_instructions.extend(then_branch);
    condition_instructions.extend(else_branch);

    Ok(condition_instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;
    use syn::{self, parse_quote};

    #[test]
    fn test_handle_if_true_expression() {
        let expr_if: ExprIf = syn::parse_quote!(if true {});
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_if_expression(&expr_if, &mut compilation_state).unwrap();

        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["true".to_string(),],
                    "CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(),
                    0,
                ),
                Instruction::new(
                    3,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(), "2".to_string()],
                    "".to_string(),
                    0,
                ),
                Instruction::new(
                    4,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    2,
                ),
            ]
        );
    }

    #[test]
    fn test_handle_if_true_nested_expression() {
        let expr_if: ExprIf = syn::parse_quote!(if true {
            if true {
                log!("nested_if");
            }

            log!("after_nested_if");
        });
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_if_expression(&expr_if, &mut compilation_state).unwrap();

        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["true".to_string(),],
                    "CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(),
                    0,
                ),
                Instruction::new(
                    3,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(), "2".to_string()],
                    "".to_string(),
                    0,
                ),
                Instruction::new(
                    5,
                    "assign".to_string(),
                    vec!["true".to_string()],
                    "CONDITIONAL_JUMP_ASSIGNMENT_4".to_string(),
                    2,
                ),
                Instruction::new(
                    7,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_ASSIGNMENT_4".to_string(), "6".to_string()],
                    "".to_string(),
                    2,
                ),
                Instruction::new(
                    8,
                    "print".to_string(),
                    vec!["\"nested_if\"".to_string()],
                    "".to_string(),
                    6,
                ),
                Instruction::new(
                    9,
                    "jump".to_string(),
                    vec!["2".to_string()],
                    "".to_string(),
                    6,
                ),
                Instruction::new(
                    10,
                    "print".to_string(),
                    vec!["\"after_nested_if\"".to_string()],
                    "".to_string(),
                    2,
                ),
                Instruction::new(
                    11,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    2,
                ),
            ]
        );
    }

    #[test]
    fn test_handle_if_condition_expression() {
        let expr_if: ExprIf = syn::parse_str("if 10 < 11 { }").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_if_expression(&expr_if, &mut compilation_state).unwrap();

        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    3,
                    "assign".to_string(),
                    vec!["10".to_string()],
                    "BINARY_EXPRESSION_LEFT_1".to_string(),
                    0,
                ),
                Instruction::new(
                    4,
                    "assign".to_string(),
                    vec!["11".to_string()],
                    "BINARY_EXPRESSION_RIGHT_2".to_string(),
                    0,
                ),
                Instruction::new(
                    5,
                    "evaluate".to_string(),
                    vec![
                        "less_than".to_string(),
                        "BINARY_EXPRESSION_LEFT_1".to_string(),
                        "BINARY_EXPRESSION_RIGHT_2".to_string()
                    ],
                    "CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(),
                    0,
                ),
                Instruction::new(
                    7,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(), "6".to_string()],
                    "".to_string(),
                    0,
                ),
                Instruction::new(
                    8,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    6,
                ),
            ]
        );
    }

    #[test]
    fn test_handle_if_else_expression() {
        let expr_if: ExprIf = parse_quote!(if true { log!("if") } else { log!("else") });
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_if_expression(&expr_if, &mut compilation_state).unwrap();

        instructions.iter().for_each(|instruction| {
            println!("{:?}", instruction);
        });

        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["true".to_string()],
                    "CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(),
                    0,
                ),
                Instruction::new(
                    3,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(), "2".to_string()],
                    "".to_string(),
                    0,
                ),
                Instruction::new(
                    7,
                    "jump".to_string(),
                    vec!["6".to_string()],
                    "".to_string(),
                    0,
                ),
                Instruction::new(
                    4,
                    "print".to_string(),
                    vec!["\"if\"".to_string()],
                    "".to_string(),
                    2,
                ),
                Instruction::new(
                    5,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    2,
                ),
                Instruction::new(
                    8,
                    "print".to_string(),
                    vec!["\"else\"".to_string()],
                    "".to_string(),
                    6,
                ),
                Instruction::new(
                    9,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    6,
                ),
            ]
        );
    }

    #[test]
    fn test_handle_if_elseif_else_expression() {
        let expr_if: ExprIf = syn::parse_str("if 10 < 11 { log!(\"if\") } else if 10 == 11 { log!(\"else if\") } else { log!(\"else\") }").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_if_expression(&expr_if, &mut compilation_state).unwrap();

        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    3,
                    "assign".to_string(),
                    vec!["10".to_string()],
                    "BINARY_EXPRESSION_LEFT_1".to_string(),
                    0,
                ),
                Instruction::new(
                    4,
                    "assign".to_string(),
                    vec!["11".to_string()],
                    "BINARY_EXPRESSION_RIGHT_2".to_string(),
                    0,
                ),
                Instruction::new(
                    5,
                    "evaluate".to_string(),
                    vec![
                        "less_than".to_string(),
                        "BINARY_EXPRESSION_LEFT_1".to_string(),
                        "BINARY_EXPRESSION_RIGHT_2".to_string()
                    ],
                    "CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(),
                    0,
                ),
                Instruction::new(
                    7,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(), "6".to_string()],
                    "".to_string(),
                    0,
                ),
                Instruction::new(
                    11,
                    "jump".to_string(),
                    vec!["10".to_string()],
                    "".to_string(),
                    0,
                ),
                Instruction::new(
                    8,
                    "print".to_string(),
                    vec!["\"if\"".to_string()],
                    "".to_string(),
                    6,
                ),
                Instruction::new(
                    9,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    6,
                ),
                Instruction::new(
                    15,
                    "assign".to_string(),
                    vec!["10".to_string()],
                    "BINARY_EXPRESSION_LEFT_13".to_string(),
                    10,
                ),
                Instruction::new(
                    16,
                    "assign".to_string(),
                    vec!["11".to_string()],
                    "BINARY_EXPRESSION_RIGHT_14".to_string(),
                    10,
                ),
                Instruction::new(
                    17,
                    "evaluate".to_string(),
                    vec![
                        "equal_to".to_string(),
                        "BINARY_EXPRESSION_LEFT_13".to_string(),
                        "BINARY_EXPRESSION_RIGHT_14".to_string()
                    ],
                    "CONDITIONAL_JUMP_ASSIGNMENT_12".to_string(),
                    10,
                ),
                Instruction::new(
                    19,
                    "jump".to_string(),
                    vec![
                        "CONDITIONAL_JUMP_ASSIGNMENT_12".to_string(),
                        "18".to_string()
                    ],
                    "".to_string(),
                    10,
                ),
                Instruction::new(
                    23,
                    "jump".to_string(),
                    vec!["22".to_string()],
                    "".to_string(),
                    10,
                ),
                Instruction::new(
                    20,
                    "print".to_string(),
                    vec!["\"else if\"".to_string(),],
                    "".to_string(),
                    18,
                ),
                Instruction::new(
                    21,
                    "jump".to_string(),
                    vec!["10".to_string()],
                    "".to_string(),
                    18,
                ),
                Instruction::new(
                    24,
                    "print".to_string(),
                    vec!["\"else\"".to_string(),],
                    "".to_string(),
                    22,
                ),
                Instruction::new(
                    25,
                    "jump".to_string(),
                    vec!["10".to_string()],
                    "".to_string(),
                    22,
                ),
                Instruction::new(
                    26,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    10,
                ),
            ]
        );
    }

    #[test]
    fn test_handle_if_let_expression() {
        let expr_if: ExprIf = syn::parse_str("if let Some(x) = Some(10) { log!(x) }").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();
        let instructions = handle_if_expression(&expr_if, &mut compilation_state).unwrap();

        instructions.clone().iter().for_each(|instruction| {
            println!("{:?}", instruction);
        });

        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    4,
                    "assign".to_string(),
                    vec!["Some".to_string()],
                    "CALL_EXPRESSION_FUNCTION_3".to_string(),
                    0,
                ),
                Instruction::new(
                    2,
                    "assign".to_string(),
                    vec!["10".to_string()],
                    "CALL_EXPRESSION_ARG_1".to_string(),
                    0,
                ),
                Instruction::new(
                    5,
                    "evaluate".to_string(),
                    vec![
                        "CALL_EXPRESSION_FUNCTION_3".to_string(),
                        "CALL_EXPRESSION_ARG_1".to_string()
                    ],
                    "INPUT_VALUE_NAME_FOR_LET_1".to_string(),
                    0,
                ),
                Instruction::new(
                    6,
                    "try_assign".to_string(),
                    vec![
                        "INPUT_VALUE_NAME_FOR_LET_1".to_string(),
                        "Some(x)".to_string()
                    ],
                    "CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(),
                    0,
                ),
                Instruction::new(
                    8,
                    "jump".to_string(),
                    vec!["CONDITIONAL_JUMP_ASSIGNMENT_0".to_string(), "7".to_string()],
                    "".to_string(),
                    0,
                ),
                Instruction::new(
                    9,
                    "print".to_string(),
                    vec!["x".to_string()],
                    "".to_string(),
                    7,
                ),
                Instruction::new(
                    10,
                    "jump".to_string(),
                    vec!["0".to_string()],
                    "".to_string(),
                    7,
                ),
            ]
        );
    }
}
