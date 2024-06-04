#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, vec, Env, String, Symbol, Vec};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, value: String) {
        log!(&env, "Hello {}", &value);
    }
}

mod test;
