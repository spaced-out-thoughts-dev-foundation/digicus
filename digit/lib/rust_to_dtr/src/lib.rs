use instruction::Instruction;
use syn::ItemStruct;
use translate::type_name::{self, parse_path};

// syn docs: https://docs.rs/syn/2.0.60/syn/index.html
extern crate syn;

pub mod common;
pub mod errors;
pub mod instruction;
pub mod optimize;
pub mod translate;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn parse_to_dtr(rust_code: &str) -> Result<String, errors::NotTranslatableError> {
    // Parse the Rust code into a syn data structure
    let parsed_ast = syn::parse_file(rust_code).unwrap();
    let mut user_defined_types: Vec<ItemStruct> = Vec::new();

    // Extract information from the parsed AST
    let mut dtr_code = String::new();
    for item in parsed_ast.items {
        match item {
            syn::Item::Struct(item_struct) => {
                // here we look at the attributes of the struct such as #[contract] or #[contractimpl]
                item_struct.attrs.iter().for_each(|attr| {
                    // dtr_code.push_str(&format!("{}\n", parse_path(attr.meta.path())));
                    if parse_path(attr.meta.path()) == "contract" {
                        dtr_code.push_str(&format!("[Contract]: {}\n\n", item_struct.ident));
                    } else if parse_path(attr.meta.path()) == "contracttype" {
                        user_defined_types.push(item_struct.clone());
                    }
                });
            }
            syn::Item::Impl(item_impl) => {
                let mut is_a_contract_impl = false;
                if item_impl.attrs.len() > 0 {
                    item_impl.attrs.iter().for_each(|attr| {
                        // dtr_code.push_str(&format!("{}\n", parse_path(attr.path)));
                        if parse_path(attr.meta.path()) == "contractimpl" {
                            is_a_contract_impl = true;
                        }
                    });
                }

                if !is_a_contract_impl {
                    continue;
                }

                dtr_code.push_str("[Functions]:\n");

                item_impl.items.iter().for_each(|item_impl_item| {
                    if let syn::ImplItem::Fn(method) = item_impl_item {
                        dtr_code.push_str(&translate::impl_block::parse_function_block(method));
                    }
                });
                dtr_code.push_str(":[Functions]\n");
            }
            _ => {} // We're ignoring other types of items for simplicity
        }
    }

    // optimize::optimize(instructions);

    dtr_code.push_str("\n\n[User Defined Types]:");

    user_defined_types.iter().for_each(|item_struct| {
        dtr_code.push_str(&format!("\n\n\t* ({})\n", item_struct.ident));
        dtr_code.push_str("\t{\n");

        item_struct.fields.iter().for_each(|field| {
            if let syn::Type::Path(type_path) = &field.ty {
                dtr_code.push_str(&format!(
                    "\t\t{}: {}\n",
                    field.ident.as_ref().unwrap(),
                    type_name::parse_path(&type_path.path)
                ));
            }
        });

        dtr_code.push_str("\t}\n");
    });

    dtr_code.push_str("\n:[User Defined Types]\n");

    Ok(dtr_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANSWER_TO_LIFE_CONTRACT: &str = r#"
    #![no_std]
    use soroban_sdk::{contract, contractimpl, Env};

    #[contract]
    pub struct AnswerToLifeContract;

    #[contractimpl]
    impl AnswerToLifeContract {
        pub fn fourty_two(env: Env) -> u32 {
            42
        }
    }
    "#;

    const INCREMENT_ANSWER_TO_LIFE_CONTRACT: &str = r#"
    #![no_std]
    use soroban_sdk::{contract, contractimpl, Env};

    #[contract]
    pub struct IncrementAnswerToLifeContract;

    #[contractimpl]
    impl IncrementAnswerToLifeContract {
        pub fn fourty_two_and_then_some(env: Env, and_then_some: u32) -> u32 {
            42 + and_then_some
        }
    }
    "#;

    const CUSTOM_TYPES_CONTRACT: &str = r#"
    #![no_std]
    use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol};
    
    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct State {
        pub count: u32,
        pub last_incr: u32,
    }
    
    const STATE: Symbol = symbol_short!("STATE");
    
    #[contract]
    pub struct IncrementContract;
    
    #[contractimpl]
    impl IncrementContract {
        /// Increment increments an internal counter, and returns the value.
       pub fn increment(env: Env, incr: u32) -> u32 {
            // Get the current count.
            let mut state = Self::get_state(env.clone());
    
            // Increment the count.
            state.count += incr;
            state.last_incr = incr;
    
            // Save the count.
            env.storage().instance().set(&STATE, &state);
    
            // Return the count to the caller.
            state.count
        }
        /// Return the current state.
        pub fn get_state(env: Env) -> State {
            unwrap_or(State {
                count: 0,
                last_incr: 0,
            }) // If no value set, assume 0.
        }
    }
    "#;

    #[test]
    fn test_parse_answer_to_life_contract() {
        let expected_dtr_code = r#"[Contract]: AnswerToLifeContract

[Functions]:
-() [fourty_two]
* Inputs:{ }
* Output: u32
* Instructions:
$
{ instruction: Return, input: (42) }
$
:[Functions]
[User Defined Types]:
:[User Defined Types]
"#;

        let actual_dtr_code = parse_to_dtr(ANSWER_TO_LIFE_CONTRACT);

        match actual_dtr_code {
            Ok(dtr_code) => {
                assert_eq!(
                    dtr_code.replace("\t", "").replace("\n", ""),
                    expected_dtr_code.replace("\t", "").replace("\n", "")
                );
            }
            Err(err) => {
                panic!("Error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_parse_increment_answer_to_life_contract() {
        let expected_dtr_code = r#"
[Contract]: IncrementAnswerToLifeContract

[Functions]:
-() [fourty_two_and_then_some]
* Inputs:{ and_then_some: u32}
* Output: u32
* Instructions:
$
{ instruction: add, input: (42, and_then_some), assign: Thing_to_return }
{ instruction: Return, input: (Thing_to_return) }
$
:[Functions]
[User Defined Types]:
:[User Defined Types]
"#;

        let actual_dtr_code = parse_to_dtr(INCREMENT_ANSWER_TO_LIFE_CONTRACT);

        match actual_dtr_code {
            Ok(dtr_code) => {
                assert_eq!(
                    dtr_code.replace("\t", "").replace("\n", ""),
                    expected_dtr_code.replace("\t", "").replace("\n", "")
                );
            }
            Err(err) => {
                panic!("Error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_parse_custom_types_contract() {
        let expected_dtr_code = r#"
[Contract]: IncrementContract
[Functions]:
-() [increment]
* Inputs:
{ 
incr: u32
}
* Output: u32
* Instructions:
$
{ instruction: evaluate, input: (clone, env), assign: 1_CALL_EXPRESSION_ARG }
{ instruction: evaluate, input: (get_state, 1_CALL_EXPRESSION_ARG), assign: state }
{ instruction: field, input: (state, count), assign: BINARY_EXPRESSION_LEFT }
{ instruction: add_and_assign, input: (BINARY_EXPRESSION_LEFT, incr) }
{ instruction: field, input: (state, last_incr), assign: ASSIGN_EXPRESSION_LEFT }
{ instruction: evaluate, input: (storage, env), assign: METHOD_CALL_EXPRESSION }
{ instruction: evaluate, input: (instance, env), assign: METHOD_CALL_EXPRESSION }
{ instruction: evaluate, input: (set, env, STATE, state), assign: METHOD_CALL_RESULT }
{ instruction: field, input: (state, count), assign: Thing_to_return }
{ instruction: Return, input: (Thing_to_return) }
$
-() [get_state]
* Inputs:
{ 
}
* Output: State
* Instructions:
$
{ instruction: initialize_udt, input: (State, 0, 0), assign: 1_CALL_EXPRESSION_ARG }
{ instruction: evaluate, input: (unwrap_or, 1_CALL_EXPRESSION_ARG), assign: Thing_to_return }
{ instruction: Return, input: (Thing_to_return) }
$
:[Functions]


[User Defined Types]:

* (State)
{
count: u32
last_incr: u32
}

:[User Defined Types]
"#;

        let actual_dtr_code = parse_to_dtr(CUSTOM_TYPES_CONTRACT);

        match actual_dtr_code {
            Ok(dtr_code) => {
                assert_eq!(
                    dtr_code.replace("\t", "").replace("\n", ""),
                    expected_dtr_code.replace("\t", "").replace("\n", "")
                );
            }
            Err(err) => {
                panic!("Error: {:?}", err);
            }
        }
    }
}
