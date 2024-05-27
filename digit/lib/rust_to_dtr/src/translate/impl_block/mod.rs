use crate::instruction::Instruction;
use crate::optimize;
use crate::translate;

pub fn parse_function_block(method: &syn::ImplItemFn) -> String {
    let mut dtr_code: String = "".to_string();

    let method_name = method.sig.ident.to_string();

    dtr_code.push_str(&format!("-() [{}]\n", method_name));

    dtr_code.push_str(&parse_inputs(&method.clone()));

    if let syn::ReturnType::Type(_, ty) = &method.sig.output {
        dtr_code.push_str(translate::parse_return_type(ty).as_str());
    }

    dtr_code.push_str(&parse_instructions(&method.clone()));

    dtr_code
}

pub fn parse_inputs(method: &syn::ImplItemFn) -> String {
    let mut dtr_code: String = "".to_string();

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
                            dtr_code.push_str(&format!("\t\t{}: {}\n", pat_ident.ident, type_name));
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

    dtr_code
}

pub fn parse_instructions(method: &syn::ImplItemFn) -> String {
    let mut dtr_code: String = "".to_string();

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

        let assignment: Option<String> = if index == total_block_stmts {
            Some("Thing_to_return".to_string())
        } else {
            None
        };
        match translate::expression::supported::block_expression::parse_block_stmt(
            &stmt, assignment,
        ) {
            Ok(block_str) => {
                let mut instructions: Vec<Instruction> = Vec::new();

                block_str.iter().for_each(|instr| {
                    instructions.push(instr.clone());
                });

                if index == total_block_stmts {
                    instructions.push(Instruction::new(
                        "Return".to_string(),
                        vec!["Thing_to_return".to_string()],
                        "".to_string(),
                    ));
                }

                dtr_code.push_str(&instructions_to_string(instructions));
            }
            Err(e) => {
                // return Err(e);
                dtr_code.push_str(&format!("Error: {:?}", e));
            }
        }
        index += 1;
    });

    dtr_code.push_str("\n\t\t$\n");

    dtr_code
}

fn instructions_to_string(instructions: Vec<Instruction>) -> String {
    let optimized_instructions = optimize::apply(instructions);

    let instructions_as_strings: Vec<String> = optimized_instructions
        .iter()
        .map(|instr| instr.as_str())
        .collect();
    instructions_as_strings.join("\n\t\t\t")
}
