#![no_std]
use soroban_sdk::{contract, Env, contractimpl};

#[contract]
pub struct AtomicSwapContract;


    pub fn move_token(env: &Env, token: &Address, from: &Address, to: &Address, max_spend_amount: Bigi64, transfer_amount: Bigi64)  {
        let mut token = &token::Client::new(&env, &token);
        &token.transfer(&from, env.current_contract_address(), &max_spend_amount);
        &token.transfer(&contract_address, &to, &transfer_amount);
        &token.transfer(&contract_address, &from, &max_spend_amount(transfer_amount));
    }

#[contractimpl]
impl AtomicSwapContract {
    pub fn swap(env: Env, a: Address, b: Address, token_a: Address, token_b: Address, amount_a: Bigi64, min_b_for_a: Bigi64, amount_b: Bigi64, min_a_for_b: Bigi64)  {
        let mut CONDITIONAL_JUMP_ASSIGNMENT = &less_than(&amount_b, &min_b_for_a);

                panic! "not enough token B for token A";
        let mut CONDITIONAL_JUMP_ASSIGNMENT = &less_than(&amount_a, &min_a_for_b);

                panic! "not enough token A for token B";
        let mut 1_TUPLE_ARG = &token_a.clone();
        let mut 2_TUPLE_ARG = &token_b.clone();
        &a.require_auth_for_args(Tuple(&1_TUPLE_ARG, &2_TUPLE_ARG, &amount_a, &min_b_for_a).into_val(&env));
        let mut 1_TUPLE_ARG = &token_b.clone();
        let mut 2_TUPLE_ARG = &token_a.clone();
        &b.require_auth_for_args(Tuple(&1_TUPLE_ARG, &2_TUPLE_ARG, &amount_b, &min_a_for_b).into_val(&env));
        &move_token(&env, &token_a, &a, &b, &amount_a, &min_a_for_b);
        &move_token(&env, &token_b, &b, &a, &amount_b, &min_b_for_a);
    }
}


mod test;
