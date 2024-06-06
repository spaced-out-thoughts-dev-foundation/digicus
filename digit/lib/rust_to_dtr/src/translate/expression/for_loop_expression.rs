use crate::common::compilation_state;
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
    let assignment_left_side = handle_pattern(*expr.pat.clone())?;
    let assignment_right_side: String = format!("FOR_LOOP_COMPARISON_{}", compilation_state.scope);

    if let syn::Expr::Range(expr_range) = &*expr.expr {
        let initial_value = match expr_range.clone().start {
            Some(start) => parse_expression(
                &start,
                &mut compilation_state.with_assignment(Some(assignment_left_side.clone())),
            )?,
            None => vec![Instruction::new(
                "assign".to_string(),
                vec!["0".to_string()],
                assignment_left_side.clone(),
                compilation_state.scope,
            )],
        };

        instructions.extend(initial_value);

        instructions.push(Instruction::new(
            "label".to_string(),
            vec![format!(
                "FOR_LOOP_COMPARISON_LABEL_{}",
                compilation_state.scope
            )],
            "".to_string(),
            compilation_state.scope,
        ));

        let condition_check = parse_expression(
            &expr_range.clone().end.unwrap(),
            &mut compilation_state.with_assignment(Some(assignment_right_side.clone())),
        )?;

        instructions.extend(condition_check);

        instructions.push(Instruction::new(
            "equal_to".to_string(),
            vec![assignment_left_side.clone(), assignment_right_side.clone()],
            format!("FOR_LOOP_COMPARISON_RESULT_{}", compilation_state.scope),
            compilation_state.scope,
        ));

        instructions.push(Instruction::new(
            "conditional_goto".to_string(),
            vec![
                format!("FOR_LOOP_COMPARISON_RESULT_{}", compilation_state.scope),
                format!("FOR_LOOP_EXIT_LABEL_{}", compilation_state.scope),
            ],
            "".to_string(),
            compilation_state.scope,
        ));

        instructions.push(Instruction::new(
            "add_and_assign".to_string(),
            vec![assignment_left_side.clone(), "1".to_string()],
            "".to_string(),
            compilation_state.scope,
        ));

        // Block
        let block_instructions = handle_block(&expr.body, compilation_state);

        instructions.extend(block_instructions);

        instructions.push(Instruction::new(
            "unconditional_jump".to_string(),
            vec![format!(
                "FOR_LOOP_COMPARISON_LABEL_{}",
                compilation_state.scope
            )],
            "".to_string(),
            compilation_state.scope,
        ));

        instructions.push(Instruction::new(
            "label".to_string(),
            vec![format!("FOR_LOOP_EXIT_LABEL_{}", compilation_state.scope)],
            "".to_string(),
            compilation_state.scope,
        ));
    } else {
        panic!("For loop expression is not a range expression");
    }

    Ok(instructions)
}
