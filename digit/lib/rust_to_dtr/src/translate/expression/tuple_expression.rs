use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprTuple;

pub fn handle_tuple_expression(
    expr: &ExprTuple,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut instructions_to_return: Vec<Instruction> = Vec::new();

    let mut index = 1;
    let mut arguments: Vec<String> = Vec::new();
    expr.elems.iter().for_each(|arg| {
        let arg_name = format!("TUPLE_ARG_{}_{}", index, compilation_state.scope());

        arguments.push(arg_name.clone());

        let instructions: Vec<Instruction> =
            parse_expression(arg, &mut compilation_state.with_assignment(Some(arg_name))).unwrap();
        instructions_to_return.extend(instructions);

        index += 1;
    });

    arguments.insert(0, "Tuple".to_string());

    instructions_to_return.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "instantiate_object".to_string(),
        arguments,
        compilation_state.next_assignment.clone().unwrap_or(format!(
            "TUPLE_RESULT_{}",
            compilation_state.get_global_uuid()
        )),
        compilation_state.scope(),
    ));

    Ok(instructions_to_return)
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::{
        common::compilation_state::CompilationState,
        translate::expression::tuple_expression::handle_tuple_expression,
    };
    use syn::{parse_quote, ExprTuple};

    #[test]
    fn test_handle_tuple() {
        let mut compilation_state = CompilationState::new();
        let expr: ExprTuple = parse_quote! { (a, b) };
        let instructions = handle_tuple_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    0,
                    "assign".to_string(),
                    vec!["a".to_string()],
                    "TUPLE_ARG_1_0".to_string(),
                    0
                ),
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["b".to_string()],
                    "TUPLE_ARG_2_0".to_string(),
                    0
                ),
                Instruction::new(
                    2,
                    "instantiate_object".to_string(),
                    vec![
                        "Tuple".to_string(),
                        "TUPLE_ARG_1_0".to_string(),
                        "TUPLE_ARG_2_0".to_string()
                    ],
                    "TUPLE_RESULT_3".to_string(),
                    0
                )
            ]
        );
    }
}
