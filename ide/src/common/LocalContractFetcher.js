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
}`, 'atomic_swap': `
//! This contract performs an atomic token swap between two parties.
//! Parties don't need to know each other and their signatures may be matched
//! off-chain.
//! This example demonstrates how multi-party authorization can be implemented.
#![no_std]

use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal};

#[contract]
pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {
    // Swap token A for token B atomically. Settle for the minimum requested price
    // for each party (this is an arbitrary choice; both parties could have
    // received the full amount as well).
    pub fn swap(
        env: Env,
        a: Address,
        b: Address,
        token_a: Address,
        token_b: Address,
        amount_a: i128,
        min_b_for_a: i128,
        amount_b: i128,
        min_a_for_b: i128,
    ) {
        // Verify preconditions on the minimum price for both parties.
        if amount_b < min_b_for_a {
            panic!("not enough token B for token A");
        }
        if amount_a < min_a_for_b {
            panic!("not enough token A for token B");
        }
        // Require authorization for a subset of arguments specific to a party.
        // Notice, that arguments are symmetric - there is no difference between
        // \`a\` and \`b\` in the call and hence their signatures can be used
        // either for \`a\` or for \`b\` role.
        a.require_auth_for_args(
            (token_a.clone(), token_b.clone(), amount_a, min_b_for_a).into_val(&env),
        );
        b.require_auth_for_args(
            (token_b.clone(), token_a.clone(), amount_b, min_a_for_b).into_val(&env),
        );

        // Perform the swap by moving tokens from a to b and from b to a.
        move_token(&env, &token_a, &a, &b, amount_a, min_a_for_b);
        move_token(&env, &token_b, &b, &a, amount_b, min_b_for_a);
    }
}

fn move_token(
    env: &Env,
    token: &Address,
    from: &Address,
    to: &Address,
    max_spend_amount: i128,
    transfer_amount: i128,
) {
    let token = token::Client::new(env, token);
    let contract_address = env.current_contract_address();
    // This call needs to be authorized by \`from\` address. It transfers the
    // maximum spend amount to the swap contract's address in order to decouple
    // the signature from \`to\` address (so that parties don't need to know each
    // other).
    token.transfer(from, &contract_address, &max_spend_amount);
    // Transfer the necessary amount to \`to\`.
    token.transfer(&contract_address, to, &transfer_amount);
    // Refund the remaining balance to \`from\`.
    token.transfer(
        &contract_address,
        from,
        &(max_spend_amount - transfer_amount),
    );
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

mod test;`, 'auth': `
//! This contract demonstrates how to implement authorization using
//! Soroban-managed auth framework for a simple case (a single user that needs
//! to authorize a single contract invocation).
//!
//! See \`timelock\` and \`single_offer\` examples for demonstration of performing
//! authorized token operations on behalf of the user.
//!
//! See \`atomic_swap\` and \`atomic_multiswap\` examples for demonstration of
//! multi-party authorizaton.
//!
//! See \`account\` example for demonstration of an acount contract with
//! a custom authentication scheme and a custom authorization policy.
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Counter(Address),
}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments a counter for the user, and returns the value.
    pub fn increment(env: Env, user: Address, value: u32) -> u32 {
        // Requires \`user\` to have authorized call of the \`increment\` of this
        // contract with all the arguments passed to \`increment\`, i.e. \`user\`
        // and \`value\`. This will panic if auth fails for any reason.
        // When this is called, Soroban host performs the necessary
        // authentication, manages replay prevention and enforces the user's
        // authorization policies.
        // The contracts normally shouldn't worry about these details and just
        // write code in generic fashion using \`Address\` and \`require_auth\` (or
        // \`require_auth_for_args\`).
        user.require_auth();

        // This call is equilvalent to the above:
        // user.require_auth_for_args((&user, value).into_val(&env));

        // The following has less arguments but is equivalent in authorization
        // scope to the above calls (the user address doesn't have to be
        // included in args as it's guaranteed to be authenticated).
        // user.require_auth_for_args((value,).into_val(&env));

        // Construct a key for the data being stored. Use an enum to set the
        // contract up well for adding other types of data to be stored.
        let key = DataKey::Counter(user.clone());

        // Get the current count for the invoker.
        let mut count: u32 = env.storage().persistent().get(&key).unwrap_or_default();

        // Increment the count.
        count += value;

        // Save the count.
        env.storage().persistent().set(&key, &count);

        // Return the count to the caller.
        count
    }
}

mod test;`, 'timelock': `
//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
//! For simplicity, the contract only supports invoker-based auth.
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Balance,
}

#[derive(Clone)]
#[contracttype]
pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contract]
pub struct ClaimableBalanceContract;

// The 'timelock' part: check that provided timestamp is before/after
// the current ledger timestamp.
fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();

    match time_bound.kind {
        TimeBoundKind::Before => ledger_timestamp <= time_bound.timestamp,
        TimeBoundKind::After => ledger_timestamp >= time_bound.timestamp,
    }
}

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        if is_initialized(&env) {
            panic!("contract has been already initialized");
        }
        // Make sure \`from\` address authorized the deposit call with all the
        // arguments.
        from.require_auth();

        // Transfer token from \`from\` to this contract address.
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);
        // Store all the necessary info to allow one of the claimants to claim it.
        env.storage().instance().set(
            &DataKey::Balance,
            &ClaimableBalance {
                token,
                amount,
                time_bound,
                claimants,
            },
        );
        // Mark contract as initialized to prevent double-usage.
        // Note, that this is just one way to approach initialization - it may
        // be viable to allow one contract to manage several claimable balances.
        env.storage().instance().set(&DataKey::Init, &());
    }

    pub fn claim(env: Env, claimant: Address) {
        // Make sure claimant has authorized this call, which ensures their
        // identity.
        claimant.require_auth();
        // Just get the balance - if it's been claimed, this will simply panic
        // and terminate the contract execution.
        let claimable_balance: ClaimableBalance =
            env.storage().instance().get(&DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant) {
            panic!("claimant is not allowed to claim this balance");
        }

        // Transfer the stored amount of token to claimant after passing
        // all the checks.
        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &claimable_balance.amount,
        );
        // Remove the balance entry to prevent any further claims.
        env.storage().instance().remove(&DataKey::Balance);
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Init)
}

mod test;`, 'simple_account': `
//! This a minimal exapmle of an account contract.
//!
//! The account is owned by a single ed25519 public key that is also used for
//! authentication.
//!
//! For a more advanced example that demonstrates all the capabilities of the
//! Soroban account contracts see \`src/ account\` example.
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

mod test;`, 'alloc': `
#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

extern crate alloc;

#[contract]
pub struct AllocContract;

#[contractimpl]
impl AllocContract {
    /// Allocates a temporary vector holding values (0..count), then computes and returns their sum.
    pub fn sum(_env: Env, count: u32) -> u32 {
        let mut v1 = alloc::vec![];
        (0..count).for_each(|i| v1.push(i));

        let mut sum = 0;
        for i in v1 {
            sum += i;
        }

        sum
    }
}

mod test;`, 'eth_abi': `
#![no_std]
extern crate alloc;
use alloy_sol_types::{sol, SolValue};
use soroban_sdk::{contract, contracterror, contractimpl, Bytes, Env};

#[cfg(test)]
mod test;

#[contracterror]
#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Error {
    Decode = 1,
}

#[contract]
pub struct Contract;

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

#[contractimpl]
impl Contract {
    pub fn exec(e: &Env, input: Bytes) -> Result<Bytes, Error> {
        let mut input_buf = [0u8; 128];
        let mut input_slice = &mut input_buf[..input.len() as usize];
        input.copy_into_slice(&mut input_slice);

        let input = Input::abi_decode(&input_slice, false).map_err(|_| Error::Decode)?;
        let output = Output {
            a: input.a,
            r: input.b + input.c,
        };
        let output_encoded = output.abi_encode();
        Ok(Bytes::from_slice(e, &output_encoded))
    }
}`, 'fuzzing': `
//! This is the 'timelock' example modified slightly to demonstrate
//! Soroban's fuzzing capabilities.
//!
//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
//! For simplicity, the contract only supports invoker-based auth.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Balance,
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contract]
pub struct ClaimableBalanceContract;

// The 'timelock' part: check that provided timestamp is before/after
// the current ledger timestamp.
fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();

    match time_bound.kind {
        TimeBoundKind::Before => ledger_timestamp <= time_bound.timestamp,
        TimeBoundKind::After => ledger_timestamp >= time_bound.timestamp,
    }
}

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
        // Perhaps this check should be enabled...
        /*if amount == 0 {
            panic!("deposit amount must not be zero");
        }*/

        if claimants.is_empty() {
            panic!("need more than 0 claimants");
        }
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        if is_initialized(&env) {
            panic!("contract has been already initialized");
        }
        // Make sure \`from\` address authorized the deposit call with all the
        // arguments.
        from.require_auth();

        // Transfer token from \`from\` to this contract address.
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);
        // Store all the necessary info to allow one of the claimants to claim it.
        env.storage().persistent().set(
            &DataKey::Balance,
            &ClaimableBalance {
                token,
                amount,
                time_bound,
                claimants,
            },
        );
        // Mark contract as initialized to prevent double-usage.
        // Note, that this is just one way to approach initialization - it may
        // be viable to allow one contract to manage several claimable balances.
        env.storage().persistent().set(&DataKey::Init, &());
    }

    pub fn claim(env: Env, claimant: Address, amount: i128) {
        // Make sure claimant has authorized this call, which ensures their
        // identity.
        claimant.require_auth();

        let mut claimable_balance: ClaimableBalance =
            env.storage().persistent().get(&DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant) {
            panic!("claimant is not allowed to claim this balance");
        }

        if amount > claimable_balance.amount {
            panic!("claimed amount greater than balance");
        }

        // Transfer the stored amount of token to claimant after passing
        // all the checks.
        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &amount,
        );

        let new_balance = claimable_balance.amount - amount;

        if new_balance > 0 {
            // Store the new balance.
            claimable_balance.amount = new_balance;
            env.storage()
                .persistent()
                .set(&DataKey::Balance, &claimable_balance);
        } else {
            // Remove the balance entry to prevent any further claims.
            env.storage().persistent().remove(&DataKey::Balance);
        }
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().persistent().has(&DataKey::Init)
}

mod proptest;`, 'mint_lock': `
#![no_std]
use soroban_sdk::{
    contract, contractclient, contracterror, contractimpl, contracttype, Address, Env, IntoVal,
};

#[contractclient(name = "MintClient")]
trait MintInterface {
    fn mint(env: Env, to: Address, amount: i128);
}

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    NotAuthorizedMinter = 1,
    DailyLimitInsufficient = 2,
    NegativeAmount = 3,
}

#[contracttype]
pub enum StorageKey {
    /// Admin. Value is an Address.
    Admin,
    /// Minters are stored keyed by the contract and minter addresses. Value is
    /// a MinterConfig.
    Minter(Address, Address),
    /// Minter stats are stored keyed by contract and minter addresses, epoch
    /// length, and epoch, which is the ledger number divided by the number of
    /// ledgers in the epoch.  Value is a MinterStats.
    MinterStats(Address, Address, u32, u32),
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinterConfig {
    limit: i128,
    epoch_length: u32,
}

#[contracttype]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MinterStats {
    consumed_limit: i128,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Set the admin.
    pub fn set_admin(env: Env, new_admin: Address) {
        if let Some(admin) = env
            .storage()
            .instance()
            .get::<_, Address>(&StorageKey::Admin)
        {
            admin.require_auth();
        };
        env.storage().instance().set(&StorageKey::Admin, &new_admin);
    }

    /// Return the admin address.
    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get::<_, Address>(&StorageKey::Admin)
            .unwrap()
    }

    /// Set the config of a minter for the given contract. Requires auth from
    /// the admin.
    pub fn set_minter(env: Env, contract: Address, minter: Address, config: MinterConfig) {
        Self::admin(env.clone()).require_auth();
        env.storage()
            .persistent()
            .set(&StorageKey::Minter(contract, minter), &config);
    }

    /// Returns the config, current epoch, and current epoch's stats for a
    /// minter.
    pub fn minter(
        env: Env,
        contract: Address,
        minter: Address,
    ) -> Result<(MinterConfig, u32, MinterStats), Error> {
        let config = env
            .storage()
            .persistent()
            .get::<_, MinterConfig>(&StorageKey::Minter(contract.clone(), minter.clone()))
            .ok_or(Error::NotAuthorizedMinter)?;
        let epoch = env.ledger().sequence() / config.epoch_length;
        let stats = env
            .storage()
            .temporary()
            .get::<_, MinterStats>(&StorageKey::MinterStats(
                contract.clone(),
                minter.clone(),
                config.epoch_length,
                epoch,
            ))
            .unwrap_or_default();
        Ok((config, epoch, stats))
    }

    /// Calls the 'mint' function of the 'contract' with 'to' and 'amount'.
    /// Authorized by the 'minter'. Uses some of the authorized 'minter's
    /// current epoch's limit.
    pub fn mint(
        env: Env,
        contract: Address,
        minter: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), Error> {
        // Verify minter is authenticated, and authorizing args.
        minter.require_auth_for_args((&contract, &to, amount).into_val(&env));

        // Verify amount is positive.
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }

        // Verify minter is authorized by contract.
        let admin = Self::admin(env.clone());
        if admin != minter {
            let Some(config) = env
                .storage()
                .persistent()
                .get::<_, MinterConfig>(&StorageKey::Minter(contract.clone(), minter.clone()))
            else {
                return Err(Error::NotAuthorizedMinter);
            };

            // Check and track daily limit.
            let epoch = env.ledger().sequence() / config.epoch_length;
            let minter_stats_key = StorageKey::MinterStats(
                contract.clone(),
                minter.clone(),
                config.epoch_length,
                epoch,
            );
            let minter_stats = env
                .storage()
                .temporary()
                .get::<_, MinterStats>(&minter_stats_key)
                .unwrap_or_default();
            let new_minter_stats = MinterStats {
                consumed_limit: minter_stats.consumed_limit + amount,
            };
            if new_minter_stats.consumed_limit > config.limit {
                return Err(Error::DailyLimitInsufficient);
            }
            env.storage()
                .temporary()
                .set::<_, MinterStats>(&minter_stats_key, &new_minter_stats);
            env.storage()
                .temporary()
                .extend_ttl(&minter_stats_key, 0, epoch * config.epoch_length);
        }

        // Perform the mint.
        let client = MintClient::new(&env, &contract);
        client.mint(&to, &amount);
        Ok(())
    }
}

mod test;`
}

export function localContractFetch(contractName) {
    return contractDictionary[contractName];
};