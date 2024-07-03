#![no_std]
use soroban_sdk::{contract, contracttype, Address, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Counter(Address),
}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, user: Address, value: i128) -> i128 {
        let Thing_to_return: i128;
        user.require_auth();
        let mut key = DataKey::Counter(user.clone());
        let mut count:i128 = env.storage().persistent().get(&key).unwrap_or_default();
        let mut count = count + value;
        env.storage().persistent().set(&key, &count);
        return count;
    }
}



mod test;
