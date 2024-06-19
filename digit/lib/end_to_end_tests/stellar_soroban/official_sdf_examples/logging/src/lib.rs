#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, log};

#[contract]
pub struct Contract;


#[contractimpl]
impl Contract {
    pub fn hello(env: Env, value: Symbol)  {
        log!(&env, "Hello {}", value);
    }
}


mod test;
