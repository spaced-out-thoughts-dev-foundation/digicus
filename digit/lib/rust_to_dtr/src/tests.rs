#[cfg(test)]
mod full_contract_tests {
    use crate::{
        common::compilation_state::GLOBAL_UNIQUE_NUMBER_GENERATOR, rust_to_dtr_c::parse_to_dtr,
    };
    use rstest::rstest;
    use std::fs;

    // Setup function to reset the global UUID generator
    fn setup() {
        GLOBAL_UNIQUE_NUMBER_GENERATOR.reset();
    }

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
                #[rstest]
                #[test]
                fn [< test_ $directory_name _ $contract_name >] () {
                    setup();
                    assert_transpiled_code(&format!("example_soroban_contracts/{}/{}", stringify!($directory_name), stringify!($contract_name)));
                }
            }
        };
    }

    test_contract!(digicus_unofficial_examples, answer_to_life);
    test_contract!(digicus_unofficial_examples, closure_around_answer_to_life);
    test_contract!(digicus_unofficial_examples, count_zero_to_answer_to_life);
    test_contract!(digicus_unofficial_examples, get_force_type_answer_to_life);
    test_contract!(digicus_unofficial_examples, increment_answer_to_life);
    test_contract!(digicus_unofficial_examples, innerds_of_enums);
    test_contract!(digicus_unofficial_examples, log_if_answer_to_life);
    test_contract!(digicus_unofficial_examples, non_range_for_loop);
    test_contract!(
        digicus_unofficial_examples,
        numbered_enums_to_the_answer_of_life
    );
    test_contract!(digicus_unofficial_examples, various_result_handling_tactics);

    test_contract!(stellar_official_repo_examples, account);
    test_contract!(stellar_official_repo_examples, alloc);
    test_contract!(stellar_official_repo_examples, atomic_multiswap);

    test_contract!(stellar_official_repo_examples, atomic_swap);
    test_contract!(stellar_official_repo_examples, auth);
    test_contract!(stellar_official_repo_examples, custom_types);
    test_contract!(stellar_official_repo_examples, errors);
    test_contract!(stellar_official_repo_examples, eth_abi);
    test_contract!(stellar_official_repo_examples, events);
    test_contract!(stellar_official_repo_examples, hello_world);
    test_contract!(stellar_official_repo_examples, increment);
    test_contract!(stellar_official_repo_examples, logging);
    test_contract!(stellar_official_repo_examples, mint_lock);
    test_contract!(stellar_official_repo_examples, simple_account);
    test_contract!(stellar_official_repo_examples, single_offer);
    test_contract!(stellar_official_repo_examples, timelock);
    test_contract!(stellar_official_repo_examples, ttl);
}
