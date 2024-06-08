#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct GetForceTypeAnswerToLife;

#[contractimpl]
impl GetForceTypeAnswerToLife {
    pub fn fourty_two_and_then_some(env: Env) {
        env.storage().instance().get::<_, i32>(42)
    }
}
