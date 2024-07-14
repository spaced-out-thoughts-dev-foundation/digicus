const contractDictionary = {
    "hello_world_logging": `#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, answer_to_life: number, value: Symbol) {
        if value == "fizz" {
            panic!("Not the answer to life!");
        }

        log!(&env, "Hello {}", value);
    }
}
`
}

export function localContractFetch(contractName) {
    return contractDictionary[contractName];
};