const contractDictionary = {
    "hello_world_logging": `#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, String};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, value: String) {
        if value == "fizz" {
            panic!(&env, "Buzz!");
        }

        log!(&env, "Hello {}", value);
    }
}
`
}

export function localContractFetch(contractName) {
    return contractDictionary[contractName];
};