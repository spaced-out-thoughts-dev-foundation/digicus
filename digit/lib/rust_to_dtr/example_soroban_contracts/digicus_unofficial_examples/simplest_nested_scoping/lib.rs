#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct NestedScoping;

#[contractimpl]
impl NestedScoping {
    pub fn fourty_two_and_then_some(some_input: u32) -> u32 {
        if some_input > 10 {
            if some_input > 15 {
                let x = 5;
                x += 10;

                if x > some_input {
                    log!("Some input is greater than 15");
                }

                return if x == some_input { x } else { some_input };
            }
        }

        some_input += 1;

        some_input
    }
}
