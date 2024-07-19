const contractDictionary = {
    "hello_world_logging": `#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, answer_to_life: i32, value: Symbol) {
        if answer_to_life != 42 {
            panic!("Not the answer to life!");
        }

        log!(&env, "Hello {}", value);
    }
}
`, "increment": `
#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        // The contract instance will be bumped to have a lifetime of at least 100 ledgers if the current expiration lifetime at most 50.
        // If the lifetime is already more than 100 ledgers, this is a no-op. Otherwise,
        // the lifetime is extended to 100 ledgers. This lifetime bump includes the contract
        // instance itself and all entries in storage().instance(), i.e, COUNTER.
        env.storage().instance().extend_ttl(50, 100);

        // Return the count to the caller.
        count
    }
}

mod test;`, "logging": `
#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, value: String) {
        log!(&env, "Hello {}", value);
    }
}

mod test;
`, "single_offer": `
//! This contract implements trading of one token pair between one seller and
//! multiple buyer.
//! It demonstrates one of the ways of how trading might be implemented.
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, unwrap::UnwrapOptimized, Address, Env,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer,
}

// Represents an offer managed by the SingleOffer contract.
// If a seller wants to sell 1000 XLM for 100 USDC the \`sell_price\` would be 1000
// and \`buy_price\` would be 100 (or 100 and 10, or any other pair of integers
// in 10:1 ratio).
#[derive(Clone)]
#[contracttype]
pub struct Offer {
    // Owner of this offer. Sells sell_token to get buy_token.
    pub seller: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    // Seller-defined price of the sell token in arbitrary units.
    pub sell_price: u32,
    // Seller-defined price of the buy token in arbitrary units.
    pub buy_price: u32,
}

#[contract]
pub struct SingleOffer;

/*
How this contract should be used:

1. Call \`create\` once to create the offer and register its seller.
2. Seller may transfer arbitrary amounts of the \`sell_token\` for sale to the
   contract address for trading. They may also update the offer price.
3. Buyers may call \`trade\` to trade with the offer. The contract will
   immediately perform the trade and send the respective amounts of \`buy_token\`
   and \`sell_token\` to the seller and buyer respectively.
4. Seller may call \`withdraw\` to claim any remaining \`sell_token\` balance.
*/
#[contractimpl]
impl SingleOffer {
    // Creates the offer for seller for the given token pair and initial price.
    // See comment above the \`Offer\` struct for information on pricing.
    pub fn create(
        e: Env,
        seller: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: u32,
        buy_price: u32,
    ) {
        if e.storage().instance().has(&DataKey::Offer) {
            panic!("offer is already created");
        }
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }
        // Authorize the \`create\` call by seller to verify their identity.
        seller.require_auth();
        write_offer(
            &e,
            &Offer {
                seller,
                sell_token,
                buy_token,
                sell_price,
                buy_price,
            },
        );
    }

    // Trades \`buy_token_amount\` of buy_token from buyer for \`sell_token\` amount
    // defined by the price.
    // \`min_sell_amount\` defines a lower bound on the price that the buyer would
    // accept.
    // Buyer needs to authorize the \`trade\` call and internal \`transfer\` call to
    // the contract address.
    pub fn trade(e: Env, buyer: Address, buy_token_amount: i128, min_sell_token_amount: i128) {
        // Buyer needs to authorize the trade.
        buyer.require_auth();

        // Load the offer and prepare the token clients to do the trade.
        let offer = load_offer(&e);
        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let buy_token_client = token::Client::new(&e, &offer.buy_token);

        // Compute the amount of token that buyer needs to receive.
        let sell_token_amount = buy_token_amount
            .checked_mul(offer.sell_price as i128)
            .unwrap_optimized()
            / offer.buy_price as i128;

        if sell_token_amount < min_sell_token_amount {
            panic!("price is too low");
        }

        let contract = e.current_contract_address();

        // Perform the trade in 3 \`transfer\` steps.
        // Note, that we don't need to verify any balances - the contract would
        // just trap and roll back in case if any of the transfers fails for
        // any reason, including insufficient balance.

        // Transfer the \`buy_token\` from buyer to this contract.
        // This \`transfer\` call should be authorized by buyer.
        // This could as well be a direct transfer to the seller, but sending to
        // the contract address allows building more transparent signature
        // payload where the buyer doesn't need to worry about sending token to
        // some 'unknown' third party.
        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);
        // Transfer the \`sell_token\` from contract to buyer.
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        // Transfer the \`buy_token\` to the seller immediately.
        buy_token_client.transfer(&contract, &offer.seller, &buy_token_amount);
    }

    // Sends amount of token from this contract to the seller.
    // This is intentionally flexible so that the seller can withdraw any
    // outstanding balance of the contract (in case if they mistakenly
    // transferred wrong token to it).
    // Must be authorized by seller.
    pub fn withdraw(e: Env, token: Address, amount: i128) {
        let offer = load_offer(&e);
        offer.seller.require_auth();
        token::Client::new(&e, &token).transfer(
            &e.current_contract_address(),
            &offer.seller,
            &amount,
        );
    }

    // Updates the price.
    // Must be authorized by seller.
    pub fn updt_price(e: Env, sell_price: u32, buy_price: u32) {
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        offer.sell_price = sell_price;
        offer.buy_price = buy_price;
        write_offer(&e, &offer);
    }

    // Returns the current state of the offer.
    pub fn get_offer(e: Env) -> Offer {
        load_offer(&e)
    }
}

fn load_offer(e: &Env) -> Offer {
    e.storage().instance().get(&DataKey::Offer).unwrap()
}

fn write_offer(e: &Env, offer: &Offer) {
    e.storage().instance().set(&DataKey::Offer, offer);
}

mod test;`, 'simple_account': `
//! This a minimal exapmle of an account contract.
//!
//! The account is owned by a single ed25519 public key that is also used for
//! authentication.
//!
//! For a more advanced example that demonstrates all the capabilities of the
//! Soroban account contracts see \`src / account\` example.
#![no_std]

#[contract]
struct SimpleAccount;

use soroban_sdk::{auth::Context, contract, contractimpl, contracttype, BytesN, Env, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner,
}

#[contractimpl]
impl SimpleAccount {
    // Initialize the contract with an owner's ed25519 public key.
    pub fn init(env: Env, public_key: BytesN<32>) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("owner is already set");
        }
        env.storage().instance().set(&DataKey::Owner, &public_key);
    }

    // This is the 'entry point' of the account contract and every account
    // contract has to implement it. \`require_auth\` calls for the Address of
    // this contract will result in calling this \`__check_auth\` function with
    // the appropriate arguments.
    //
    // This should return \`()\` if authentication and authorization checks have
    // been passed and return an error (or panic) otherwise.
    //
    // \`__check_auth\` takes the payload that needed to be signed, arbitrarily
    // typed signatures (\`BytesN < 64 > \` type here) and authorization
    // context that contains all the invocations that this call tries to verify
    // (not used in this example).
    //
    // In this example \`__check_auth\` only verifies the signature.
    //
    // Note, that \`__check_auth\` function shouldn't call \`require_auth\` on the
    // contract's own address in order to avoid infinite recursion.
    #[allow(non_snake_case)]
    pub fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signature: BytesN<64>,
        _auth_context: Vec<Context>,
    ) {
        let public_key: BytesN<32> = env
            .storage()
            .instance()
            .get::<_, BytesN<32>>(&DataKey::Owner)
            .unwrap();
        env.crypto()
            .ed25519_verify(&public_key, &signature_payload.into(), &signature);
    }
}

mod test;`, 'ttl': `#![no_std]
/// This is a simple contract that just extends TTL for its keys.
/// It's main purpose is to demonstrate how TTL extension can be tested,
/// so the most interesting part here is \`test.rs\`.
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
pub enum DataKey {
    MyKey,
}

#[contract]
pub struct TtlContract;

#[contractimpl]
impl TtlContract {
    /// Creates a contract entry in every kind of storage.
    pub fn setup(env: Env) {
        env.storage().persistent().set(&DataKey::MyKey, &0);
        env.storage().instance().set(&DataKey::MyKey, &1);
        env.storage().temporary().set(&DataKey::MyKey, &2);
    }

    /// Extend the persistent entry TTL to 5000 ledgers, when its
    /// TTL is smaller than 1000 ledgers.
    pub fn extend_persistent(env: Env) {
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::MyKey, 1000, 5000);
    }

    /// Extend the instance entry TTL to become at least 10000 ledgers,
    /// when its TTL is smaller than 2000 ledgers.
    pub fn extend_instance(env: Env) {
        env.storage().instance().extend_ttl(2000, 10000);
    }

    /// Extend the temporary entry TTL to become at least 7000 ledgers,
    /// when its TTL is smaller than 3000 ledgers.
    pub fn extend_temporary(env: Env) {
        env.storage()
            .temporary()
            .extend_ttl(&DataKey::MyKey, 3000, 7000);
    }
}

mod test;`, 'errors': `#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, log, symbol_short, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    LimitReached = 1,
}

const COUNTER: Symbol = symbol_short!("COUNTER");
const MAX: u32 = 5;

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value. Errors
    /// if the value is attempted to be incremented past 5.
    pub fn increment(env: Env) -> Result<u32, Error> {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Check if the count exceeds the max.
        if count <= MAX {
            // Save the count.
            env.storage().instance().set(&COUNTER, &count);

            // Return the count to the caller.
            Ok(count)
        } else {
            // Return an error if the max is exceeded.
            Err(Error::LimitReached)
        }
    }
}

mod test;`, 'events': `
#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        // Publish an event about the increment occuring.
        // The event has two topics:
        //   - The "COUNTER" symbol.
        //   - The "increment" symbol.
        // The event data is the count.
        env.events()
            .publish((COUNTER, symbol_short!("increment")), count);

        // Return the count to the caller.
        count
    }
}

mod test;`, 'atomic_multiswap': `
//! This contract performs a batch of atomic token swaps between multiple
//! parties and does a simple price matching.
//! Parties don't need to know each other and also don't need to know their
//! signatures are used in this contract; they sign the \`AtomicSwap\` contract
//! invocation that guarantees that their token will be swapped with someone
//! while following the price limit.
//! This example demonstrates how authorized calls can be batched together.
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

mod atomic_swap {
    soroban_sdk::contractimport!(
        file = "../atomic_swap/target/wasm32-unknown-unknown/release/soroban_atomic_swap_contract.wasm"
    );
}

#[derive(Clone)]
#[contracttype]
pub struct SwapSpec {
    pub address: Address,
    pub amount: i128,
    pub min_recv: i128,
}

#[contract]
pub struct AtomicMultiSwapContract;

#[contractimpl]
impl AtomicMultiSwapContract {
    // Swap token A for token B atomically between the parties that want to
    // swap A->B and parties that want to swap B->A.
    // All the parties should have authorized the \`swap\` via \`swap_contract\`,
    // but they don't need to authorize \`multi_swap\` itself.
    pub fn multi_swap(
        env: Env,
        swap_contract: Address,
        token_a: Address,
        token_b: Address,
        swaps_a: Vec<SwapSpec>,
        swaps_b: Vec<SwapSpec>,
    ) {
        let mut swaps_b = swaps_b;
        let swap_client = atomic_swap::Client::new(&env, &swap_contract);
        for acc_a in swaps_a.iter() {
            for i in 0..swaps_b.len() {
                let acc_b = swaps_b.get(i).unwrap();

                if acc_a.amount >= acc_b.min_recv && acc_a.min_recv <= acc_b.amount {
                    // As this is a simple 'batching' contract, there is no need
                    // for all swaps to succeed, hence we handle the failures
                    // gracefully to try and clear as many swaps as possible.
                    if swap_client
                        .try_swap(
                            &acc_a.address,
                            &acc_b.address,
                            &token_a,
                            &token_b,
                            &acc_a.amount,
                            &acc_a.min_recv,
                            &acc_b.amount,
                            &acc_b.min_recv,
                        )
                        .is_ok()
                    {
                        swaps_b.remove(i);
                        break;
                    }
                }
            }
        }
    }
}

mod test;`, 'custom_types': `
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}

const STATE: Symbol = symbol_short!("STATE");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env, incr: u32) -> u32 {
        // Get the current count.
        let mut state = Self::get_state(env.clone());

        // Increment the count.
        state.count += incr;
        state.last_incr = incr;

        // Save the count.
        env.storage().instance().set(&STATE, &state);

        // Return the count to the caller.
        state.count
    }
    /// Return the current state.
    pub fn get_state(env: Env) -> State {
        env.storage().instance().get(&STATE).unwrap_or(State {
            count: 0,
            last_incr: 0,
        }) // If no value set, assume 0.
    }
}`

}

export function localContractFetch(contractName) {
    return contractDictionary[contractName];
};