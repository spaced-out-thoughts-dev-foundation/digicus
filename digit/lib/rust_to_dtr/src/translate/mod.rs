extern crate syn;

pub mod block;
pub mod expression;
pub mod function;
pub mod impl_block;
pub mod pattern;
pub mod rust_to_dtr_term;
pub mod statement;
pub mod type_name;

pub fn parse_return_type(ty: &syn::Type) -> String {
    match type_name::figure_out_type(ty) {
        Ok(val) => format!("\t* Output: {}\n", val),
        Err(e) => format!(
            "\t* Output: Could not figure out type for return type parsing {}\n",
            e
        ),
    }
}
