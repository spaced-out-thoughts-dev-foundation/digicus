use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprArray;

pub fn handle_array_expression(
    expr: &ExprArray,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut element_names: Vec<String> = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    let mut element_index = 0;
    for element in expr.elems.iter() {
        let element_name = format!(
            "ARRAY_EXPRESSION_ELEMENT_{}_{}",
            element_index,
            compilation_state.scope(),
        );

        let original_assignment = compilation_state.next_assignment.clone();
        instructions.extend(crate::translate::expression::parse_expression(
            element,
            &mut compilation_state.with_assignment(Some(element_name.clone())),
        )?);

        compilation_state.with_assignment(original_assignment);

        element_names.push(element_name.clone());

        element_index += 1;
    }

    element_names.insert(0, "Array".to_string());

    instructions.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "instantiate_object".to_string(),
        element_names,
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or_default(),
        compilation_state.scope(),
    ));

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use crate::common::compilation_state::CompilationState;
    use crate::instruction::Instruction;
    use crate::translate::expression::array_expression::handle_array_expression;
    use syn::parse_quote;

    #[test]
    fn test_handle_array_expression() {
        let mut compilation_state = CompilationState::new();
        let expr: syn::ExprArray = parse_quote! { [0, 1, 2] };
        let instructions = handle_array_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    0,
                    "assign".to_string(),
                    vec!["0".to_string()],
                    "ARRAY_EXPRESSION_ELEMENT_0_0".to_string(),
                    0
                ),
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "ARRAY_EXPRESSION_ELEMENT_1_0".to_string(),
                    0
                ),
                Instruction::new(
                    2,
                    "assign".to_string(),
                    vec!["2".to_string()],
                    "ARRAY_EXPRESSION_ELEMENT_2_0".to_string(),
                    0
                ),
                Instruction::new(
                    3,
                    "instantiate_object".to_string(),
                    vec![
                        "Array".to_string(),
                        "ARRAY_EXPRESSION_ELEMENT_0_0".to_string(),
                        "ARRAY_EXPRESSION_ELEMENT_1_0".to_string(),
                        "ARRAY_EXPRESSION_ELEMENT_2_0".to_string()
                    ],
                    "".to_string(),
                    0
                ),
            ]
        );
    }
}
