use super::type_name::{figure_out_type, parse_path};
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_lit;
use syn;

pub fn handle_pattern(pat: syn::Pat) -> Result<String, NotTranslatableError> {
    match pat {
        syn::Pat::Lit(lit_pat) => Ok(format!("{:?}", parse_lit(&lit_pat.lit))),
        syn::Pat::Ident(ident_pat) => Ok(ident_pat.ident.to_string()),
        syn::Pat::Path(path_pat) => Ok(parse_path(&path_pat.path)),
        syn::Pat::Type(type_pat) => {
            let the_pattern: String = handle_pattern(*type_pat.pat)?;
            let type_name: String = figure_out_type(&type_pat.ty)?;

            Ok(format!("{}|||{}", the_pattern, type_name))
        }
        syn::Pat::TupleStruct(tuple_struct) => {
            let main_path = parse_path(&tuple_struct.path);

            let mut tuple_strs: Vec<String> = vec![];
            for elem in tuple_struct.elems.iter() {
                tuple_strs.push(handle_pattern(elem.clone())?);
            }
            Ok(format!("{}({})", main_path, tuple_strs.join("  ")))
        }
        syn::Pat::Tuple(tuple) => {
            let mut tuple_strs: Vec<String> = vec![];
            for elem in tuple.elems.iter() {
                tuple_strs.push(handle_pattern(elem.clone())?);
            }
            Ok(format!("({})", tuple_strs.join(" ")))
        }
        syn::Pat::Struct(struct_pat) => {
            let main_path = parse_path(&struct_pat.path);

            let struct_str = String::new();
            // for field in struct_pat.fields.iter() {
            //     struct_str.push_str(&handle_pattern(field.pat.clone())?);
            //     struct_str.push_str(" ");
            // }
            Ok(format!("{}{{ {} }}", main_path, struct_str))
        }

        syn::Pat::Slice(slice) => {
            let mut slice_str = String::new();
            for elem in slice.elems.iter() {
                slice_str.push_str(&handle_pattern(elem.clone())?);
                slice_str.push_str(" ");
            }
            Ok(format!("[{}]", slice_str))
        }
        syn::Pat::Verbatim(_) => Ok("Verbatim".to_string()),
        syn::Pat::Wild(_) => Ok("_".to_string()),
        syn::Pat::Rest(_) => Ok("...".to_string()),

        _ => Err(NotTranslatableError::Custom(
            "Unknown pattern in block pat".to_string(),
        )),
    }
}
