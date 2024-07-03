#![no_std]
use soroban_sdk::{Symbol, symbol_short, contract, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

const KEY: Symbol = symbol_short!("value");

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn init(env: Env, value: i128)  {
        env.storage().instance().set(&KEY, &value);
    }


    pub fn value(env: Env) -> i128 {
        let Thing_to_return: i128;
        Thing_to_return = env.storage().instance().get(&KEY).unwrap();
        return Thing_to_return;
    }
}

