use crate::common::compilation_state::CompilationState;
use crate::errors;
use crate::translate;
use crate::translate::expression::parse_expression;
use crate::translate::type_name::figure_out_type;
use crate::translate::type_name::{self, parse_path};

pub fn parse_to_dtr(rust_code: &str) -> Result<String, errors::NotTranslatableError> {
    // Parse the Rust code into a syn data structure
    let parsed_ast = syn::parse_file(rust_code).unwrap();
    let mut user_defined_types: Vec<syn::Item> = Vec::new();
    let mut state_str: String = String::new();

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

                dtr_code.push_str("[Functions]:\n");

                item_impl.items.iter().for_each(|item_impl_item| {
                    if let syn::ImplItem::Fn(method) = item_impl_item {
                        dtr_code.push_str(&translate::impl_block::parse_function_block(method));
                    }
                });
                dtr_code.push_str(":[Functions]\n");
            }
            syn::Item::Enum(enum_item) => {
                enum_item.attrs.iter().for_each(|attr| {
                    if parse_path(attr.meta.path()) == "contracttype" {
                        user_defined_types.push(item.clone());
                    }
                });
            }
            syn::Item::Const(const_item) => {
                let name = const_item.ident.to_string();

                state_str.push_str(&format!("\n* [{}]", name));
                state_str.push_str(&format!(
                    "\n\t* Type: {}",
                    figure_out_type(&const_item.ty.clone())?
                ));
                // TODO: this is super hacky and won't always work
                state_str.push_str(&format!("\n\t* Initial Value: \"{}\"", name));
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

    Ok(dtr_code)
}

fn syn_item_to_user_defined_type(item: &syn::Item) -> String {
    match item {
        syn::Item::Struct(item_struct) => syn_item_struct_to_user_defined_type(item_struct),
        syn::Item::Enum(item_enum) => syn_item_enum_to_user_defined_type(item_enum),
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
                type_name::parse_path(&type_path.path)
            ));
        }
    });

    dtr_code.push_str("}\n");

    dtr_code
}

fn syn_item_enum_to_user_defined_type(item: &syn::ItemEnum) -> String {
    let mut dtr_code = String::new();

    dtr_code.push_str(&format!("* ({})\n", item.ident));
    dtr_code.push_str("{\n");

    item.variants.iter().for_each(|variant| {
        dtr_code.push_str(&format!("\t* ({})\n", variant.ident));
        dtr_code.push_str("\t{\n");

        if let syn::Fields::Named(fields_named) = &variant.fields {
            fields_named.named.iter().for_each(|field| {
                if let syn::Type::Path(type_path) = &field.ty {
                    dtr_code.push_str(&format!(
                        "\t\t{}: {}\n",
                        field.ident.as_ref().unwrap(),
                        type_name::parse_path(&type_path.path)
                    ));
                }
            });
        }

        dtr_code.push_str("\t}\n");
    });

    dtr_code.push_str("}\n");

    dtr_code
}
