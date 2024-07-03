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
    compilation_state: &mut CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let macro_path = parse_path(&mac.path);
    let macro_path_to_instruction_and_should_assign_result =
        macro_path_to_instruction_and_should_assign(macro_path.clone());

    let instruction_operation = macro_path_to_instruction_and_should_assign_result.0;
    let should_assign = macro_path_to_instruction_and_should_assign_result.1;

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

    let assign = if should_assign {
        assignment.unwrap_or(get_random_string())
    } else {
        "".to_string()
    };

    Ok(vec![Instruction::new(
        // TODO: fix this hardcoding
        compilation_state.get_global_uuid(),
        instruction_operation,
        macro_tokens_to_inputs(mac.tokens.clone()),
        assign,
        compilation_state.scope(),
    )])
}

fn macro_path_to_instruction_and_should_assign(path: String) -> (String, bool) {
    match path.as_str() {
        "vec" => ("create_list".to_string(), true),
        "log" => ("print".to_string(), false),
        "print" => ("print".to_string(), false),
        "println" => ("print".to_string(), false),
        "symbol_short" => ("assign".to_string(), true),
        "panic" => ("exit_with_message".to_string(), false),
        "alloc::vec" => ("create_list".to_string(), true),
        _ => ("unknown_macro".to_string(), false),
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
