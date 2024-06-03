#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};

#[contract]
pub struct IncrementContract;

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> u32 {
let mut METHOD_CALL_EXPRESSION_4 = env.storage();
let mut METHOD_CALL_EXPRESSION_3 = METHOD_CALL_EXPRESSION_4.instance();
let mut METHOD_CALL_EXPRESSION_1 = METHOD_CALL_EXPRESSION_3.get(&COUNTER);
let mut count = METHOD_CALL_EXPRESSION_1.unwrap_or(0);
log!(&env, "count: {}", count);
count += 1;
let mut METHOD_CALL_EXPRESSION_3 = env.storage();
let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_3.instance();
let mut METHOD_CALL_RESULT = METHOD_CALL_EXPRESSION_2.set(&COUNTER, &count);
let mut METHOD_CALL_EXPRESSION_3 = env.storage();
let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_3.instance();
let mut METHOD_CALL_RESULT = METHOD_CALL_EXPRESSION_2.extend_ttl(50, 100);
count
    }
}


mod test;
