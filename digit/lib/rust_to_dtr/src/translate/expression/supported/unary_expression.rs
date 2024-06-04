use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprUnary;

pub fn handle_unary_expression(
    expr: &ExprUnary,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    println!("Entered handle_unary_expression");
    let global_uuid = compilation_state.global_uuid;
    compilation_state.increment_global_uuid();

    let unary_arg_name = format!("UNARY_ARGUMENT_{}", global_uuid);

    let mut preceding_instructions = parse_expression(
        &expr.expr,
        &mut compilation_state.with_assignment(Some(unary_arg_name.to_string())),
    )?;

    preceding_instructions.push(Instruction::from_compilation_state(
        "evaluate".to_string(),
        vec![determine_unary_operation(&expr.op), unary_arg_name],
        compilation_state,
    ));

    Ok(preceding_instructions)
}

fn determine_unary_operation(op: &syn::UnOp) -> String {
    match op {
        syn::UnOp::Not(_) => "!".to_string(),
        syn::UnOp::Neg(_) => "-".to_string(),
        _ => panic!("Unsupported unary operation"),
    }
}
