#![no_std]
use soroban_sdk::{contract, contracttype, token, Address, Vec, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};



mod proptest {
}

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
        let mut CONDITIONAL_JUMP_ASSIGNMENT_0 = claimants.is_empty();
        if CONDITIONAL_JUMP_ASSIGNMENT_0 {
            panic!("need more than 0 claimants");
        }
        let CONDITIONAL_JUMP_ASSIGNMENT_8 = claimants.len() > 10;
        if CONDITIONAL_JUMP_ASSIGNMENT_8 {
            panic!("too many claimants");
        }
        let mut CONDITIONAL_JUMP_ASSIGNMENT_20 = is_initialized(&env);
        if CONDITIONAL_JUMP_ASSIGNMENT_20 {
            panic!("contract has been already initialized");
        }
        from.require_auth();
        let mut METHOD_CALL_EXPRESSION_41 = token::Client::new(&env, &token);
        METHOD_CALL_EXPRESSION_41.transfer(&from, &env.current_contract_address(), &amount);
        let mut METHOD_CALL_ARG_2_52 = ClaimableBalance{token: token, amount: amount, claimants: time_bound, time_bound: claimants};
        env.storage().persistent().set(&DataKey::Balance, &METHOD_CALL_ARG_2_52);
        let mut METHOD_CALL_ARG_2_68 = ();
        env.storage().persistent().set(&DataKey::Init, &METHOD_CALL_ARG_2_68);
    }


    pub fn claim(env: Env, claimant: Address, amount: i128)  {
        claimant.require_auth();
        let mut claimable_balance:ClaimableBalance = env.storage().persistent().get(&DataKey::Balance).unwrap();
        let CONDITIONAL_JUMP_ASSIGNMENT_92 = !(check_time_bound(&env, &claimable_balance.time_bound));
        if CONDITIONAL_JUMP_ASSIGNMENT_92 {
            panic!("time predicate is not fulfilled");
        }
        let CONDITIONAL_JUMP_ASSIGNMENT_113 = !(claimants.contains(&claimant));
        if CONDITIONAL_JUMP_ASSIGNMENT_113 {
            panic!("claimant is not allowed to claim this balance");
        }
        let CONDITIONAL_JUMP_ASSIGNMENT_125 = amount > claimable_balance.amount;
        if CONDITIONAL_JUMP_ASSIGNMENT_125 {
            panic!("claimed amount greater than balance");
        }
        let mut METHOD_CALL_EXPRESSION_146 = token::Client::new(&env, &claimable_balance.token);
        METHOD_CALL_EXPRESSION_146.transfer(&env.current_contract_address(), &claimant, &amount);
        let mut new_balance = claimable_balance.amount - amount;
        let CONDITIONAL_JUMP_ASSIGNMENT_166 = new_balance > 0;
        if CONDITIONAL_JUMP_ASSIGNMENT_166 {
        else {
            claimable_balance.amount = new_balance;
            env.storage().persistent().set(&DataKey::Balance, &claimable_balance);
            else {
                env.storage().persistent().remove(&DataKey::Balance);
                else {
            }
        }
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
    Thing_to_return = env.storage().persistent().has(&DataKey::Init);
    return Thing_to_return;
}



mod test;
