pub mod compilation_state;

use crate::instruction::Instruction;
use crate::translate::type_name::parse_path;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use compilation_state::CompilationState;
use rand::{distributions::Alphanumeric, Rng};
use syn::Macro;

pub fn get_random_string() -> String {
    let mut rng = rand::thread_rng();
    (0..10).map(|_| rng.sample(Alphanumeric) as char).collect()
}

pub fn join_with_newline(s1: &str, s2: &str) -> String {
    s1.to_string() + "\n" + s2
}

pub fn handle_macro(
    mac: &Macro,
    assignment: Option<String>,
    compilation_state: CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let macro_path = parse_path(&mac.path);
    let instruction_operation = macro_path_to_instruction(macro_path.clone());

    if instruction_operation == "create_list" {
        let mut inputs = macro_tokens_to_inputs(mac.tokens.clone());
        inputs.insert(0, "List".to_string());
        return Ok(vec![Instruction::new(
            compilation_state.get_global_uuid(),
            "instantiate_object".to_string(),
            inputs,
            assignment.unwrap_or("".to_string()),
            compilation_state.scope(),
        )]);
    }

    Ok(vec![Instruction::new(
        // TODO: fix this hardcoding
        compilation_state.get_global_uuid(),
        instruction_operation,
        macro_tokens_to_inputs(mac.tokens.clone()),
        assignment.unwrap_or("".to_string()),
        compilation_state.scope(),
    )])
}

fn macro_path_to_instruction(path: String) -> String {
    match path.as_str() {
        "vec" => "create_list".to_string(),
        "log" => "print".to_string(),
        "symbol_short" => "assign".to_string(),
        "panic" => "exit_with_message".to_string(),
        "alloc::vec" => "create_list".to_string(),
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
