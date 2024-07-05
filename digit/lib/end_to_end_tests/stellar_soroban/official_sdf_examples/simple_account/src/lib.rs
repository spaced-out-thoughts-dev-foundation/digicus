#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, BytesN, Vec, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Owner,
}

#[contract]
pub struct SimpleAccount;

#[contractimpl]
impl SimpleAccount {
    pub fn init(env: Env, public_key: BytesN<32>)  {
        let mut CONDITIONAL_JUMP_ASSIGNMENT_1 = env.storage().instance().has(&DataKey::Owner);
        if CONDITIONAL_JUMP_ASSIGNMENT_1 {
            panic!("owner is already set");
        }
        env.storage().instance().set(&DataKey::Owner, &public_key);
    }


    pub fn __check_auth(env: Env, signature_payload: BytesN<32>, signature: BytesN<64>, _auth_context: Vec<Context>)  {
        let mut public_key:BytesN<32> = env.storage().instance().get(&DataKey::Owner).unwrap();
        let mut METHOD_CALL_EXPRESSION_20 = env.crypto();
        METHOD_CALL_EXPRESSION_20.ed25519_verify(&public_key, &signature_payload.into(), &signature);
    }
}



mod test;
