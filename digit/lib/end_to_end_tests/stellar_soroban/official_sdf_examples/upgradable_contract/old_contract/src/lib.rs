#![no_std]
use soroban_sdk::{contract, contracttype, contracterror, contractimpl, Env, Address, BytesN, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    AlreadyInitialized = 1,
}

#[contract]
pub struct UpgradeableContract;

#[contractimpl]
impl UpgradeableContract {
    pub fn init(e: Env, admin: Address) -> Result<(), Error> {
        let Thing_to_return: Result<(), Error>;
        let mut CONDITIONAL_JUMP_ASSIGNMENT_0 = e.storage().instance().has(&DataKey::Admin);
        if CONDITIONAL_JUMP_ASSIGNMENT_0 {
            let mut RETURN_VALUE_LABEL_12 = Err(Error::AlreadyInitialized);
            return RETURN_VALUE_LABEL_12;
        }
        e.storage().instance().set(&DataKey::Admin, &admin);
        let mut CALL_EXPRESSION_ARG_1 = ();
        Thing_to_return = Ok(CALL_EXPRESSION_ARG_1);
        return Thing_to_return;
    }


    pub fn version() -> i128 {
        let Thing_to_return: i128;
        return 1;
    }


    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>)  {
        let mut admin:Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}



mod test;
