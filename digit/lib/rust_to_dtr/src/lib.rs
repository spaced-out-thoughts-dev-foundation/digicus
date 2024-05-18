// syn docs: https://docs.rs/syn/2.0.60/syn/index.html
extern crate syn;
// use std::string::String;

pub mod errors;
pub mod translate;

// pub fn parse_to_dtr(rust_code: &str) -> Result<String, errors::NotTranslatableError> {
//     // Parse the Rust code into a syn data structure
//     let parsed_ast = syn::parse_file(rust_code).unwrap();

//     // Extract information from the parsed AST
//     let mut dtr_code = String::new();
//     for item in parsed_ast.items {
//         match item {
//             syn::Item::Struct(item_struct) => {
//                 dtr_code.push_str(&format!("[Contract]: {}\n\n", item_struct.ident));
//             }
//             syn::Item::Impl(item_impl) => {
//                 dtr_code.push_str("[Functions]:\n");

//                 item_impl.items.iter().for_each(|item_impl_item| {
//                     if let syn::ImplItem::Fn(method) = item_impl_item {
//                         let method_name = method.sig.ident.to_string();

//                         dtr_code.push_str(&format!("-() [{}]\n", method_name));

//                         dtr_code.push_str("\t* Inputs:\n");
//                         dtr_code.push_str("\t{ \n");

//                         method.sig.inputs.iter().for_each(|input| {
//                             if let syn::FnArg::Typed(pat_type) = input {
//                                 if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
//                                     if pat_ident.ident != "env" {
//                                         dtr_code.push_str(&format!(
//                                             "\t\t{}: {}\n",
//                                             pat_ident.ident,
//                                             translate::type_name::figure_out_type(&pat_type.ty)

//                                             match
//                                         ));
//                                     }
//                                 }
//                             }
//                         });

//                         dtr_code.push_str("\t}\n");

//                         if let syn::ReturnType::Type(_, ty) = &method.sig.output {
//                             dtr_code.push_str(translate::parse_return_type(ty).as_str());
//                         }

//                         dtr_code.push_str("\t* Instructions:\n");
//                         dtr_code.push_str("\t\t$\n");

//                         let block = &method.block;

//                         block.stmts.iter().for_each(|stmt| {
//                             dtr_code
//                                 .push_str(&format!("\t\t{}", translate::parse_block_stmt(stmt)));
//                             dtr_code.push_str("\t\t{ }\n");
//                         });

//                         dtr_code.push_str("\t\t$\n");

//                         dtr_code.push_str(":[Functions]\n");
//                     }
//                 });
//             }
//             _ => {} // We're ignoring other types of items for simplicity
//         }
//     }
//     Ok(dtr_code)
// }
