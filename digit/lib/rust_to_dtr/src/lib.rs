// syn docs: https://docs.rs/syn/2.0.60/syn/index.html
extern crate syn;

pub mod common;
pub mod errors;
pub mod instruction;
pub mod translate;

use regex::Regex;

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
                            println!("[DEBUG] {} - {}", index, total_block_stmts);
                            if index != 1 {
                              dtr_code.push_str("\n\t\t\t");
                            } else {
                              dtr_code.push_str("\t\t\t");
                            }
                            match translate::expression::supported::block_expression::parse_block_stmt(&stmt) {
                                Ok(block_str) => {
                                    let mut instructions_as_strings: Vec<String> = Vec::new();

                                    block_str.iter().for_each(|instr|instructions_as_strings.push(instr.as_str()));

                                    if index == total_block_stmts {
                                        println!("[DEBUG] - HERE");
                                        let last_instruction = instructions_as_strings.pop().unwrap();
                                        let return_instruction = mutate_to_be_return_instruction(&last_instruction);
                                        instructions_as_strings.push(return_instruction);
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

                        dtr_code.push_str(":[Functions]\n");
                    }
                });
            }
            _ => {} // We're ignoring other types of items for simplicity
        }
    }
    Ok(dtr_code)
}

fn mutate_to_be_return_instruction(instruction: &str) -> String {
    let mut return_instruction = instruction.to_string();
    let re = Regex::new(r"\bassign\b").unwrap();

    let output = re.replacen(&return_instruction, 1, "Return").into_owned();

    println!("{}", output.to_string());

    output
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

    #[test]
    fn test_parse_answer_to_life_contract() {
        let expected_dtr_code = r#"[Contract]: AnswerToLifeContract

[Functions]:
-() [fourty_two]* Inputs:{ }* Output: u32* Instructions:${ instruction: Return, input: (42) }$:[Functions]"#;

        let actual_dtr_code = parse_to_dtr(ANSWER_TO_LIFE_CONTRACT);

        match actual_dtr_code {
            Ok(dtr_code) => {
                println!(
                    "Expected DTR code:\n\n{}",
                    expected_dtr_code.replace("\t", "").replace("\n", "")
                );

                println!(
                    "Actual DTR code:\n\n{}",
                    dtr_code.replace("\t", "").replace("\n", "")
                );

                assert_eq!(
                    dtr_code.replace("\t", "").replace("\n", ""),
                    expected_dtr_code.replace("\t", "").replace("\n", "")
                );
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
