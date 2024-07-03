#![no_std]
use soroban_sdk::{contract, contracttype, Symbol, symbol_short, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub count: i128,
    pub last_incr: i128,
}

const STATE: Symbol = symbol_short!("STATE");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, incr: i128) -> i128 {
        let Thing_to_return: i128;
        let mut state = Self::get_state(env.clone());
        state.count = state.count + incr;
        state.last_incr = incr;
        env.storage().instance().set(&STATE, &state);
        return state.count;
    }


    pub fn get_state(env: Env) -> State {
        let Thing_to_return: State;
        let mut METHOD_CALL_ARG_1_0 = State{count: 0, last_incr: 0};
        Thing_to_return = env.storage().instance().get(&STATE).unwrap_or(METHOD_CALL_ARG_1_0);
        return Thing_to_return;
    }
}



mod test;
