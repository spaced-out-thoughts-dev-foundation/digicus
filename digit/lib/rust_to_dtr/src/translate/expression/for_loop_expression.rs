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

    let global_uuid = compilation_state.global_uuid;
    compilation_state.increment_global_uuid();

    let top_label = format!("loop_{}_top", global_uuid);
    let exit_label = format!("loop_{}_exit", global_uuid);
    let iterator_variable: String = handle_pattern(*expr.pat.clone()).unwrap();
    let check_condition_assignment: String = "CHECK_CONDITION_ASSIGNMENT".to_string();

    instructions.extend(get_initial_value(
        expr.clone(),
        compilation_state.clone(),
        iterator_variable.clone(),
    ));
    instructions.extend(get_label(top_label.clone(), compilation_state.clone()));
    instructions.extend(get_check_condition(
        iterator_variable.clone(),
        check_condition_assignment.clone(),
    ));
    instructions.extend(get_if_condition_false_jump_to_exit_label(
        exit_label.clone(),
        check_condition_assignment.clone(),
    ));
    instructions.extend(get_execute_block(expr.clone(), compilation_state));
    instructions.extend(get_increment_value(iterator_variable.clone()));
    instructions.extend(get_back_to_top(
        top_label.clone(),
        compilation_state.clone(),
    ));
    instructions.extend(get_label(exit_label, compilation_state.clone()));

    Ok(instructions)
}

fn get_initial_value(
    for_loop_expr: ExprForLoop,
    compilation_state: CompilationState,
    iterator_variable: String,
) -> Vec<Instruction> {
    parse_expression(
        &*for_loop_expr.expr.clone(),
        &mut compilation_state.with_assignment(Some(iterator_variable.clone())),
    )
    .unwrap()
}

fn get_check_condition(
    iterator_variable: String,
    check_condition_assignment: String,
) -> Vec<Instruction> {
    vec![Instruction::new(
        "end_of_iteration_check".to_string(),
        vec![iterator_variable.clone()],
        check_condition_assignment.clone(),
        0,
    )]
}

fn get_if_condition_false_jump_to_exit_label(
    exit_label: String,
    check_condition_assignment: String,
) -> Vec<Instruction> {
    vec![Instruction::new(
        "conditional_goto".to_string(),
        vec![check_condition_assignment.clone(), exit_label.clone()],
        "".to_string(),
        0,
    )]
}

fn get_increment_value(iterator_variable: String) -> Vec<Instruction> {
    vec![Instruction::new(
        "increment".to_string(),
        vec![iterator_variable],
        "".to_string(),
        0,
    )]
}

fn get_execute_block(
    for_loop_expr: ExprForLoop,
    compilation_state: &mut CompilationState,
) -> Vec<Instruction> {
    handle_block(&for_loop_expr.body.clone(), compilation_state)
}

fn get_back_to_top(label: String, compilation_state: CompilationState) -> Vec<Instruction> {
    vec![Instruction::from_compilation_state(
        "goto".to_string(),
        vec![label.clone()],
        &mut compilation_state.clone(),
    )]
}

fn get_label(label: String, compilation_state: CompilationState) -> Vec<Instruction> {
    vec![Instruction::from_compilation_state(
        "label".to_string(),
        vec![label.clone()],
        &mut compilation_state.clone(),
    )]
}
