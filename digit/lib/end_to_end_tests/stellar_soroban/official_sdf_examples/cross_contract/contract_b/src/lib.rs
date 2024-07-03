#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, auth::Context, IntoVal, unwrap::UnwrapOptimized};



mod contract_a {
	soroban_sdk::contractimport!(
		file = "../contract_a/target/wasm32-unknown-unknown/release/soroban_cross_contract_a_contract.wasm"
	);
}

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract: Address, x: i128, y: i128) -> i128 {
        let Thing_to_return: i128;
        let mut client = contract_a::Client::new(&env, &contract);
        Thing_to_return = client.add(&x, &y);
        return Thing_to_return;
    }
}



mod test;
