#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, token, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {
    pub fn swap(env: Env, a: Address, b: Address, token_a: Address, token_b: Address, amount_a: i128, min_b_for_a: i128, amount_b: i128, min_a_for_b: i128)  {
        let CONDITIONAL_JUMP_ASSIGNMENT_0 = amount_b < min_b_for_a;
        if CONDITIONAL_JUMP_ASSIGNMENT_0 {
            panic!("not enough token B for token A");
        }
        let CONDITIONAL_JUMP_ASSIGNMENT_10 = amount_a < min_a_for_b;
        if CONDITIONAL_JUMP_ASSIGNMENT_10 {
            panic!("not enough token A for token B");
        }
        let mut TUPLE_ARG_1_0 = token_a.clone();
        let mut TUPLE_ARG_2_0 = token_b.clone();
        let mut METHOD_CALL_EXPRESSION_23 = (TUPLE_ARG_1_0, TUPLE_ARG_2_0, amount_a, min_b_for_a);
        a.require_auth_for_args(METHOD_CALL_EXPRESSION_23.into_val(&env));
        let mut TUPLE_ARG_1_0 = token_b.clone();
        let mut TUPLE_ARG_2_0 = token_a.clone();
        let mut METHOD_CALL_EXPRESSION_41 = (TUPLE_ARG_1_0, TUPLE_ARG_2_0, amount_b, min_a_for_b);
        b.require_auth_for_args(METHOD_CALL_EXPRESSION_41.into_val(&env));
        move_token(&env, &token_a, &a, &b, &amount_a, &min_a_for_b);
        move_token(&env, &token_b, &b, &a, &amount_b, &min_b_for_a);
    }
}

pub fn move_token(env: &Env, token: &Address, from: &Address, to: &Address, max_spend_amount: &i128, transfer_amount: &i128)  {
    let mut token = token::Client::new(env, token);
    let mut contract_address = env.current_contract_address();
    token.transfer(from, &contract_address, max_spend_amount);
    token.transfer(&contract_address, to, transfer_amount);
    let mut METHOD_CALL_ARG_3_32 = max_spend_amount - transfer_amount;
    token.transfer(&contract_address, from, &METHOD_CALL_ARG_3_32);
}



mod test;
