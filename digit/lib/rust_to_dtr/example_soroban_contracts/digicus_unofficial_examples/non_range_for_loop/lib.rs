//! This contract performs a batch of atomic token swaps between multiple
//! parties and does a simple price matching.
//! Parties don't need to know each other and also don't need to know their
//! signatures are used in this contract; they sign the `AtomicSwap` contract
//! invocation that guarantees that their token will be swapped with someone
//! while following the price limit.
//! This example demonstrates how authorized calls can be batched together.
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
