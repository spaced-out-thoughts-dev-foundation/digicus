#![no_std]

use soroban_sdk::{contract, contractimpl, log, Address, Env, Vec};

#[contract]
pub struct NonRangeForLoopContract;

#[contractimpl]
impl NonRangeForLoopContract {
    pub fn do_a_thing_for_each_element_in_vec(some_vec: Vec<String>) {
        for some_thing_in_a_vec in some_vec.iter() {
            log!("Doing a thing");
        }
    }
}

mod test;
