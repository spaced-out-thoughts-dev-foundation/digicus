use rust_to_dtr_term::map_name;

extern crate syn;

pub mod block;
pub mod expression;
pub mod function;
pub mod pattern;
pub mod rust_to_dtr_term;
pub mod statement;
pub mod type_name;

pub fn parse_return_type(ty: &syn::Type) -> String {
    match type_name::figure_out_type(ty) {
        Ok(val) => {
            let mapped_val = map_name(&val).unwrap();
            if mapped_val != "" {
                return format!("\t* Output: {}\n", mapped_val);
            }

            format!("")
        }
        Err(e) => format!(
            "\t* Output: Could not figure out type for return type parsing {}\n",
            e
        ),
    }
}
