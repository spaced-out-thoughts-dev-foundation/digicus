#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct NestedScoping;

#[contractimpl]
impl NestedScoping {
    pub fn fourty_two_and_then_some(is_answer_to_life: bool) -> u32 {
        return if is_answer_to_life { 42 } else { 40 };
    }
}
