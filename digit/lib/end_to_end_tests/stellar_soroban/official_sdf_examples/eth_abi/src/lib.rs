#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct  {}

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
        let mut Thing_to_return = &Ok(Bytes::from_slice(&e, &UDT(Output, &a, &r).abi_encode());
        Thing_to_return
    }
}


mod test;
