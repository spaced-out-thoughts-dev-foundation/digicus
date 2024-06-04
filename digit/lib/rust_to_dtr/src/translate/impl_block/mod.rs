use crate::common::compilation_state::CompilationState;
use crate::instruction::Instruction;
use crate::optimize;
use crate::translate;

pub fn parse_function_block(method: &syn::ImplItemFn) -> String {
    let mut dtr_code: String = "".to_string();

    let method_name = method.sig.ident.to_string();

    dtr_code.push_str(&format!("-() [{}]\n", method_name));

    dtr_code.push_str(&parse_inputs(&method.clone()));

    let mut has_output = false;

    if let syn::ReturnType::Type(_, ty) = &method.sig.output {
        has_output = true;
        dtr_code.push_str(translate::parse_return_type(ty).as_str());
    }

    dtr_code.push_str(&&parse_instructions(&method.clone(), has_output));

    dtr_code
}

fn parse_inputs(method: &syn::ImplItemFn) -> String {
    let mut dtr_code: String = "".to_string();

    dtr_code.push_str("\t* Inputs:\n");
    dtr_code.push_str("\t{\n");

    method.sig.inputs.iter().for_each(|input| {
        if let syn::FnArg::Typed(pat_type) = input {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                // dtr_code
                //     .push_str(&translate::pattern::handle_pattern(pat_ident).unwrap());
                // if pat_ident.ident != "env" {
                match translate::type_name::figure_out_type(&pat_type.ty) {
                    Ok(type_name) => {
                        dtr_code.push_str(&format!(
                            "\t\t{}: {}\n",
                            pat_ident.ident.to_string().trim(),
                            type_name.trim()
                        ));
                    }
                    Err(e) => {
                        // return Err(e);
                        dtr_code.push_str(&format!("Error: {:?}", e));
                    }
                }
                // }
            }
        }
    });

    dtr_code.push_str("\t}\n");

    dtr_code
}

fn parse_instructions(method: &syn::ImplItemFn, has_output: bool) -> String {
    let mut dtr_code: String = "".to_string();

    dtr_code.push_str("\t* Instructions:\n");
    dtr_code.push_str("\t\t$\n");

    let mut comp_state = CompilationState::new();
    comp_state.should_output = has_output;

    let block_instructions: Vec<Instruction> =
        translate::block::handle_block(&method.block, &mut comp_state);
    dtr_code.push_str(instructions_to_string(block_instructions.clone()).as_str());

    dtr_code.push_str("\n\t\t$\n");

    dtr_code
}

fn instructions_to_string(instructions: Vec<Instruction>) -> String {
    let optimized_instructions = optimize::apply(instructions);

    let instructions_as_strings: Vec<String> = optimized_instructions
        .iter()
        .map(|instr| instr.as_str())
        .collect();

    let mut instructions_string = instructions_as_strings.join("\n\t\t\t");
    instructions_string = format!("\t\t\t{}", instructions_string);

    instructions_string
}
