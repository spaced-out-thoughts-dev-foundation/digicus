#![no_std]
use soroban_sdk::{contract, contractimpl, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn add(x: i128, y: i128) -> i128 {
        let Thing_to_return: i128;
        Thing_to_return = x.checked_add(y).expect("no overflow");
        return Thing_to_return;
    }
}

