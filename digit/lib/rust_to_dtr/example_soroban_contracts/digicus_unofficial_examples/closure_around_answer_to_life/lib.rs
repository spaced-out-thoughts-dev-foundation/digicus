#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, log, Address, Env, Vec};

#[contract]
pub struct ClosureAroundAnswerToLife;

#[contractimpl]
impl ClosureAroundAnswerToLife {
    pub fn closure_around_answer_to_life() -> u32 {
        let x = 20;
        let add_closure = |a, b| x + a + b;

        add_closure(1, 21)
    }
}

mod test;
