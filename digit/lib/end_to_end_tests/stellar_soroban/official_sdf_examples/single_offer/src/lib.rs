#![no_std]
use soroban_sdk::{contract, contracttype, Address, token, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Offer,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Offer {
    pub seller: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_price: i128,
    pub buy_price: i128,
}

#[contract]
pub struct SingleOffer;

#[contractimpl]
impl SingleOffer {
    pub fn create(e: Env, seller: Address, sell_token: Address, buy_token: Address, sell_price: i128, buy_price: i128)  {
        let mut CONDITIONAL_JUMP_ASSIGNMENT_0 = e.storage().instance().has(&DataKey::Offer);
        if CONDITIONAL_JUMP_ASSIGNMENT_0 {
            panic!("offer is already created");
        }
        let BINARY_EXPRESSION_LEFT_15 = buy_price == 0;
        let BINARY_EXPRESSION_RIGHT_16 = sell_price == 0;
        let CONDITIONAL_JUMP_ASSIGNMENT_14 = BINARY_EXPRESSION_LEFT_15 || BINARY_EXPRESSION_RIGHT_16;
        if CONDITIONAL_JUMP_ASSIGNMENT_14 {
            panic!("zero price is not allowed");
        }
        seller.require_auth();
        let mut CALL_EXPRESSION_ARG_2_37 = Offer{seller: seller, sell_token: sell_token, buy_token: buy_token, sell_price: sell_price, buy_price: buy_price};
        write_offer(&e, &CALL_EXPRESSION_ARG_2_37);
    }


    pub fn trade(e: Env, buyer: Address, buy_token_amount: i128, min_sell_token_amount: i128)  {
        buyer.require_auth();
        let mut offer = load_offer(&e);
        let mut sell_token_client = token::Client::new(&e, &offer.sell_token);
        let mut buy_token_client = token::Client::new(&e, &offer.buy_token);
        let mut BINARY_EXPRESSION_LEFT_76 = buy_token_amount.checked_mul(offer.sell_price).unwrap_optimized();
        let mut sell_token_amount = BINARY_EXPRESSION_LEFT_76 / offer.buy_price;
        let CONDITIONAL_JUMP_ASSIGNMENT_93 = sell_token_amount < min_sell_token_amount;
        if CONDITIONAL_JUMP_ASSIGNMENT_93 {
            panic!("price is too low");
        }
        let mut contract = e.current_contract_address();
        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        buy_token_client.transfer(&contract, &offer.seller, &buy_token_amount);
    }


    pub fn withdraw(e: Env, token: Address, amount: i128)  {
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        let mut METHOD_CALL_EXPRESSION_158 = token::Client::new(&e, &token);
        METHOD_CALL_EXPRESSION_158.transfer(&e.current_contract_address(), &offer.seller, &amount);
    }


    pub fn updt_price(e: Env, sell_price: i128, buy_price: i128)  {
        let BINARY_EXPRESSION_LEFT_168 = buy_price == 0;
        let BINARY_EXPRESSION_RIGHT_169 = sell_price == 0;
        let CONDITIONAL_JUMP_ASSIGNMENT_167 = BINARY_EXPRESSION_LEFT_168 || BINARY_EXPRESSION_RIGHT_169;
        if CONDITIONAL_JUMP_ASSIGNMENT_167 {
            panic!("zero price is not allowed");
        }
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        offer.sell_price = sell_price;
        offer.buy_price = buy_price;
        write_offer(&e, &offer);
    }


    pub fn get_offer(e: Env) -> Offer {
        let Thing_to_return: Offer;
        Thing_to_return = load_offer(&e);
        return Thing_to_return;
    }
}

pub fn load_offer(e: &Env) -> Offer {
        let Thing_to_return: Offer;
    Thing_to_return = e.storage().instance().get(&DataKey::Offer).unwrap();
    return Thing_to_return;
}


pub fn write_offer(e: &Env, offer: &Offer)  {
    e.storage().instance().set(&DataKey::Offer, offer);
}



mod test;
