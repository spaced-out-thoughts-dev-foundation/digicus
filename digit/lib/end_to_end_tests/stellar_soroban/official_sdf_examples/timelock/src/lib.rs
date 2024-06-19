#![no_std]
use soroban_sdk::{contract, contracttype, Vec, Env, contractimpl};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Init,\n    Balance,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeBoundKind {
    Before,\n    After,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeBound {kind: TimeBoundKind, timestamp: i64}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimableBalance {token: Address, amount: Bigi64, claimants: Vec<Address>, time_bound: TimeBound}

#[contract]
pub struct ClaimableBalanceContract;


    pub fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
        let mut ledger_timestamp = env.ledger().timestamp();
        let mut CONDITIONAL_JUMP_CHECK_100 = &equal_to(&time_bound(kind), &TimeBoundKind::Before);

                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut Thing_to_return = &less_than_or_equal_to(&ledger_timestamp, &time_bound(timestamp));
        let mut CONDITIONAL_JUMP_CHECK_200 = &equal_to(&Thing_to_compare_against, &TimeBoundKind::After);

                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut Thing_to_return = &greater_than_or_equal_to(&ledger_timestamp, &time_bound(timestamp));
        Thing_to_return
    }


    pub fn is_initialized(env: &Env) -> bool {
        let mut Thing_to_return = env.storage().instance().has(&DataKey::Init);
        Thing_to_return
    }

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(env: Env, from: Address, token: Address, amount: Bigi64, claimants: Vec<Address>, time_bound: TimeBound)  {
        let mut CONDITIONAL_JUMP_ASSIGNMENT = &greater_than(claimants.len(), 10);

                panic! "too many claimants";
        let mut CONDITIONAL_JUMP_ASSIGNMENT = &is_initialized(&env);

                panic! "contract has been already initialized";
        &from.require_auth();
        let mut METHOD_CALL_EXPRESSION_7 = &token::Client::new(&env, &token);
        &METHOD_CALL_EXPRESSION_7.transfer(&from, env.current_contract_address(), &amount);
        let mut METHOD_CALL_EXPRESSION_11 = env.storage().instance();
        let mut token = token;
        let mut amount = amount;
        let mut time_bound = time_bound;
        let mut claimants = claimants;
        &METHOD_CALL_EXPRESSION_11.set(&DataKey::Balance, &UDT(ClaimableBalance, token, amount, time_bound, claimants));
        env.storage().instance().set(&DataKey::Init, &Tuple());
    }


    pub fn claim(env: Env, claimant: Address)  {
        &claimant.require_auth();
        let mut claimable_balance = env.storage().instance().get(&DataKey::Balance).unwrap();
        let mut CONDITIONAL_JUMP_ASSIGNMENT = &!(&check_time_bound(env, claimable_balance(time_bound)));

                panic! "time predicate is not fulfilled";
        let mut CONDITIONAL_JUMP_ASSIGNMENT = &!(claimable_balance(&claimants).contains(&claimant));

                panic! "claimant is not allowed to claim this balance";
        let mut METHOD_CALL_EXPRESSION_15 = &token::Client::new(&env, &claimable_balance(token));
        &METHOD_CALL_EXPRESSION_15.transfer(env.current_contract_address(), &claimant, &claimable_balance(amount));
        env.storage().instance().remove(&DataKey::Balance);
    }
}


mod test;
