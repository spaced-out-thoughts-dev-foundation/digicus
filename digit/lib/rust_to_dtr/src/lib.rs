// syn docs: https://docs.rs/syn/2.0.60/syn/index.html
extern crate syn;

pub mod common;
pub mod errors;
pub mod instruction;
pub mod optimize;
pub mod rust_to_dtr_c;
pub mod translate;

use std::ffi::{CStr, CString};

use std::os::raw::c_char;
use std::ptr;

use instruction::Instruction;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn parse_to_dtr(rust_code: &str) -> Result<String, errors::NotTranslatableError> {
    rust_to_dtr_c::parse_to_dtr(rust_code)
}

#[no_mangle]
pub extern "C" fn rust_string_to_dtr_string(rust_code: *const c_char) -> *const c_char {
    if rust_code.is_null() {
        return ptr::null();
    }

    // Convert the C string to a Rust &str
    let c_str = unsafe { CStr::from_ptr(rust_code) };
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null(),
    };

    // Call the function to convert the Rust string to DTR string
    let dtr_string = rust_to_dtr_c::parse_to_dtr(rust_str).unwrap_or_default();

    // Convert the DTR string to CString and return the raw pointer
    let c_string = match CString::new(dtr_string) {
        Ok(s) => s,
        Err(_) => return ptr::null(),
    };

    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_cstring(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
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
                println!("DTR Code: {:}", dtr_code);

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
                    assert_transpiled_code(&format!("example_soroban_contracts/{}/{}", stringify!($directory_name), stringify!($contract_name)));
                }
            }
        };
    }

    test_contract!(digicus_unofficial_examples, answer_to_life);
    test_contract!(digicus_unofficial_examples, count_zero_to_answer_to_life);
    test_contract!(digicus_unofficial_examples, increment_answer_to_life);
    test_contract!(digicus_unofficial_examples, log_if_answer_to_life);

    // test_contract!(stellar_official_repo_examples, account);
    // test_contract!(stellar_official_repo_examples, alloc);
    // test_contract!(stellar_official_repo_examples, atomic_multiswap);

    test_contract!(stellar_official_repo_examples, atomic_swap);
    test_contract!(stellar_official_repo_examples, auth);
    test_contract!(stellar_official_repo_examples, cross_contract_a);
    // test_contract!(stellar_official_repo_examples, cross_contract_b);

    test_contract!(stellar_official_repo_examples, custom_types);
    test_contract!(stellar_official_repo_examples, deployer_contract);

    test_contract!(stellar_official_repo_examples, deployer_deployer);

    test_contract!(stellar_official_repo_examples, errors);
    // test_contract!(stellar_official_repo_examples, eth_abi);

    test_contract!(stellar_official_repo_examples, events);
    // test_contract!(stellar_official_repo_examples, fuzzing);
    test_contract!(stellar_official_repo_examples, hello_world);
    test_contract!(stellar_official_repo_examples, increment);
    // test_contract!(stellar_official_repo_examples, liquidity_pool);
    test_contract!(stellar_official_repo_examples, logging);
    test_contract!(stellar_official_repo_examples, simple_account);
    test_contract!(stellar_official_repo_examples, single_offer);
    test_contract!(stellar_official_repo_examples, timelock);
    test_contract!(stellar_official_repo_examples, ttl);
    // test_contract!(
    //     stellar_official_repo_examples,
    //     upgradable_contract_new_contract
    // );
    // test_contract!(
    //     stellar_official_repo_examples,
    //     upgradable_contract_old_contract
    // );
    // test_contract!(stellar_official_repo_examples, workspace_contract_a);
    // test_contract!(
    //     stellar_official_repo_examples,
    //     workspace_contract_a_interface
    // );
    // test_contract!(stellar_official_repo_examples, workspace_contract_b);
}
