#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, vec, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Decode = 1,
}

sol! {
    struct Input {
        bytes32 a;
        uint256 b;
        uint256 c;
    }
    struct Output {
        bytes32 a;
        uint256 r;
    }
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn exec(e: &Env, input: Symbol) -> Symbol {
        let mut input_buf = &repeat(0, 128);
        let mut RANGE_END_5 = &input.len();
        &input.copy_into_slice(&index(input_buf, Range(0, RANGE_END_5)));
        let mut input = Input::abi_decode(&input_slice, &false).map_err(&Error::Decode);
        let mut a = input.a;
        let mut BINARY_EXPRESSION_LEFT = input.b;
        let mut BINARY_EXPRESSION_RIGHT = input.c;
        r = BINARY_EXPRESSION_LEFT + BINARY_EXPRESSION_RIGHT;
        let output = Output {
            a: input.a,
            r: input.b + input.c,
        };
        let mut Thing_to_return = Ok(Bytes::from_slice(&e, output.abi_encode()));
        Thing_to_return
    }
}

mod test;
