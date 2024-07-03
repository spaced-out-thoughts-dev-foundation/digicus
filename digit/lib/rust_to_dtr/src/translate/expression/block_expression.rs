// use super::pattern::handle_pattern;
use crate::common::compilation_state::CompilationState;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::block::handle_block;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;
use crate::translate::statement::macro_statement::handle_macro_statement;
use syn::ExprBlock;

// A block is a collection of statements
pub fn handle_block_expression(
    expr_block: &ExprBlock,
    compilation_state: &mut CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    Ok(handle_block(&expr_block.block, compilation_state))
}

pub fn parse_block_stmt(
    stmt: &syn::Stmt,
    compilation_state: &mut CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    match stmt {
        syn::Stmt::Local(local) => {
            let pattern_as_string = handle_pattern(local.pat.clone()).unwrap();
            match &local.init {
                Some(local_init) => match local_init.diverge.clone() {
                    None => parse_expression(
                        &local_init.expr,
                        &mut compilation_state.with_assignment(Some(pattern_as_string)),
                    ),
                    Some(diverged_expr) => {
                        let try_assign_result =
                            format!("TRY_ASSIGN_RESULT_{}", compilation_state.get_global_uuid());
                        let try_assign_result_conditional = format!(
                            "TRY_ASSIGN_RESULT_CONDITIONAL_{}",
                            compilation_state.get_global_uuid()
                        );

                        let mut return_instructions: Vec<Instruction> = parse_expression(
                            &local_init.expr,
                            &mut compilation_state.with_assignment(Some(try_assign_result.clone())),
                        )?;

                        return_instructions.push(Instruction::new(
                            compilation_state.get_global_uuid(),
                            "evaluate".to_string(),
                            vec![
                                "try_assign".to_string(),
                                pattern_as_string.clone(),
                                try_assign_result.clone(),
                            ],
                            try_assign_result_conditional.clone(),
                            compilation_state.scope(),
                        ));

                        let mut prev_scope = compilation_state.scope();
                        let original_scope = prev_scope;
                        compilation_state.enter_new_scope();
                        let conditional_jump_scope = compilation_state.scope();

                        return_instructions.push(Instruction::conditional_jump(
                            prev_scope,
                            conditional_jump_scope,
                            compilation_state.get_global_uuid(),
                            try_assign_result_conditional.clone(),
                        ));

                        compilation_state.exit_scope();

                        prev_scope = compilation_state.scope();
                        compilation_state.enter_new_scope();
                        return_instructions.push(Instruction::new(
                            compilation_state.get_global_uuid(),
                            "jump".to_string(),
                            vec![compilation_state.scope().to_string()],
                            "".to_string(),
                            prev_scope,
                        ));

                        return_instructions.push(Instruction::unconditional_jump(
                            conditional_jump_scope,
                            original_scope,
                            compilation_state.get_global_uuid(),
                        ));

                        return_instructions.extend(parse_expression(
                            &diverged_expr.1,
                            &mut compilation_state.with_assignment(Some(pattern_as_string.clone())),
                        )?);

                        prev_scope = compilation_state.scope();
                        compilation_state.exit_scope();
                        return_instructions.push(Instruction::new(
                            compilation_state.get_global_uuid(),
                            "jump".to_string(),
                            vec![compilation_state.scope().to_string()],
                            "".to_string(),
                            prev_scope,
                        ));

                        Ok(return_instructions)
                    }
                },
                None => Ok(vec![Instruction::new(
                    compilation_state.get_global_uuid(),
                    "assign".to_string(),
                    vec![pattern_as_string],
                    "".to_string(),
                    compilation_state.scope(),
                )]),
            }
        }
        syn::Stmt::Item(_item) => Err(NotTranslatableError::Custom(
            "Item statement not translatable".to_string(),
        )),
        syn::Stmt::Expr(exp, _r) => parse_expression(exp, compilation_state),
        syn::Stmt::Macro(stmt_mac) => handle_macro_statement(
            stmt_mac,
            compilation_state.next_assignment.clone(),
            compilation_state,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;
    use syn;

    #[test]
    fn test_handle_block_expression() {
        let expr_block: ExprBlock = syn::parse_str("{ let x = 1; }").unwrap();
        let mut compilation_state = CompilationState::new();
        let instructions = handle_block_expression(&expr_block, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["1".to_string()],
                "x".to_string(),
                0,
            ),]
        );
    }

    #[test]
    fn test_parse_block_stmt() {
        let stmt: syn::Stmt = syn::parse_str("let x = 1;").unwrap();
        let mut compilation_state = CompilationState::new();
        let instructions = parse_block_stmt(&stmt, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["1".to_string()],
                "x".to_string(),
                0,
            ),]
        );
    }
}
