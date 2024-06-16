use crate::common::compilation_state::CompilationState;
use crate::errors;
use crate::instruction;
use crate::translate;
use crate::translate::expression::parse_expression;
use crate::translate::function;
use crate::translate::rust_to_dtr_term::map_name;
use crate::translate::type_name::figure_out_type;
use crate::translate::type_name::{self, parse_path};

pub fn parse_to_dtr(rust_code: &str) -> Result<String, errors::NotTranslatableError> {
    // Parse the Rust code into a syn data structure
    let parsed_ast = syn::parse_file(rust_code).unwrap();
    let mut user_defined_types: Vec<syn::Item> = Vec::new();
    let mut state_str: String = String::new();
    let mut outside_of_contract_functions: Vec<syn::ItemFn> = Vec::new();

    state_str.push_str("[State]:");

    // Extract information from the parsed AST
    let mut dtr_code = String::new();
    for item in parsed_ast.items {
        match &item {
            syn::Item::Struct(item_struct) => {
                // here we look at the attributes of the struct such as #[contract] or #[contractimpl]
                item_struct.attrs.iter().for_each(|attr| {
                    if parse_path(attr.meta.path()) == "contract" {
                        dtr_code.push_str(&format!("[Contract]: {}\n\n", item_struct.ident));
                    } else if parse_path(attr.meta.path()) == "contracttype" {
                        user_defined_types.push(item.clone());
                    }
                });
            }
            syn::Item::Impl(item_impl) => {
                let mut is_a_contract_impl = false;
                if item_impl.attrs.len() > 0 {
                    item_impl.attrs.iter().for_each(|attr| {
                        if parse_path(attr.meta.path()) == "contractimpl" {
                            is_a_contract_impl = true;
                        }
                    });
                }

                if !is_a_contract_impl {
                    continue;
                }

                dtr_code.push_str("[Interface]:\n");

                item_impl.items.iter().for_each(|item_impl_item| {
                    if let syn::ImplItem::Fn(method) = item_impl_item {
                        dtr_code.push_str(&translate::impl_block::parse_function_block(method));
                    }
                });
                dtr_code.push_str(":[Interface]\n");
            }
            syn::Item::Enum(enum_item) => {
                enum_item.attrs.iter().for_each(|attr| {
                    if parse_path(attr.meta.path()) == "contracttype"
                        || parse_path(attr.meta.path()) == "contracterror"
                    {
                        user_defined_types.push(item.clone());
                    }
                });
            }
            syn::Item::Const(const_item) => {
                let name = const_item.ident.to_string();

                state_str.push_str(&format!("\n* [{}]", name));
                state_str.push_str(&format!(
                    "\n\t* Type: {}",
                    map_name(&figure_out_type(&const_item.ty.clone())?).unwrap()
                ));
                state_str.push_str(&format!(
                    "\n\t* Initial Value: {}",
                    extract_value_from_instruction(&parse_expression(
                        &const_item.expr,
                        &mut CompilationState::new()
                    )?)
                ));
            }
            syn::Item::Fn(fn_item) => {
                outside_of_contract_functions.push(fn_item.clone());
            }
            syn::Item::Macro(item_macro) => {
                // TODO: handle other macros, this is hacky but covers a few cases
                if parse_path(&item_macro.mac.path).as_str() == "sol" {
                    user_defined_types.push(item.clone());
                }
            }
            _ => {} // We're ignoring other types of items for simplicity
        }
    }

    // optimize::optimize(instructions);

    dtr_code.push_str("\n\n[User Defined Types]:");

    user_defined_types.iter().for_each(|item| {
        dtr_code.push_str(&syn_item_to_user_defined_type(item));
    });

    dtr_code.push_str("\n:[User Defined Types]\n");

    if state_str != "[State]:" {
        dtr_code.push_str(&state_str);
        dtr_code.push_str("\n");
    }

    if outside_of_contract_functions.len() > 0 {
        dtr_code.push_str("\n\n[Helpers]:\n");

        outside_of_contract_functions.iter().for_each(|fn_item| {
            dtr_code.push_str(&function::parse_function_block(fn_item));
        });

        dtr_code.push_str("\n:[Helpers]\n");
    }

    Ok(dtr_code)
}

fn extract_value_from_instruction(instructions: &Vec<instruction::Instruction>) -> String {
    instructions[0].input[0].clone()
}

fn syn_item_to_user_defined_type(item: &syn::Item) -> String {
    match item {
        syn::Item::Struct(item_struct) => syn_item_struct_to_user_defined_type(item_struct),
        syn::Item::Enum(item_enum) => syn_item_enum_to_user_defined_type(item_enum),
        syn::Item::Macro(macro_item) => syn_item_macro_to_user_defined_type(macro_item),
        _ => "".to_string(),
    }
}

fn syn_item_struct_to_user_defined_type(item: &syn::ItemStruct) -> String {
    let mut dtr_code = String::new();

    dtr_code.push_str(&format!("* ({})\n", item.ident));
    dtr_code.push_str("{\n");

    item.fields.iter().for_each(|field| {
        if let syn::Type::Path(type_path) = &field.ty {
            dtr_code.push_str(&format!(
                "\t{}: {}\n",
                field.ident.as_ref().unwrap(),
                map_name(&type_name::parse_path(&type_path.path)).unwrap()
            ));
        }
    });

    dtr_code.push_str("}\n");

    dtr_code
}

fn syn_item_enum_to_user_defined_type(item: &syn::ItemEnum) -> String {
    let mut dtr_code = String::new();

    dtr_code.push_str(&format!("\n* ({})\n", item.ident));
    dtr_code.push_str("{\n");

    item.variants.iter().for_each(|variant| {
        dtr_code.push_str(&format!("\t{}", variant.ident));

        let disc = match &variant.discriminant {
            Some((_, expr)) => {
                let instructions = parse_expression(&expr, &mut CompilationState::new()).unwrap();
                instructions[0].input[0].clone()
            }
            None => "".to_string(),
        };

        match &variant.fields {
            syn::Fields::Named(fields_named) => {
                let mut innerd_enum_types: Vec<String> = vec![];

                // ASSUMPTION: here we ignore the name
                fields_named.named.iter().for_each(|field| {
                    innerd_enum_types.push(
                        map_name(&figure_out_type(&field.ty.clone()).unwrap_or_default()).unwrap(),
                    );
                });

                dtr_code.push_str(format!(": ({})\n", innerd_enum_types.join(", ")).as_str());
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                let mut innerd_enum_types: Vec<String> = vec![];

                fields_unnamed.unnamed.iter().for_each(|field| {
                    innerd_enum_types.push(
                        map_name(&figure_out_type(&field.ty.clone()).unwrap_or_default()).unwrap(),
                    );
                });

                dtr_code.push_str(format!(": ({})\n", innerd_enum_types.join(", ")).as_str());
            }
            syn::Fields::Unit => {
                if disc != "" {
                    dtr_code.push_str(format!(" = {}\n", disc).as_str());
                } else {
                    dtr_code.push_str(": ()\n");
                }
            }
        }
    });

    dtr_code.push_str("}\n");

    dtr_code
}

fn syn_item_macro_to_user_defined_type(item: &syn::ItemMacro) -> String {
    let mut dtr_code = String::new();
    let macro_name = item.mac.path.segments[0].ident.to_string();
    let macro_title = if macro_name == "sol" {
        "Solidity ABI Types".to_string()
    } else {
        macro_name
    };

    dtr_code.push_str(&format!("\n* ({})\n", macro_title));
    dtr_code.push_str("{\n");

    item.mac.tokens.to_string().lines().for_each(|line| {
        dtr_code.push_str(&format!("\t{}\n", line));
    });

    dtr_code.push_str("}\n");

    dtr_code
}
