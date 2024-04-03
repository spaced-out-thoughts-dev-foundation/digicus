extern crate syn;

use syn::parse_file;
// use std::error::Error;
// use std::fs::File;
// use std::io::Read;

fn main() {
    // Your Rust code to parse
    let rust_code = r#"
        fn main() {
            println!("Hello, world!");
        }
    "#;

    let ast = parse_file(rust_code);
    ast.unwrap().attrs.iter().for_each(|attr| {
        println!("{:?}", attr.bracket_token.span.unwrap());
    });
}
