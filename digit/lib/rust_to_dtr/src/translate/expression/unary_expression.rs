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
    let global_uuid = compilation_state.get_global_uuid();

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

#[cfg(test)]
mod tests {
    use crate::common::compilation_state::CompilationState;
    use crate::instruction::Instruction;
    use crate::translate::expression::unary_expression::handle_unary_expression;
    use syn::parse_quote;

    #[test]
    fn test_handle_unary_negation_expression() {
        let mut compilation_state = CompilationState::new();
        let expr = parse_quote! { !a };
        let instructions = handle_unary_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "UNARY_ARGUMENT_0".to_string(),
                    0
                ),
                Instruction::from_compilation_state(
                    "evaluate".to_string(),
                    vec!["!".to_string(), "UNARY_ARGUMENT_0".to_string()],
                    &compilation_state
                )
            ]
        );
    }

    #[test]
    fn test_handle_unary_minus_expression() {
        let mut compilation_state = CompilationState::new();
        let expr = parse_quote! { -a };
        let instructions = handle_unary_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "UNARY_ARGUMENT_0".to_string(),
                    0
                ),
                Instruction::from_compilation_state(
                    "evaluate".to_string(),
                    vec!["-".to_string(), "UNARY_ARGUMENT_0".to_string()],
                    &compilation_state
                )
            ]
        );
    }
}
