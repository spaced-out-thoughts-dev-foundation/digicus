// syn docs: https://docs.rs/syn/2.0.60/syn/index.html
extern crate syn;
use std::string::String;
pub mod errors;
pub mod translate;

pub fn parse_to_dtr(rust_code: &str) -> Result<String, errors::NotTranslatableError> {
    // Parse the Rust code into a syn data structure
    let parsed_ast = syn::parse_file(rust_code).unwrap();

    // Extract information from the parsed AST
    let mut dtr_code = String::new();
    for item in parsed_ast.items {
        match item {
            syn::Item::Struct(item_struct) => {
                dtr_code.push_str(&format!("[Contract]: {}\n\n", item_struct.ident));
            }
            syn::Item::Impl(item_impl) => {
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

                        block.stmts.iter().for_each(|stmt| {
                            dtr_code.push_str("\n\t\t");
                            match translate::expression::block_expression::parse_block_stmt(&stmt) {
                                Ok(block_str) => {
                                    dtr_code.push_str(block_str.as_str());
                                }
                                Err(e) => {
                                    // return Err(e);
                                    dtr_code.push_str(&format!("Error: {:?}", e));
                                }
                            }
                        });

                        dtr_code.push_str("\t\t$\n");

                        dtr_code.push_str(":[Functions]\n");
                    }
                });
            }
            _ => {} // We're ignoring other types of items for simplicity
        }
    }
    Ok(dtr_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELLO_WORLD_CONTRACT_CODE: &str = r#"
        #![no_std]
        use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

        #[contract]
        pub struct HelloContract;

        #[contractimpl]
        impl HelloContract {
            pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
                vec![&env, symbol_short!("Hello"), to]
            }
        }
    "#;

    const INCREMENT_CONTRACT_CODE: &str = r#"
    #![no_std]
    use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};
    
    const COUNTER: Symbol = symbol_short!("COUNTER");
    
    #[contract]
    pub struct IncrementContract;
    
    #[contractimpl]
    impl IncrementContract {
        /// Increment increments an internal counter, and returns the value.
        pub fn increment(env: Env) -> u32 {
            // Get the current count.
            let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
            log!(&env, "count: {}", count);
    
            // Increment the count.
            count += 1;
    
            // Save the count.
            env.storage().instance().set(&COUNTER, &count);
    
            // The contract instance will be bumped to have a lifetime of at least 100 ledgers if the current expiration lifetime at most 50.
            // If the lifetime is already more than 100 ledgers, this is a no-op. Otherwise,
            // the lifetime is extended to 100 ledgers. This lifetime bump includes the contract
            // instance itself and all entries in storage().instance(), i.e, COUNTER.
            env.storage().instance().extend_ttl(50, 100);
    
            // Return the count to the caller.
            count
        }
    }
    "#;

    // #[test]
    // fn test_parse_to_dtr() {
    //     let expected_dtr_code = r#"
    //         [Contract]: HelloContract

    //         [Functions]:
    //         -() [hello]
    //             * Inputs:
    //                 {
    //                 to: Symbol
    //                 }
    //             * Output: Symbol
    //             * Instructions:
    //                 $
    //                 { instruction: AddSymbols, input: ("Hello", to), assign: HelloToResult }
    //                 { instruction: Return, input: (HelloToResult) }
    //                 $
    //         :[Functions]
    //     "#;

    //     let actual_dtr_code = parse_to_dtr(HELLO_WORLD_CONTRACT_CODE);

    //     println!("Expected DTR code:\n\n{}", expected_dtr_code);

    //     println!("Actual DTR code:\n\n{}", actual_dtr_code);

    //     assert_eq!(actual_dtr_code, expected_dtr_code);
    // }

    #[test]
    fn test_parse_increment_contract() {
        let expected_dtr_code = r#"
            [Contract]: IncrementContract

            [Functions]:
            -() [increment]
                * Inputs:
                    { 
                    }
                * Output: u32
                * Instructions:
                    $
                    { instruction: GetStorage, input: (COUNTER), assign: Count }
                    { instruction: Log, input: (Count) }
                    { instruction: Add, input: (Count, 1), assign: Count }
                    { instruction: SetStorage, input: (COUNTER, Count) }
                    { instruction: ExtendTTL, input: (50, 100) }
                    { instruction: Return, input: (Count) }
                    $
            :[Functions]
        "#;

        let actual_dtr_code = parse_to_dtr(INCREMENT_CONTRACT_CODE);

        match actual_dtr_code {
            Ok(dtr_code) => {
                println!("Expected DTR code:\n\n{}", INCREMENT_CONTRACT_CODE);

                println!("Actual DTR code:\n\n{}", dtr_code);

                assert_eq!(dtr_code, expected_dtr_code);
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
