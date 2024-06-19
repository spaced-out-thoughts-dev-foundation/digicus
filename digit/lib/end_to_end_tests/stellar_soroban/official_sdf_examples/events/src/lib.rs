#![no_std]
use soroban_sdk::{Symbol, symbol_short, contract, contractimpl, Env};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;


#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> i64 {
        let mut count = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count = count + 1;
        env.storage().instance().set(&COUNTER, &count);
        env.events().publish(Tuple(COUNTER, "increment"), &count);
        count
    }
}


mod test;
