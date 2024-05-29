// syn docs: https://docs.rs/syn/2.0.60/syn/index.html
extern crate syn;

pub mod common;
pub mod errors;
pub mod instruction;
pub mod optimize;
pub mod rust_to_dtr_c;
pub mod translate;

use instruction::Instruction;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn parse_to_dtr(rust_code: &str) -> Result<String, errors::NotTranslatableError> {
    rust_to_dtr_c::parse_to_dtr(rust_code)
}

#[cfg(test)]
mod full_contract_tests {
    use super::*;
    use std::fs;

    fn read_file_content(file_path: &str) -> String {
        fs::read_to_string(file_path).expect("Something went wrong reading the file")
    }

    fn assert_transpiled_code(contract_directory: &str) -> () {
        let expected_dtr_code = read_file_content(&format!("{}/lib.dtr", contract_directory));
        let actual_rust_code = read_file_content(&format!("{}/lib.rs", contract_directory));

        match parse_to_dtr(&actual_rust_code) {
            Ok(dtr_code) => {
                assert_eq!(
                    dtr_code.replace("\t", "").replace("\n", ""),
                    expected_dtr_code.replace("\t", "").replace("\n", "")
                );
            }
            Err(err) => {
                panic!("Error: {:?}", err);
            }
        }
    }

    macro_rules! test_contract {
        ($directory_name:ident, $contract_name:ident) => {
            paste::item! {
                #[test]
                fn [< test_ $directory_name _ $contract_name >] () {
                    assert_transpiled_code(&format!("src/example_soroban_contracts/{}/{}", stringify!($directory_name), stringify!($contract_name)));
                }
            }
        };
    }

    test_contract!(digicus_unofficial_examples, answer_to_life);
    test_contract!(digicus_unofficial_examples, increment_answer_to_life);
    test_contract!(stellar_official_repo_examples, hello_world);
    test_contract!(stellar_official_repo_examples, custom_types);
    test_contract!(stellar_official_repo_examples, logging);
    test_contract!(stellar_official_repo_examples, increment);

    // test_contract!(stellar_official_repo_examples, eth_abi);
    // test_contract!(stellar_official_repo_examples, timelock);
    // test_contract!(stellar_official_repo_examples, ttl);
}
