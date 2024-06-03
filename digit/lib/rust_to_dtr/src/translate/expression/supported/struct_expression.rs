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

        let field_value_parsed = parse_expression(
            &field_value,
            &mut compilation_state.with_assignment(Some(field_name.clone())),
        );

        instructions.extend(field_value_parsed.unwrap_or(Vec::new()));
        field_names.push(field_name.clone());
    });

    field_names.insert(0, path_value.clone());

    instructions.push(Instruction::new(
        "initialize_udt".to_string(),
        field_names,
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or("STRUCT_EXPRESSION_RESULT".to_string()),
        compilation_state.scope,
    ));

    Ok(instructions)
}
