use super::type_name::parse_path;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_lit;
use syn;

pub fn handle_pattern(pat: syn::Pat) -> Result<String, NotTranslatableError> {
    match pat {
        syn::Pat::Lit(lit_pat) => Ok(format!("{:?}", parse_lit(&lit_pat.lit))),
        syn::Pat::Ident(ident_pat) => Ok(ident_pat.ident.to_string()),
        syn::Pat::Macro(_) => Ok(format!("Macro")),
        syn::Pat::Or(_) => Ok(format!("Or")),
        syn::Pat::Paren(_) => Ok(format!("Paren")),
        syn::Pat::Path(path_pat) => Ok(parse_path(&path_pat.path)),
        syn::Pat::Range(_) => Ok(format!("Range")),
        syn::Pat::Reference(_) => Ok(format!("Reference")),
        syn::Pat::Rest(_) => Ok(format!("Rest")),
        syn::Pat::Slice(_) => Ok(format!("Slice")),
        syn::Pat::Struct(_) => Ok(format!("Struct")),
        syn::Pat::Tuple(_) => Ok(format!("Tuple")),
        syn::Pat::TupleStruct(_) => Ok(format!("TupleStruct")),
        syn::Pat::Type(type_pat) => Ok(handle_pattern(*type_pat.pat)?),
        syn::Pat::Verbatim(_) => Ok(format!("Verbatim")),
        syn::Pat::Wild(_) => Ok(format!("any")),
        _ => Err(NotTranslatableError::Custom(
            "Unknown pattern in block pat".to_string(),
        )),
    }
}
