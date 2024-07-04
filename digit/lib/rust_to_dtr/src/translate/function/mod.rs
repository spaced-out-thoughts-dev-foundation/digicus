use crate::common::compilation_state::CompilationState;
use crate::instruction::Instruction;
use crate::optimize;
use crate::translate;

// It turns out that `syn::ItemFn` and `syn::ImplItemFn` are very similar, so we can refactor the code to use a interface for both.
// ItemFn docs: https://docs.rs/syn/2.0.60/syn/struct.ItemFn.html
// ImplItemFn docs: https://docs.rs/syn/2.0.60/syn/struct.ImplItemFn.html

use syn::{Attribute, Block, Signature, Visibility};

pub trait SynFunction {
    fn attrs(&self) -> Vec<Attribute>;
    fn vis(&self) -> Visibility;
    fn sig(&self) -> Signature;
    fn block(&self) -> Box<Block>;
}

impl SynFunction for syn::ItemFn {
    fn attrs(&self) -> Vec<Attribute> {
        self.attrs.clone()
    }

    fn vis(&self) -> Visibility {
        self.vis.clone()
    }

    fn sig(&self) -> Signature {
        self.sig.clone()
    }

    fn block(&self) -> Box<Block> {
        self.block.clone()
    }
}

impl SynFunction for syn::ImplItemFn {
    fn attrs(&self) -> Vec<Attribute> {
        self.attrs.clone()
    }

    fn vis(&self) -> Visibility {
        self.vis.clone()
    }

    fn sig(&self) -> Signature {
        self.sig.clone()
    }

    fn block(&self) -> Box<Block> {
        Box::new(self.block.clone())
    }
}

pub fn parse_function_block(
    method: &dyn SynFunction,
    compilation_state: &mut CompilationState,
) -> String {
    let mut dtr_code: String = "".to_string();

    let method_name = method.sig().ident.to_string();

    dtr_code.push_str(&format!("-() [{}]\n", method_name));

    dtr_code.push_str(&parse_inputs(method));

    let mut has_output = false;

    if let syn::ReturnType::Type(_, ty) = &method.sig().output {
        has_output = true;
        dtr_code.push_str(translate::parse_return_type(ty).as_str());
    }

    dtr_code.push_str(&&parse_instructions(method, has_output, compilation_state));

    dtr_code
}

fn parse_inputs(method: &dyn SynFunction) -> String {
    let mut dtr_code: String = "".to_string();

    dtr_code.push_str("\t* Inputs:\n");
    dtr_code.push_str("\t{\n");

    method.sig().inputs.iter().for_each(|input| {
        if let syn::FnArg::Typed(pat_type) = input {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
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

fn parse_instructions(
    method: &dyn SynFunction,
    has_output: bool,
    compilation_state: &mut CompilationState,
) -> String {
    let mut dtr_code: String = "".to_string();

    dtr_code.push_str("\t* Instructions:\n");
    dtr_code.push_str("\t\t$\n");

    compilation_state.should_output = has_output;

    let block_instructions: Vec<Instruction> =
        translate::block::handle_block(&method.block(), compilation_state);
    dtr_code
        .push_str(instructions_to_string(block_instructions.clone(), compilation_state).as_str());

    dtr_code.push_str("\n\t\t$\n");

    dtr_code
}

fn instructions_to_string(
    instructions: Vec<Instruction>,
    compilation_state: &mut CompilationState,
) -> String {
    let optimized_instructions = optimize::apply(instructions, compilation_state.clone());

    let instructions_as_strings: Vec<String> = optimized_instructions
        .iter()
        .map(|instr| instr.as_str())
        .collect();

    let mut instructions_string = instructions_as_strings.join("\n\t\t\t");
    instructions_string = format!("\t\t\t{}", instructions_string);

    instructions_string
}
