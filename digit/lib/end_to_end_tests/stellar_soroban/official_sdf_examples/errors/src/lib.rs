#![no_std]
use soroban_sdk::{contract, contracttype, Symbol, symbol_short, contractimpl, Env, log};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {

}

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;


#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> i64 {
        let mut count = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);
        let mut CONDITIONAL_JUMP_ASSIGNMENT = &less_than_or_equal_to(&count(1), &MAX);


                env.storage().instance().set(&COUNTER, &count);
                let mut Thing_to_return = &Ok(&count);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut Thing_to_return = &Err(&Error::LimitReached);
        Thing_to_return
    }
}


mod test;
