#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataKey {MyKey: ()}

#[contract]
pub struct TtlContract;

#[contractimpl]
impl TtlContract {
    pub fn setup(env: Env)  {
        let mut METHOD_CALL_EXPRESSION_3 = env.storage();
        let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_3.persistent();
        METHOD_CALL_EXPRESSION_2.set(&DataKey::MyKey, 0);
        let mut METHOD_CALL_EXPRESSION_3 = env.storage();
        let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_3.instance();
        METHOD_CALL_EXPRESSION_2.set(&DataKey::MyKey, 1);
        let mut METHOD_CALL_EXPRESSION_3 = env.storage();
        let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_3.temporary();
        METHOD_CALL_EXPRESSION_2.set(&DataKey::MyKey, 2);
    }


    pub fn extend_persistent(env: Env)  {
        let mut METHOD_CALL_EXPRESSION_4 = env.storage();
        let mut METHOD_CALL_EXPRESSION_3 = METHOD_CALL_EXPRESSION_4.persistent();
        METHOD_CALL_EXPRESSION_3.extend_ttl(&DataKey::MyKey, 1000, 5000);
    }


    pub fn extend_instance(env: Env)  {
        let mut METHOD_CALL_EXPRESSION_3 = env.storage();
        let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_3.instance();
        METHOD_CALL_EXPRESSION_2.extend_ttl(2000, 10000);
    }


    pub fn extend_temporary(env: Env)  {
        let mut METHOD_CALL_EXPRESSION_4 = env.storage();
        let mut METHOD_CALL_EXPRESSION_3 = METHOD_CALL_EXPRESSION_4.temporary();
        METHOD_CALL_EXPRESSION_3.extend_ttl(&DataKey::MyKey, 3000, 7000);
    }
}


mod test;
