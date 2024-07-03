use crate::common::compilation_state::{self, CompilationState};
use crate::instruction::Instruction;
use crate::translate::block::handle_block;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprForLoop;

pub fn handle_for_loop_expression(
    expr: &ExprForLoop,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut instructions = vec![];

    let iterator_variable: String = format!("ITERATOR_{}", compilation_state.get_global_uuid());
    let iterator_temp_variable: String = handle_pattern(*expr.pat.clone()).unwrap();
    let check_condition_assignment: String = format!(
        "CHECK_CONDITION_ASSIGNMENT_{}",
        compilation_state.get_global_uuid()
    );

    instructions.extend(get_initial_value(
        expr.clone(),
        compilation_state,
        iterator_variable.clone(),
    ));

    instructions.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "evaluate".to_string(),
        vec!["start".to_string(), iterator_variable.clone()],
        iterator_temp_variable.clone(),
        compilation_state.scope(),
    ));

    let check_condition_instruction_id = compilation_state.get_global_uuid();
    instructions.extend(get_check_condition(
        check_condition_instruction_id,
        iterator_variable.clone(),
        iterator_temp_variable.clone(),
        check_condition_assignment.clone(),
        compilation_state.scope(),
    ));

    let prev_scope = compilation_state.scope();
    compilation_state.enter_new_scope();
    let body_scope = compilation_state.scope();

    instructions.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "jump".to_string(),
        vec![check_condition_assignment.clone(), body_scope.to_string()],
        "".to_string(),
        prev_scope,
    ));

    compilation_state.should_output = false;
    instructions.extend(get_execute_block(expr.clone(), compilation_state));
    instructions.extend(get_increment_value(
        compilation_state.get_global_uuid(),
        iterator_temp_variable.clone(),
        body_scope,
    ));

    instructions.extend(get_back_to_top(
        check_condition_instruction_id,
        compilation_state,
    ));

    compilation_state.exit_scope();

    Ok(instructions)
}

fn get_initial_value(
    for_loop_expr: ExprForLoop,
    compilation_state: &mut CompilationState,
    iterator_variable: String,
) -> Vec<Instruction> {
    let original_assignment = compilation_state.next_assignment.clone();

    let result = parse_expression(
        &*for_loop_expr.expr.clone(),
        compilation_state.with_assignment(Some(iterator_variable.clone())),
    )
    .unwrap();

    compilation_state.update_next_assignment(original_assignment);

    result
}

fn get_check_condition(
    id: u128,
    iterator_variable: String,
    iterator_temp_variable: String,
    check_condition_assignment: String,
    scope: u128,
) -> Vec<Instruction> {
    vec![Instruction::new(
        id,
        "end_of_iteration_check".to_string(),
        vec![iterator_temp_variable.clone(), iterator_variable.clone()],
        check_condition_assignment.clone(),
        scope,
    )]
}

fn get_increment_value(id: u128, iterator_variable: String, scope: u128) -> Vec<Instruction> {
    vec![Instruction::new(
        id,
        "increment".to_string(),
        vec![iterator_variable],
        "".to_string(),
        scope,
    )]
}

fn get_execute_block(
    for_loop_expr: ExprForLoop,
    compilation_state: &mut CompilationState,
) -> Vec<Instruction> {
    handle_block(&for_loop_expr.body.clone(), compilation_state)
}

fn get_back_to_top(
    return_to_condition_id: u128,
    compilation_state: &mut CompilationState,
) -> Vec<Instruction> {
    vec![Instruction::new(
        compilation_state.get_global_uuid(),
        "goto".to_string(),
        vec![return_to_condition_id.to_string()],
        "".to_string(),
        compilation_state.scope(),
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;
    use syn::parse_quote;

    #[test]
    fn test_handle_for_loop_expression() {
        let expr = parse_quote! {
            for x in 0..10 {
                log!("{}", x);
            }
        };
        let mut compilation_state = CompilationState::new();

        let result = handle_for_loop_expression(&expr, &mut compilation_state);

        assert_eq!(
            result.unwrap(),
            vec![
                Instruction::new(
                    4,
                    "assign".to_string(),
                    vec!["0".to_string()],
                    "RANGE_START_2".to_string(),
                    0
                ),
                Instruction::new(
                    5,
                    "assign".to_string(),
                    vec!["10".to_string()],
                    "RANGE_END_3".to_string(),
                    0
                ),
                Instruction::new(
                    6,
                    "instantiate_object".to_string(),
                    vec![
                        "Range".to_string(),
                        "RANGE_START_2".to_string(),
                        "RANGE_END_3".to_string()
                    ],
                    "ITERATOR_0".to_string(),
                    0
                ),
                Instruction::new(
                    7,
                    "evaluate".to_string(),
                    vec!["start".to_string(), "ITERATOR_0".to_string()],
                    "x".to_string(),
                    0
                ),
                Instruction::new(
                    8,
                    "end_of_iteration_check".to_string(),
                    vec!["x".to_string(), "ITERATOR_0".to_string()],
                    "CHECK_CONDITION_ASSIGNMENT_1".to_string(),
                    0
                ),
                Instruction::new(
                    10,
                    "jump".to_string(),
                    vec!["CHECK_CONDITION_ASSIGNMENT_1".to_string(), "9".to_string()],
                    "".to_string(),
                    0
                ),
                Instruction::new(
                    11,
                    "print".to_string(),
                    vec!["\"{}\"".to_string(), "x".to_string()],
                    "".to_string(),
                    9
                ),
                Instruction::new(
                    12,
                    "increment".to_string(),
                    vec!["x".to_string()],
                    "".to_string(),
                    9
                ),
                Instruction::new(
                    13,
                    "goto".to_string(),
                    vec!["8".to_string()],
                    "".to_string(),
                    9
                ),
            ]
        );
    }

    #[test]
    fn test_handle_for_loop_non_range_iterator_expression() {
        let expr = parse_quote! {
            for x in xs.iter() {
                log!("{}", x);
            }
        };
        let mut compilation_state = CompilationState::new();

        let result = handle_for_loop_expression(&expr, &mut compilation_state);

        assert_eq!(
            result.unwrap(),
            vec![
                Instruction::new(
                    3,
                    "assign".to_string(),
                    vec!["xs".to_string()],
                    "METHOD_CALL_EXPRESSION_2".to_string(),
                    0
                ),
                Instruction::new(
                    4,
                    "evaluate".to_string(),
                    vec!["METHOD_CALL_EXPRESSION_2.iter".to_string()],
                    "ITERATOR_0".to_string(),
                    0
                ),
                Instruction::new(
                    5,
                    "evaluate".to_string(),
                    vec!["start".to_string(), "ITERATOR_0".to_string()],
                    "x".to_string(),
                    0
                ),
                Instruction::new(
                    6,
                    "end_of_iteration_check".to_string(),
                    vec!["x".to_string(), "ITERATOR_0".to_string()],
                    "CHECK_CONDITION_ASSIGNMENT_1".to_string(),
                    0
                ),
                Instruction::new(
                    8,
                    "jump".to_string(),
                    vec!["CHECK_CONDITION_ASSIGNMENT_1".to_string(), "7".to_string()],
                    "".to_string(),
                    0
                ),
                Instruction::new(
                    9,
                    "print".to_string(),
                    vec!["\"{}\"".to_string(), "x".to_string()],
                    "".to_string(),
                    7
                ),
                Instruction::new(
                    10,
                    "increment".to_string(),
                    vec!["x".to_string()],
                    "".to_string(),
                    7
                ),
                Instruction::new(
                    11,
                    "goto".to_string(),
                    vec!["6".to_string()],
                    "".to_string(),
                    7
                ),
            ]
        );
    }

    #[test]
    fn test_handle_nested_for_loop_expression() {
        let expr = parse_quote! {
            for i in 0..10 {
                for j in 1..5 {
                    log!("{} {}", i, j);
                }
                log!("{}", i);
            }
        };
        let mut compilation_state = CompilationState::new();

        let result = handle_for_loop_expression(&expr, &mut compilation_state);

        assert_eq!(
            result.unwrap(),
            vec![
                Instruction::new(
                    4,
                    "assign".to_string(),
                    vec!["0".to_string()],
                    "RANGE_START_2".to_string(),
                    0
                ),
                Instruction::new(
                    5,
                    "assign".to_string(),
                    vec!["10".to_string()],
                    "RANGE_END_3".to_string(),
                    0
                ),
                Instruction::new(
                    6,
                    "instantiate_object".to_string(),
                    vec![
                        "Range".to_string(),
                        "RANGE_START_2".to_string(),
                        "RANGE_END_3".to_string()
                    ],
                    "ITERATOR_0".to_string(),
                    0
                ),
                Instruction::new(
                    7,
                    "evaluate".to_string(),
                    vec!["start".to_string(), "ITERATOR_0".to_string()],
                    "i".to_string(),
                    0
                ),
                Instruction::new(
                    8,
                    "end_of_iteration_check".to_string(),
                    vec!["i".to_string(), "ITERATOR_0".to_string()],
                    "CHECK_CONDITION_ASSIGNMENT_1".to_string(),
                    0
                ),
                Instruction::new(
                    10,
                    "jump".to_string(),
                    vec!["CHECK_CONDITION_ASSIGNMENT_1".to_string(), "9".to_string()],
                    "".to_string(),
                    0
                ),
                Instruction::new(
                    15,
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "RANGE_START_13".to_string(),
                    9
                ),
                Instruction::new(
                    16,
                    "assign".to_string(),
                    vec!["5".to_string()],
                    "RANGE_END_14".to_string(),
                    9
                ),
                Instruction::new(
                    17,
                    "instantiate_object".to_string(),
                    vec![
                        "Range".to_string(),
                        "RANGE_START_13".to_string(),
                        "RANGE_END_14".to_string()
                    ],
                    "ITERATOR_11".to_string(),
                    9
                ),
                Instruction::new(
                    18,
                    "evaluate".to_string(),
                    vec!["start".to_string(), "ITERATOR_11".to_string()],
                    "j".to_string(),
                    9
                ),
                Instruction::new(
                    19,
                    "end_of_iteration_check".to_string(),
                    vec!["j".to_string(), "ITERATOR_11".to_string()],
                    "CHECK_CONDITION_ASSIGNMENT_12".to_string(),
                    9
                ),
                Instruction::new(
                    21,
                    "jump".to_string(),
                    vec![
                        "CHECK_CONDITION_ASSIGNMENT_12".to_string(),
                        "20".to_string()
                    ],
                    "".to_string(),
                    9
                ),
                Instruction::new(
                    22,
                    "print".to_string(),
                    vec!["\"{} {}\"".to_string(), "i".to_string(), "j".to_string()],
                    "".to_string(),
                    20
                ),
                Instruction::new(
                    23,
                    "increment".to_string(),
                    vec!["j".to_string()],
                    "".to_string(),
                    20
                ),
                Instruction::new(
                    24,
                    "goto".to_string(),
                    vec!["19".to_string()],
                    "".to_string(),
                    20
                ),
                Instruction::new(
                    25,
                    "print".to_string(),
                    vec!["\"{}\"".to_string(), "i".to_string()],
                    "".to_string(),
                    9
                ),
                Instruction::new(
                    26,
                    "increment".to_string(),
                    vec!["i".to_string()],
                    "".to_string(),
                    9
                ),
                Instruction::new(
                    27,
                    "goto".to_string(),
                    vec!["8".to_string()],
                    "".to_string(),
                    9
                ),
            ]
        );
    }
}
