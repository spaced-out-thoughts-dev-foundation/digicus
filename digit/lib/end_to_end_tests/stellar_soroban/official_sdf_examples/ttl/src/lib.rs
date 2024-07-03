#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    MyKey,
}

#[contract]
pub struct TtlContract;

#[contractimpl]
impl TtlContract {
    pub fn setup(env: Env)  {
        env.storage().persistent().set(&DataKey::MyKey, &0);
        env.storage().instance().set(&DataKey::MyKey, &1);
        env.storage().temporary().set(&DataKey::MyKey, &2);
    }


    pub fn extend_persistent(env: Env)  {
        env.storage().persistent().extend_ttl(&DataKey::MyKey, 1000, 5000);
    }


    pub fn extend_instance(env: Env)  {
        env.storage().instance().extend_ttl(2000, 10000);
    }


    pub fn extend_temporary(env: Env)  {
        env.storage().temporary().extend_ttl(&DataKey::MyKey, 3000, 7000);
    }
}



mod test;
