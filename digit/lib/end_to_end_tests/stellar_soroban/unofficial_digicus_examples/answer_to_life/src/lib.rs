#![no_std]
use soroban_sdk::{contract, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct AnswerToLifeContract;

#[contractimpl]
impl AnswerToLifeContract {
    pub fn fourty_two(env: Env) -> i128 {
        let Thing_to_return: i128;
        return 42;
    }
}



mod test;
