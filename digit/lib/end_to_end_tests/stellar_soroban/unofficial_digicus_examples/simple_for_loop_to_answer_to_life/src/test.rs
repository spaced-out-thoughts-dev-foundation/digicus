#![cfg(test)]

use super::{AnswerToLifeContract, AnswerToLifeContractClient};
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AnswerToLifeContract);
    let client = AnswerToLifeContractClient::new(&env, &contract_id);

    assert_eq!(client.fourty_two(), 42);
}
