use translate::{
    expression::supported::path_expression::handle_path_expression,
    type_name::{self, parse_path},
};

// syn docs: https://docs.rs/syn/2.0.60/syn/index.html
extern crate syn;

pub mod common;
pub mod errors;
pub mod instruction;
pub mod translate;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn parse_to_dtr(rust_code: &str) -> Result<String, errors::NotTranslatableError> {
    // Parse the Rust code into a syn data structure
    let parsed_ast = syn::parse_file(rust_code).unwrap();

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

                // match type_name::figure_out_type(&item_impl.self_ty) {
                //     Ok(type_name) => {
                //         dtr_code.push_str(&format!("[Contract]: {}\n\n", type_name));
                //     }
                //     Err(e) => {
                //         // return Err(e);
                //         dtr_code.push_str(&format!("Error: {:?}", e));
                //     }
                // }

                dtr_code.push_str("[Functions]:\n");

                item_impl.items.iter().for_each(|item_impl_item| {
                    if let syn::ImplItem::Fn(method) = item_impl_item {
                        let method_name = method.sig.ident.to_string();

                        dtr_code.push_str(&format!("-() [{}]\n", method_name));

                        dtr_code.push_str("\t* Inputs:\n");
                        dtr_code.push_str("\t{ \n");

                        method.sig.inputs.iter().for_each(|input| {
                            if let syn::FnArg::Typed(pat_type) = input {
                                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                    // dtr_code
                                    //     .push_str(&translate::pattern::handle_pattern(pat_ident).unwrap());
                                    if pat_ident.ident != "env" {
                                        match translate::type_name::figure_out_type(&pat_type.ty) {
                                            Ok(type_name) => {
                                                dtr_code.push_str(&format!(
                                                    "\t\t{}: {}\n",
                                                    pat_ident.ident, type_name
                                                ));
                                            }
                                            Err(e) => {
                                                // return Err(e);
                                                dtr_code.push_str(&format!("Error: {:?}", e));
                                            }
                                        }
                                    }
                                }
                            }
                        });

                        dtr_code.push_str("\t}\n");

                        if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                            dtr_code.push_str(translate::parse_return_type(ty).as_str());
                        }

                        dtr_code.push_str("\t* Instructions:\n");
                        dtr_code.push_str("\t\t$\n");

                        let block = &method.block;

                        let mut index = 1;
                        let total_block_stmts = block.stmts.len();
                        block.stmts.iter().for_each(|stmt| {
                            if index != 1 {
                              dtr_code.push_str("\n\t\t\t");
                            } else {
                              dtr_code.push_str("\t\t\t");
                            }

                            let assignment:Option<String> = if index == total_block_stmts { Some("Thing_to_return".to_string()) } else { None };
                            match translate::expression::supported::block_expression::parse_block_stmt(&stmt, assignment) {
                                Ok(block_str) => {
                                    let mut instructions_as_strings: Vec<String> = Vec::new();

                                    block_str.iter().for_each(|instr|instructions_as_strings.push(instr.as_str()));

                                    if index == total_block_stmts {
                                        instructions_as_strings.push("{ instruction: Return, input: (Thing_to_return) }".to_string());
                                    }

                                    dtr_code.push_str(&instructions_as_strings.join("\n\t\t\t"));
                                }
                                Err(e) => {
                                    // return Err(e);
                                    dtr_code.push_str(&format!("Error: {:?}", e));
                                }
                            }
                            index += 1;

                        });

                        dtr_code.push_str("\n\t\t$\n");

                    }

                });
                dtr_code.push_str(":[Functions]\n");
            }
            _ => {} // We're ignoring other types of items for simplicity
        }
    }
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
            //let mut state = Self::get_state(env.clone());
    
            // Increment the count.
            //state.count += incr;
            //state.last_incr = incr;
    
            // Save the count.
            // env.storage().instance().set(&STATE, &state);
    
            // Return the count to the caller.
            //state.count

            10
        }
        /// Return the current state.
        pub fn get_state(env: Env) -> State {
            env.storage().instance().get(&STATE).unwrap_or(State {
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
-() [fourty_two]* Inputs:{ }* Output: u32* Instructions:${ instruction: assign, input: (42), assign: Thing_to_return }{ instruction: Return, input: (Thing_to_return) }$:[Functions]"#;

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
-() [fourty_two_and_then_some]* Inputs:{ and_then_some: u32}* Output: u32* Instructions:${ instruction: assign, input: (42), assign: BINARY_EXPRESSION_LEFT }{ instruction: assign, input: (and_then_some), assign: BINARY_EXPRESSION_RIGHT }{ instruction: add, input: (BINARY_EXPRESSION_LEFT, BINARY_EXPRESSION_RIGHT), assign: Thing_to_return }{ instruction: Return, input: (Thing_to_return) }$:[Functions]"#;

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
            { instruction: assign, input: (Self), assign: CALL_EXPRESSION_FUNCTION }
            { instruction: assign, input: (env), assign: METHOD_CALL_EXPRESSION }
            { instruction: evaluate, input: (METHOD_CALL_EXPRESSION), assign: METHOD_CALL_RESULT }
            { instruction: evaluate, input: (CALL_EXPRESSION_FUNCTION, 1 METHOD_CALL_ARG), assign: CALL_EXPRESSION_RESULT }
            { instruction: assign, input: (state), assign: FIELD_BASE }
            { instruction: field, input: (FIELD_BASE, count), assign: BINARY_EXPRESSION_LEFT }
            { instruction: assign, input: (incr), assign: BINARY_EXPRESSION_RIGHT }
            { instruction: add_and_assign, input: (BINARY_EXPRESSION_LEFT, BINARY_EXPRESSION_RIGHT) }
            { instruction: assign, input: (state), assign: FIELD_BASE }
            { instruction: field, input: (FIELD_BASE, last_incr), assign: ASSIGN_EXPRESSION_LEFT }
            { instruction: assign, input: (incr), assign: ASSIGN_EXPRESSION_RIGHT }
            { instruction: assign, input: (ASSIGN_EXPRESSION_LEFT, ASSIGN_EXPRESSION_RIGHT) }
            { instruction: assign, input: (env), assign: METHOD_CALL_EXPRESSION }
            { instruction: evaluate, input: (METHOD_CALL_EXPRESSION), assign: METHOD_CALL_RESULT }
            { instruction: evaluate, input: (METHOD_CALL_EXPRESSION), assign: METHOD_CALL_RESULT }
            { instruction: assign, input: (STATE), assign: 1 METHOD_CALL_ARG }
            { instruction: assign, input: (state), assign: 2 METHOD_CALL_ARG }
            { instruction: evaluate, input: (METHOD_CALL_EXPRESSION, 1 METHOD_CALL_ARG, 2 METHOD_CALL_ARG), assign: METHOD_CALL_RESULT }
            { instruction: assign, input: (state), assign: FIELD_BASE }
            { instruction: field, input: (FIELD_BASE, count), assign: Thing_to_return }
            { instruction: Return, input: (Thing_to_return) }
        $
-() [get_state]
    * Inputs:
    { 
    }
    * Output: Could not figure out type
    * Instructions:
        $
            { instruction: assign, input: (env), assign: METHOD_CALL_EXPRESSION }
            { instruction: evaluate, input: (METHOD_CALL_EXPRESSION), assign: METHOD_CALL_RESULT }
            { instruction: evaluate, input: (METHOD_CALL_EXPRESSION), assign: METHOD_CALL_RESULT }
            { instruction: assign, input: (STATE), assign: 1 METHOD_CALL_ARG }
            { instruction: evaluate, input: (METHOD_CALL_EXPRESSION, 1 METHOD_CALL_ARG), assign: METHOD_CALL_RESULT }
            { instruction: evaluate, input: (METHOD_CALL_EXPRESSION, 1 METHOD_CALL_ARG), assign: METHOD_CALL_RESULT }
            { instruction: Return, input: (Thing_to_return) }
        $
:[Functions]
"#;

        let actual_dtr_code = parse_to_dtr(CUSTOM_TYPES_CONTRACT);

        match actual_dtr_code {
            Ok(dtr_code) => {
                println!("dtr_code: {:}\n", dtr_code);

                assert_eq!(
                    dtr_code.replace("\t", "").replace("\n", ""),
                    "".replace("\t", "").replace("\n", "")
                );
            }
            Err(err) => {
                panic!("Error: {:?}", err);
            }
        }
    }
}
