#![no_std]

use soroban_sdk::{contract, contractimpl, log, Address, Env, Vec};

#[contract]
pub struct MoreComplexIfConditionalContract;

#[contractimpl]
impl MoreComplexIfConditionalContract {
    pub fn do_a_more_complex_if_conditional(buy_price: i32, sell_price: i32) {
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }
    }
}

mod test;
