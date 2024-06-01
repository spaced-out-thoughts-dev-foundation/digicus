#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct AnswerToLifeContract;

#[contractimpl]
impl AnswerToLifeContract {
    pub fn fourty_two(env: Env) -> u32 {
      42
    }
}


mod test;
