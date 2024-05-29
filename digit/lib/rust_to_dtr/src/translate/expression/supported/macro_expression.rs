use crate::instruction::Instruction;
use crate::translate::type_name::parse_path;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprMacro;

pub fn handle_macro_expression(
    expr: &ExprMacro,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let macro_path = parse_path(&expr.mac.path);

    Ok(vec![Instruction::new(
        macro_path_to_instruction(macro_path.clone()),
        macro_tokens_to_inputs(expr.mac.tokens.clone()),
        assignment.unwrap_or("".to_string()),
    )])
}

fn macro_path_to_instruction(path: String) -> String {
    match path.as_str() {
        "vec" => "create_list".to_string(),
        _ => "unknown_macro".to_string(),
    }
}

fn macro_tokens_to_inputs(tokens: proc_macro2::TokenStream) -> Vec<String> {
    let mut inputs = vec![];
    let mut last_token_was_exclamation_point = false;

    for token in tokens {
        match token.clone() {
            proc_macro2::TokenTree::Punct(_punct) => {
                if token.clone().to_string() == "!" {
                    last_token_was_exclamation_point = true;
                    continue;
                }
            }

            proc_macro2::TokenTree::Group(group) => {
                // HACK: allows us to not include macro name idents/literals
                if last_token_was_exclamation_point {
                    inputs.pop();
                }

                macro_tokens_to_inputs(group.stream()).iter().for_each(|x| {
                    inputs.push(x.clone());
                });
            }
            proc_macro2::TokenTree::Ident(ident) => {
                inputs.push(ident.to_string());
            }
            proc_macro2::TokenTree::Literal(literal) => {
                inputs.push(literal.to_string());
            }
        }

        last_token_was_exclamation_point = false;
    }

    inputs.into_iter().filter(|x| x != "" && x != " ").collect()
}
