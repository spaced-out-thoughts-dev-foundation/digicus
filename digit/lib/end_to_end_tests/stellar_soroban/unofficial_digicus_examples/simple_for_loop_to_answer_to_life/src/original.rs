#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct SimpleForLoopToAnswerToLifeContract;

#[contractimpl]
impl SimpleForLoopToAnswerToLifeContract {
    pub fn fourty_two(env: Env) {
        for i in 0..42 {}
    }
}
