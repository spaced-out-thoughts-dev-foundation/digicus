#![no_std]
use soroban_sdk::{contract, contracttype, Symbol, symbol_short, contractimpl, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {count: i64, last_incr: i64}

const STATE: Symbol = symbol_short!("STATE");

#[contract]
pub struct IncrementContract;


#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, incr: i64) -> i64 {
        let mut state = &get_state(env.clone());
        let mut BINARY_EXPRESSION_LEFT = state.count;
        BINARY_EXPRESSION_LEFT = BINARY_EXPRESSION_LEFT + incr;
        let mut ASSIGN_EXPRESSION_LEFT = state.last_incr;
        env.storage().instance().set(&STATE, &state);
        let mut Thing_to_return = state.count;
        Thing_to_return
    }


    pub fn get_state(env: Env) -> State {
        let mut Thing_to_return = env.storage().instance().get(&STATE).unwrap_or(&UDT(State, 0, 0));
        Thing_to_return
    }
}


mod test;
