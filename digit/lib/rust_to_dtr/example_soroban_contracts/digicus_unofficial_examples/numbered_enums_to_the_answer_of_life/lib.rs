#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct AnswerToLifeContract;

#[contractimpl]
impl AnswerToLifeContract {
    pub fn fourty_two(env: Env) -> u32 {
        SomeNumberedEnums::AnswerToLife as u32
    }
}

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum SomeNumberedEnums {
    NotTheAnswerToLife = 0,
    CloserToTheAnswerToLife = 41,
    AnswerToLife = 42,
}
