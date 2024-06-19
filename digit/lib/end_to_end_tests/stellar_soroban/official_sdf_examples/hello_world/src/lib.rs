#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec, vec, symbol_short};

#[contract]
pub struct HelloContract;


#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut Thing_to_return = vec![&env, symbol_short!("Hello"), to];
        Thing_to_return
    }
}


mod test;
