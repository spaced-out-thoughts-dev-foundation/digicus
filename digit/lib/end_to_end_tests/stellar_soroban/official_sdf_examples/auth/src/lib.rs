#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, user: Address, value: u32) -> u32 {
        user.require_auth();
        let mut 1_CALL_EXPRESSION_ARG = user.clone();
        let mut key = DataKey::Counter(&1_CALL_EXPRESSION_ARG);
        let mut METHOD_CALL_EXPRESSION_3 = env.storage();
        let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_3.persistent();
        let mut METHOD_CALL_EXPRESSION_0 = METHOD_CALL_EXPRESSION_2.get(&key);
        let mut count = METHOD_CALL_EXPRESSION_0.unwrap_or_default();
        count += value;
        let mut METHOD_CALL_EXPRESSION_3 = env.storage();
        let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_3.persistent();
        METHOD_CALL_EXPRESSION_2.set(&key, &count);
        count
    }
}


mod test;
