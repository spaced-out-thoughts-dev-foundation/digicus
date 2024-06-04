#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct LogIfAnswerToLife;

const ANSWER_TO_LIFE: u32 = 42;

#[contractimpl]
impl LogIfAnswerToLife {
    pub fn fourty_two_and_then_some(env: Env, possibly_the_answer_to_life: u32) {
        if !(possibly_the_answer_to_life == ANSWER_TO_LIFE) {
            log_to_env(&env, "Yes, the answer to life is 42!");
        }
    }
}

fn log_to_env(env: Env, message: String) {
    log!(&env, &message);
}
