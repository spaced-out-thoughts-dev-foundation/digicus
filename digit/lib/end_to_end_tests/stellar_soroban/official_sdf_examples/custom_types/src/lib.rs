#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {count: u32, last_incr: u32}

const STATE: Symbol = symbol_short!("STATE");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, incr: u32) -> u32 {
        let mut CALL_EXPRESSION_ARG_1 = env.clone();
        let mut state = get_state(&CALL_EXPRESSION_ARG_1);
        let mut BINARY_EXPRESSION_LEFT = state.count;
        BINARY_EXPRESSION_LEFT += incr;
        let mut ASSIGN_EXPRESSION_LEFT = state.last_incr;
        let mut METHOD_CALL_EXPRESSION_3 = env.storage();
        let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_3.instance();
        METHOD_CALL_EXPRESSION_2.set(&STATE, &state);
        let mut Thing_to_return = state.count;
        Thing_to_return
    }


    pub fn get_state(env: Env) -> State {
        let mut METHOD_CALL_EXPRESSION_4 = env.storage();
        let mut METHOD_CALL_EXPRESSION_3 = METHOD_CALL_EXPRESSION_4.instance();
        let mut METHOD_CALL_EXPRESSION_1 = METHOD_CALL_EXPRESSION_3.get(&STATE);
        let mut METHOD_CALL_ARG_1_0 = State { 0 0 };
        let mut Thing_to_return = METHOD_CALL_EXPRESSION_1.unwrap_or(&METHOD_CALL_ARG_1_0);
        Thing_to_return
    }
}


mod test;
