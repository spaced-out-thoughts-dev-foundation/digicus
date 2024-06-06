#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, log, Address, Env, Vec};

#[contract]
pub struct CountZeroToAnswerToLife;

const Zero: i32 = 0;
const AnswerToLife: i32 = 42;

#[contractimpl]
impl CountZeroToAnswerToLife {
    pub fn count_zero_to_answer_to_life() {
        for i in Zero..AnswerToLife {
            log!("Counting: {}", i);
        }
    }
}

mod test;
