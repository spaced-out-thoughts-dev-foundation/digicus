#![no_std]
use soroban_sdk::{contract, contracttype, token, Address, Vec, contractimpl, Env, auth::Context, IntoVal};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Init,
    Balance,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeBoundKind {
    Before,
    After,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: i64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contract]
pub struct ClaimableBalanceContract;

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(env: Env, from: Address, token: Address, amount: i128, claimants: Vec<Address>, time_bound: TimeBound)  {
        let CONDITIONAL_JUMP_ASSIGNMENT = claimants.len() > 10;
        if CONDITIONAL_JUMP_ASSIGNMENT {
            panic!("too many claimants");
        }
        let mut CONDITIONAL_JUMP_ASSIGNMENT = is_initialized(&env);
        if CONDITIONAL_JUMP_ASSIGNMENT {
            panic!("contract has been already initialized");
        }
        from.require_auth();
        let mut METHOD_CALL_EXPRESSION_7 = token::Client::new(&env, &token);
        METHOD_CALL_EXPRESSION_7.transfer(&from, env.current_contract_address(), &amount);
        let mut METHOD_CALL_ARG_2_10 = ClaimableBalance{token: token, amount: amount, claimants: time_bound, time_bound: claimants};
        env.storage().instance().set(&DataKey::Balance, &METHOD_CALL_ARG_2_10);
        let mut METHOD_CALL_ARG_2_15 = ();
        env.storage().instance().set(&DataKey::Init, &METHOD_CALL_ARG_2_15);
    }


    pub fn claim(env: Env, claimant: Address)  {
        claimant.require_auth();
        let mut claimable_balance = env.storage().instance().get(&DataKey::Balance).unwrap();
        let CONDITIONAL_JUMP_ASSIGNMENT = !(check_time_bound(&env, &claimable_balance.time_bound));
        if CONDITIONAL_JUMP_ASSIGNMENT {
            panic!("time predicate is not fulfilled");
        }
        let CONDITIONAL_JUMP_ASSIGNMENT = !(claimants.contains(&claimant));
        if CONDITIONAL_JUMP_ASSIGNMENT {
            panic!("claimant is not allowed to claim this balance");
        }
        let mut METHOD_CALL_EXPRESSION_15 = token::Client::new(&env, &claimable_balance.token);
        METHOD_CALL_EXPRESSION_15.transfer(env.current_contract_address(), &claimant, &claimable_balance.amount);
        env.storage().instance().remove(&DataKey::Balance);
    }
}

pub fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
        let Thing_to_return: bool;
    let mut ledger_timestamp = env.ledger().timestamp();
    let mut Thing_to_compare_against = time_bound.kind;
    let CONDITIONAL_JUMP_CHECK_100 = time_bound.kind == TimeBoundKind::Before;
    if CONDITIONAL_JUMP_CHECK_100 {
    let Thing_to_return = ledger_timestamp <= time_bound.timestamp;
}
    let CONDITIONAL_JUMP_CHECK_200 = Thing_to_compare_against == TimeBoundKind::After;
    if CONDITIONAL_JUMP_CHECK_200 {
    let Thing_to_return = ledger_timestamp >= time_bound.timestamp;
}
    Thing_to_return
}


pub fn is_initialized(env: &Env) -> bool {
        let Thing_to_return: bool;
    Thing_to_return = env.storage().instance().has(&DataKey::Init);
    Thing_to_return
}



mod test;
