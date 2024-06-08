#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contractclient(name = "AnswerToLifeContractWithInterfaceTraitIgnoredClient")]
trait AnswerToLifeContractWithInterfaceTraitIgnoredInterface {
    fn fourty_two(env: Env);
}

#[contract]
pub struct AnswerToLifeContractWithInterfaceTraitIgnored;

#[contractimpl]
impl AnswerToLifeContractWithInterfaceTraitIgnored {
    pub fn fourty_two(env: Env) -> u32 {
        42
    }
}
