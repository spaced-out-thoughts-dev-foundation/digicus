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
        _ => Err(NotTranslatableError::Custom(
            "Unknown pattern in block pat".to_string(),
        )),
    }
}
