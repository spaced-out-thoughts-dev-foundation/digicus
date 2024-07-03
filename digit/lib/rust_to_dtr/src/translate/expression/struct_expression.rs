use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::translate::type_name::parse_path;
use syn::ExprStruct;

pub fn handle_struct_expression(
    expr: &ExprStruct,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let path_value: String = parse_path(&expr.path);

    let mut field_names: Vec<String> = Vec::new();

    expr.fields.iter().for_each(|field| {
        let field_name = match field.member {
            syn::Member::Named(ref ident) => ident.to_string(),
            syn::Member::Unnamed(_) => "".to_string(),
        };
        let field_value = field.expr.clone();

        let original_assignment = compilation_state.next_assignment.clone();

        let field_value_parsed = parse_expression(
            &field_value,
            &mut compilation_state.with_assignment(Some(field_name.clone())),
        );

        compilation_state.with_assignment(original_assignment);

        instructions.extend(field_value_parsed.unwrap_or(Vec::new()));
        field_names.push(field_name.clone());
    });

    field_names.insert(0, path_value.clone());
    field_names.insert(0, "UDT".to_string());

    instructions.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "instantiate_object".to_string(),
        field_names,
        compilation_state.next_assignment.clone().unwrap_or(format!(
            "STRUCT_EXPRESSION_RESULT_{}",
            compilation_state.get_global_uuid()
        )),
        compilation_state.scope(),
    ));

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::{
        common::compilation_state::CompilationState,
        translate::expression::struct_expression::handle_struct_expression,
    };
    use syn::{parse_quote, ExprStruct};

    #[test]
    fn test_handle_struct_expression() {
        let mut compilation_state = CompilationState::new();
        let expr: ExprStruct = parse_quote! { Struct { a: 1, b: 2 } };
        let instructions = handle_struct_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::new(
                    0,
                    "assign".to_string(),
                    vec!["1".to_string()],
                    "a".to_string(),
                    0
                ),
                Instruction::new(
                    1,
                    "assign".to_string(),
                    vec!["2".to_string()],
                    "b".to_string(),
                    0
                ),
                Instruction::new(
                    2,
                    "instantiate_object".to_string(),
                    vec![
                        "UDT".to_string(),
                        "Struct".to_string(),
                        "a".to_string(),
                        "b".to_string()
                    ],
                    "STRUCT_EXPRESSION_RESULT_3".to_string(),
                    0
                )
            ]
        );
    }
}
