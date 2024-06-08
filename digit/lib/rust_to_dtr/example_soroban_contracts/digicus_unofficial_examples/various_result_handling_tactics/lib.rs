#![no_std]

use soroban_sdk::{contract, contractimpl, log, Address, Env, Vec};

#[contract]
pub struct VariousResultHandlingTacticsContract;

#[contractimpl]
impl VariousResultHandlingTacticsContract {
    pub fn return_some_thing() -> i32 {
        let some_thing: Option<i32> = Some(42);

        if let Some(some_thing) = some_thing {
            log!("Doing a thing");
        }

        some_thing?
    }
}

mod test;
