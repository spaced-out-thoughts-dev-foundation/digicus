#![no_std]
use soroban_sdk::{contract, contracttype, Address, contractimpl, Env, token, Vec, auth::Context, IntoVal, unwrap::UnwrapOptimized};



mod atomic_swap {
	soroban_sdk::contractimport!(
		file = "../atomic_swap/target/wasm32-unknown-unknown/release/soroban_atomic_swap_contract.wasm"
	);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapSpec {
    pub address: Address,
    pub amount: i128,
    pub min_recv: i128,
}

#[contract]
pub struct AtomicMultiSwapContract;

#[contractimpl]
impl AtomicMultiSwapContract {
    pub fn multi_swap(env: Env, swap_contract: Address, token_a: Address, token_b: Address, swaps_a: Vec<SwapSpec>, swaps_b: Vec<SwapSpec>)  {
        let mut swap_client = atomic_swap::Client::new(&env, &swap_contract);
        let mut acc_a = start(swaps_a.iter());
        if !iteration_finished {
        if CHECK_CONDITION_ASSIGNMENT_9 {
            let mut RANGE_END_20 = swaps_b.len();
            let mut ITERATOR_17 = 0..RANGE_END_20;
            let mut i = start(&ITERATOR_17);
            if !iteration_finished {
            if CHECK_CONDITION_ASSIGNMENT_18 {
                let mut acc_b = swaps_b.get(&i).unwrap();
                let BINARY_EXPRESSION_LEFT_38 = acc_a.amount >= acc_b.min_recv;
                let BINARY_EXPRESSION_RIGHT_39 = acc_a.min_recv <= acc_b.amount;
                let CONDITIONAL_JUMP_ASSIGNMENT_37 = BINARY_EXPRESSION_LEFT_38 && BINARY_EXPRESSION_RIGHT_39;
                if CONDITIONAL_JUMP_ASSIGNMENT_37 {
                    let mut CONDITIONAL_JUMP_ASSIGNMENT_65 = swap_client.try_swap(&acc_a.address, &acc_b.address, &token_a, &token_b, &acc_a.amount, &acc_a.min_recv, &acc_b.amount, &acc_b.min_recv).is_ok();
                    if CONDITIONAL_JUMP_ASSIGNMENT_65 {
                        swaps_b.remove(&i);
                        else {
                    }
                    else {
                }
                increment: i
                goto: 27
            }
            increment: acc_a
            goto: 14
        }
    }
}



mod test;
