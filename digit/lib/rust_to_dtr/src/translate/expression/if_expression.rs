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
    let mut global_uuid = compilation_state.get_global_uuid();
    let conditional_jump_assignment_label = format!("CONDITIONAL_JUMP_ASSIGNMENT_{}", global_uuid);

    let mut condition_instructions: Vec<Instruction> = parse_expression(
        &expr.cond,
        &mut compilation_state.with_assignment(Some(conditional_jump_assignment_label.to_string())),
    )?;

    let conditional_jump_instruction = Instruction::new(
        "jump".to_string(),
        vec![
            conditional_jump_assignment_label.to_string(),
            (compilation_state.scope + 1).to_string(),
        ],
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or_default(),
        compilation_state.scope,
    );

    condition_instructions.push(conditional_jump_instruction);

    let then_branch = handle_block(&expr.then_branch, &mut compilation_state.with_scope_jump(1));

    condition_instructions.extend(then_branch);

    let else_branch = match &expr.else_branch {
        Some(else_branch) => {
            condition_instructions.push(Instruction::from_compilation_state(
                "jump".to_string(),
                vec![format!("{}", compilation_state.scope + 100)],
                &compilation_state,
            ));

            parse_expression(
                &else_branch.1,
                &mut compilation_state
                    .with_assignment(Some("else_branch".to_string()))
                    .with_scope_jump(100),
            )?
        }
        None => vec![],
    };

    condition_instructions.extend(else_branch);

    Ok(condition_instructions)
}
