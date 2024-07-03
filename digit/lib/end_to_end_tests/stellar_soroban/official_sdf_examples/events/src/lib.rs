#![no_std]
use soroban_sdk::{Symbol, symbol_short, contract, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> i128 {
        let Thing_to_return: i128;
        let mut count:i128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let mut count = count + 1;
        env.storage().instance().set(&COUNTER, &count);
        let mut METHOD_CALL_ARG_1_29 = (COUNTER, symbol_short!("increment"));
        env.events().publish(METHOD_CALL_ARG_1_29, count);
        return count;
    }
}



mod test;
