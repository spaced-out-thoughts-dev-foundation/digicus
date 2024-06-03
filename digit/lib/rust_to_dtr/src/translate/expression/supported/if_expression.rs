use crate::common::compilation_state;
use crate::instruction::Instruction;
// use crate::translate::block::handle_block;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprIf;

pub fn handle_if_expression(
    expr: &ExprIf,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let condition_instructions: Vec<Instruction> = parse_expression(&expr.cond, compilation_state)?;

    // let then_branch = handle_block(&expr.then_branch, scope + 1);
    // let else_branch = match &expr.else_branch {
    //     Some(else_branch) => {
    //         condition_instructions.push(Instruction::new(
    //             "unconditional_jump".to_string(),
    //             vec![format!("{}", scope + 2)],
    //             "".to_string(),
    //             scope,
    //         ));

    //         parse_expression(&else_branch.1, Some("else_branch".to_string()), scope + 2)?
    //     }
    //     None => vec![],
    // };

    // condition_instructions.extend(then_branch);
    // condition_instructions.extend(else_branch);

    Ok(condition_instructions)
}
