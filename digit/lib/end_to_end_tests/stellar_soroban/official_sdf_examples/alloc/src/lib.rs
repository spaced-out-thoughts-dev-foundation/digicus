#![no_std]
use soroban_sdk::{contract, contractimpl, Env, vec, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct AllocContract;

#[contractimpl]
impl AllocContract {
    pub fn sum(_env: Env, count: i128) -> i128 {
        let Thing_to_return: i128;
        let mut v1 = vec![];
        let mut METHOD_CALL_EXPRESSION_7 = 0..count;
        METHOD_CALL_EXPRESSION_7.for_each(v1.push(&i));
        let mut sum = 0;
        let mut ITERATOR_15 = v1;
        let mut i = start(&v1);
        if !iteration_finished {
        if CHECK_CONDITION_ASSIGNMENT_16 {
            sum = sum + i;
            increment: i
            goto: 19
        }
        return sum;
    }
}



mod test;
