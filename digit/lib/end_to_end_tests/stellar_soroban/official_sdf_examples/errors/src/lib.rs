#![no_std]
use soroban_sdk::{contract, contracterror, Symbol, symbol_short, contractimpl, Env, log, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    LimitReached = 1,
}

const COUNTER: Symbol = symbol_short!("COUNTER");
const MAX: i128 = 5;

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> Result<i128, Error> {
        let Thing_to_return: Result<i128, Error>;
        let mut count:i128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);
        let mut count = count + 1;
        let CONDITIONAL_JUMP_ASSIGNMENT_19 = count <= MAX;
        if CONDITIONAL_JUMP_ASSIGNMENT_19 {
        else {
            env.storage().instance().set(&COUNTER, &count);
            Thing_to_return = Ok(count);
            else {
                Thing_to_return = Err(Error::LimitReached);
                else {
            }
        }
        return Thing_to_return;
    }
}



mod test;
