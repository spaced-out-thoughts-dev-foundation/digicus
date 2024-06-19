#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Owner,
}

#[contract]
pub struct SimpleAccount;


#[contractimpl]
impl SimpleAccount {
    pub fn init(env: Env, public_key: Symbol)  {
        let mut CONDITIONAL_JUMP_ASSIGNMENT = env.storage().instance().has(&DataKey::Owner);

                panic! "owner is already set";
        env.storage().instance().set(&DataKey::Owner, &public_key);
    }


    pub fn __check_auth(env: Env, signature_payload: Symbol, signature: Symbol, _auth_context: Vec<Context>)  {
        let mut public_key = env.storage().instance().get(&DataKey::Owner).unwrap();
        let mut METHOD_CALL_EXPRESSION_9 = &env.crypto();
        &METHOD_CALL_EXPRESSION_9.ed25519_verify(&public_key, signature_payload.into(), &signature);
    }
}


mod test;
