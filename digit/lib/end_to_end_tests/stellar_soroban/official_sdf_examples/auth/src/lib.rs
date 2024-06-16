#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataKey {Counter: (Address)}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, user: Address, value: u32) -> u32 {
        user.require_auth();
        let mut CALL_EXPRESSION_ARG_1 = user.clone();
        let mut key = DataKey::Counter(&CALL_EXPRESSION_ARG_1);
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
