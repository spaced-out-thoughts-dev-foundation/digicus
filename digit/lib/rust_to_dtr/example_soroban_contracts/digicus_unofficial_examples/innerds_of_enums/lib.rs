#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contracttype]
pub enum StorageKey {
    Admin,
    Minter(Address, Address),
    MinterStats(Address, Address, u32, u32),
}

#[contract]
pub struct InnerdsOfEnums;

#[contractimpl]
impl InnerdsOfEnums {
    pub fn answer_to_life() -> u32 {
        42
    }
}
