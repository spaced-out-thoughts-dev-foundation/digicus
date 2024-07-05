#![no_std]
use soroban_sdk::{contract, contracttype, token, Address, Vec, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

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
    pub timestamp: i128,
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
        let CONDITIONAL_JUMP_ASSIGNMENT_0 = claimants.len() > 10;
        if CONDITIONAL_JUMP_ASSIGNMENT_0 {
            panic!("too many claimants");
        }
        let mut CONDITIONAL_JUMP_ASSIGNMENT_12 = is_initialized(&env);
        if CONDITIONAL_JUMP_ASSIGNMENT_12 {
            panic!("contract has been already initialized");
        }
        from.require_auth();
        let mut METHOD_CALL_EXPRESSION_33 = token::Client::new(&env, &token);
        METHOD_CALL_EXPRESSION_33.transfer(&from, &env.current_contract_address(), &amount);
        let mut METHOD_CALL_ARG_2_44 = ClaimableBalance{token: token, amount: amount, claimants: time_bound, time_bound: claimants};
        env.storage().instance().set(&DataKey::Balance, &METHOD_CALL_ARG_2_44);
        let mut METHOD_CALL_ARG_2_60 = ();
        env.storage().instance().set(&DataKey::Init, &METHOD_CALL_ARG_2_60);
    }


    pub fn claim(env: Env, claimant: Address)  {
        claimant.require_auth();
        let mut claimable_balance:ClaimableBalance = env.storage().instance().get(&DataKey::Balance).unwrap();
        let CONDITIONAL_JUMP_ASSIGNMENT_84 = !(check_time_bound(&env, &claimable_balance.time_bound));
        if CONDITIONAL_JUMP_ASSIGNMENT_84 {
            panic!("time predicate is not fulfilled");
        }
        let CONDITIONAL_JUMP_ASSIGNMENT_105 = !(claimants.contains(&claimant));
        if CONDITIONAL_JUMP_ASSIGNMENT_105 {
            panic!("claimant is not allowed to claim this balance");
        }
        let mut METHOD_CALL_EXPRESSION_128 = token::Client::new(&env, &claimable_balance.token);
        METHOD_CALL_EXPRESSION_128.transfer(&env.current_contract_address(), &claimant, &claimable_balance.amount);
        env.storage().instance().remove(&DataKey::Balance);
    }
}

pub fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
        let Thing_to_return: bool;
    let mut ledger_timestamp = env.ledger().timestamp();
    let CONDITIONAL_JUMP_CHECK_10 = time_bound.kind == TimeBoundKind::Before;
    if CONDITIONAL_JUMP_CHECK_10 {
    let CONDITIONAL_JUMP_CHECK_23 = time_bound(&kind) == TimeBoundKind::After;
    if CONDITIONAL_JUMP_CHECK_23 {
    let Thing_to_return = ledger_timestamp <= time_bound.timestamp;
    else {
    let Thing_to_return = ledger_timestamp >= time_bound.timestamp;
    else {
}
}
    return Thing_to_return;
}


pub fn is_initialized(env: &Env) -> bool {
        let Thing_to_return: bool;
    Thing_to_return = env.storage().instance().has(&DataKey::Init);
    return Thing_to_return;
}



mod test;
