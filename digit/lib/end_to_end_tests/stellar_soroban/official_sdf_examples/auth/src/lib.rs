#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Counter,
}

#[contract]
pub struct IncrementContract;


#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, user: Address, value: i64) -> i64 {
        &user.require_auth();
        let mut key = &DataKey::Counter(user.clone());
        let mut count = env.storage().persistent().get(&key).unwrap_or_default();
        count = count + value;
        env.storage().persistent().set(&key, &count);
        count
    }
}


mod test;
