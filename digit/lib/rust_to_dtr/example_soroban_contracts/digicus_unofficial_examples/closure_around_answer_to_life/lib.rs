#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, log, Address, Env, Vec};

#[contract]
pub struct ClosureAroundAnswerToLife;

#[contractimpl]
impl ClosureAroundAnswerToLife {
    pub fn closure_around_answer_to_life() -> u32 {
        let add_closure = |a, b| a + b;

        add_closure(20, 22)
    }
}

mod test;
