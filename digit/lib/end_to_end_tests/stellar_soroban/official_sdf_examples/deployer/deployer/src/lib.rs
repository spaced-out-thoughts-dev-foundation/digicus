#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, BytesN, Symbol, Vec, Val, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct Deployer;

#[contractimpl]
impl Deployer {
    pub fn deploy(env: Env, deployer: Address, wasm_hash: BytesN<32>, salt: BytesN<32>, init_fn: Symbol, init_args: Vec<Val>) -> (Address, Val) {
        let Thing_to_return: (Address, Val);
        let CONDITIONAL_JUMP_ASSIGNMENT_1 = deployer != env.current_contract_address();
        if CONDITIONAL_JUMP_ASSIGNMENT_1 {
            Thing_to_return = deployer.require_auth();
            else {
        }
        let mut deployed_address = env.deployer().with_address(deployer, salt).deploy(wasm_hash);
        let mut res:Val = env.invoke_contract(&deployed_address, &init_fn, &init_args);
        let mut Thing_to_return = (deployed_address, res);
        return Thing_to_return;
    }
}



mod test;
