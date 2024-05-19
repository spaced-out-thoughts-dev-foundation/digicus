#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct AnswerToLifeContract;

#[contractimpl]
impl AnswerToLifeContract {
    pub fn fourty_two(env: Env) -> u32 {
        let x = 10;
        x += 1;

        x
    }
}
