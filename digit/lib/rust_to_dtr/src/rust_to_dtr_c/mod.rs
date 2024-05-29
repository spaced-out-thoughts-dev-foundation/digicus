use syn::ItemStruct;

use crate::errors;
use crate::translate;
use crate::translate::type_name::{self, parse_path};

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
