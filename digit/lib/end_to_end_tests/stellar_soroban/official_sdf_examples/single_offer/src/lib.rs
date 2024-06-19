#![no_std]
use soroban_sdk::{contract, contracttype, Env, contractimpl};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Offer,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Offer {seller: Address, sell_token: Address, buy_token: Address, sell_price: i64, buy_price: i64}

#[contract]
pub struct SingleOffer;


    pub fn load_offer(e: &Env) -> Offer {
        let mut Thing_to_return = e.storage().instance().get(&DataKey::Offer).unwrap();
        Thing_to_return
    }


    pub fn write_offer(e: &Env, offer: &Offer)  {
        e.storage().instance().set(&DataKey::Offer, &offer);
    }

#[contractimpl]
impl SingleOffer {
    pub fn create(e: Env, seller: Address, sell_token: Address, buy_token: Address, sell_price: i64, buy_price: i64)  {
        let mut CONDITIONAL_JUMP_ASSIGNMENT = e.storage().instance().has(&DataKey::Offer);

                panic! "offer is already created";
        let mut BINARY_EXPRESSION_LEFT = &equal_to(&buy_price, 0);
        let mut BINARY_EXPRESSION_RIGHT = &equal_to(&sell_price, 0);
        let CONDITIONAL_JUMP_ASSIGNMENT = sell_price || BINARY_EXPRESSION_RIGHT;

                panic! "zero price is not allowed";
        &seller.require_auth();
        let mut seller = seller;
        let mut sell_token = sell_token;
        let mut buy_token = buy_token;
        let mut sell_price = sell_price;
        let mut buy_price = buy_price;
        &write_offer(&e, &UDT(Offer, seller, sell_token, buy_token, sell_price, buy_price));
    }


    pub fn trade(e: Env, buyer: Address, buy_token_amount: Bigi64, min_sell_token_amount: Bigi64)  {
        &buyer.require_auth();
        let mut offer = &load_offer(&e);
        let mut sell_token_client = &token::Client::new(&e, &offer(sell_token));
        let mut buy_token_client = &token::Client::new(&e, &offer(buy_token));
        let mut BINARY_EXPRESSION_LEFT = buy_token_amount.checked_mul(&offer(sell_price).unwrap_optimized();
        let mut BINARY_EXPRESSION_RIGHT = offer.buy_price;
        let mut CONDITIONAL_JUMP_ASSIGNMENT = &less_than(&BINARY_EXPRESSION_LEFT(BINARY_EXPRESSION_RIGHT), &min_sell_token_amount);

                panic! "price is too low";
        &buy_token_client.transfer(&buyer, e.current_contract_address(), &buy_token_amount);
        &sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        &buy_token_client.transfer(&contract, &offer(seller), &buy_token_amount);
    }


    pub fn withdraw(e: Env, token: Address, amount: Bigi64)  {
        let mut offer = &load_offer(&e);
        offer(&seller).require_auth();
        let mut METHOD_CALL_EXPRESSION_6 = &token::Client::new(&e, &token);
        &METHOD_CALL_EXPRESSION_6.transfer(e.current_contract_address(), &offer(seller), &amount);
    }


    pub fn updt_price(e: Env, sell_price: i64, buy_price: i64)  {
        let mut BINARY_EXPRESSION_LEFT = &equal_to(&buy_price, 0);
        let mut BINARY_EXPRESSION_RIGHT = &equal_to(&sell_price, 0);
        let CONDITIONAL_JUMP_ASSIGNMENT = sell_price || BINARY_EXPRESSION_RIGHT;

                panic! "zero price is not allowed";
        let mut offer = &load_offer(&e);
        offer(&seller).require_auth();
        let mut ASSIGN_EXPRESSION_LEFT = offer.sell_price;
        let mut ASSIGN_EXPRESSION_LEFT = offer.buy_price;
        &write_offer(&e, &offer);
    }


    pub fn get_offer(e: Env) -> Offer {
        let mut Thing_to_return = &load_offer(&e);
        Thing_to_return
    }
}


mod test;
