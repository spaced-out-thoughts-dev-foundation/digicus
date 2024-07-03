#![no_std]
use soroban_sdk::{contract, contracterror, contracttype, Address, contractimpl, Env, auth::Context, IntoVal};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    NotAuthorizedMinter(1),
    DailyLimitInsufficient(2),
    NegativeAmount(3),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    Admin(),
    Minter(Address,  Address),
    MinterStats(Address,  Address,  i64,  i64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinterConfig {
    pub limit: i128,
    pub epoch_length: i64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinterStats {
    pub consumed_limit: i128,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn set_admin(env: Env, new_admin: Address)  {
        let mut INPUT_VALUE_NAME_FOR_LET_0 = env.storage().instance().get(&StorageKey::Admin);
        if INPUT_VALUE_NAME_FOR_LET_0 {
            admin.require_auth();
        }
        env.storage().instance().set(&StorageKey::Admin, &new_admin);
    }


    pub fn admin(env: Env) -> Address {
        let Thing_to_return: Address;
        Thing_to_return = env.storage().instance().get(&StorageKey::Admin).unwrap();
        Thing_to_return
    }


    pub fn set_minter(env: Env, contract: Address, minter: Address, config: MinterConfig)  {
        admin(env.clone()).require_auth();
        let mut METHOD_CALL_EXPRESSION_6 = env.storage().persistent();
        METHOD_CALL_EXPRESSION_6.set(&StorageKey::Minter(&contract, &minter), &config);
    }


    pub fn minter(env: Env, contract: Address, minter: Address) -> Result<MinterConfig, i64, MinterStats, Error> {
        let Thing_to_return: Result<MinterConfig, i64, MinterStats, Error>;
        let mut METHOD_CALL_EXPRESSION_6 = env.storage().persistent();
        let mut CALL_EXPRESSION_ARG_1 = contract.clone();
        let mut config = METHOD_CALL_EXPRESSION_6.get(StorageKey::Minter(&CALL_EXPRESSION_ARG_1, minter.clone()).ok_or(&Error::NotAuthorizedMinter);
        let mut BINARY_EXPRESSION_LEFT = env.ledger().sequence();
        let mut BINARY_EXPRESSION_RIGHT = config.epoch_length;
        epoch = BINARY_EXPRESSION_LEFT / BINARY_EXPRESSION_RIGHT;
        let mut METHOD_CALL_EXPRESSION_16 = env.storage().temporary();
        let mut CALL_EXPRESSION_ARG_1 = contract.clone();
        let mut stats = METHOD_CALL_EXPRESSION_16.get(StorageKey::MinterStats(&CALL_EXPRESSION_ARG_1, minter.clone(), config.epoch_length()).unwrap_or_default();
        let mut CALL_EXPRESSION_ARG_1 = (config, epoch, stats);
        Thing_to_return = Ok(CALL_EXPRESSION_ARG_1);
        Thing_to_return
    }


    pub fn mint(env: Env, contract: Address, minter: Address, to: Address, amount: i128) -> Error {
        let Thing_to_return: Error;
        let mut METHOD_CALL_EXPRESSION_2 = (contract, to, amount);
        minter.require_auth_for_args(METHOD_CALL_EXPRESSION_2.into_val(&env));
        let CONDITIONAL_JUMP_ASSIGNMENT = amount < 0;
        if CONDITIONAL_JUMP_ASSIGNMENT {
            let mut RETURN_VALUE_LABEL = Err(Error::NegativeAmount);
            RETURN_VALUE_LABEL
        }
        let mut admin = Self::admin(env.clone());
        let mut CONDITIONAL_JUMP_ASSIGNMENT = not_equal_to(&admin, &minter);
        if CONDITIONAL_JUMP_ASSIGNMENT {
            let mut METHOD_CALL_EXPRESSION_11 = env.storage().persistent();
            let mut CALL_EXPRESSION_ARG_1 = contract.clone();
            let mut TupleStruct = METHOD_CALL_EXPRESSION_11.get(StorageKey::Minter(&CALL_EXPRESSION_ARG_1, minter.clone());
            let mut BINARY_EXPRESSION_LEFT = env.ledger().sequence();
            let mut BINARY_EXPRESSION_RIGHT = config.epoch_length;
            epoch = BINARY_EXPRESSION_LEFT / BINARY_EXPRESSION_RIGHT;
            let mut CALL_EXPRESSION_ARG_1 = contract.clone();
            let mut minter_stats_key = StorageKey::MinterStats(&CALL_EXPRESSION_ARG_1, minter.clone(), config.epoch_length(), &epoch);
            let mut minter_stats = env.storage().temporary().get(&minter_stats_key).unwrap_or_default();
            let mut BINARY_EXPRESSION_LEFT = minter_stats.consumed_limit;
            consumed_limit = BINARY_EXPRESSION_LEFT + amount;
            let mut new_minter_stats = MinterStats{consumed_limit: consumed_limit};
            let mut BINARY_EXPRESSION_LEFT = new_minter_stats.consumed_limit;
            let mut CONDITIONAL_JUMP_ASSIGNMENT = greater_than(&BINARY_EXPRESSION_LEFT, config.limit());
            if CONDITIONAL_JUMP_ASSIGNMENT {
                let mut RETURN_VALUE_LABEL = Err(Error::DailyLimitInsufficient);
                RETURN_VALUE_LABEL
            }
            env.storage().temporary().set(&minter_stats_key, &new_minter_stats);
            let mut METHOD_CALL_EXPRESSION_33 = env.storage().temporary();
            let mut BINARY_EXPRESSION_RIGHT = config.epoch_length;
            METHOD_CALL_ARG_3_32 = epoch * BINARY_EXPRESSION_RIGHT;
            Thing_to_return = METHOD_CALL_EXPRESSION_33.extend_ttl(&minter_stats_key, 0, &METHOD_CALL_ARG_3_32);
        }
        let mut client = MintClient::new(&env, &contract);
        client.mint(&to, &amount);
        let mut CALL_EXPRESSION_ARG_1 = ();
        Thing_to_return = Ok(CALL_EXPRESSION_ARG_1);
        Thing_to_return
    }
}



mod test;
