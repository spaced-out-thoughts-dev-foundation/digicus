const contractDictionary = {
    "hello_world": `
#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        if x == 8 || x == 9 && x == 10 {
            for x in 0..10 {
                log!(&env, "Hello, {}!", to);
            }      

            panic!("This is a panic");
        }  
    }
}

mod test;
`,
    "increment": `
#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env, foo: i32, bar: i32, baz: i32, faz: i32) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;
        count -= 2;
        count /= 3;
        count *= 4;

        if x > 0 {
            log!("before nested");
            if x > 10 {
                log!("x is greater than 10");
            } else {
                log!("x is less than or equal to 10");
            }
            log!("after nested");
        } else {
            log!("x is less than or equal to 0"); 
        }

        // Save the count.
        env.storage().instance().set(&COUNTER, &count, foo, bar, baz, foof);

        // The contract instance will be bumped to have a lifetime of at least 100 ledgers if the current expiration lifetime at most 50.
        // If the lifetime is already more than 100 ledgers, this is a no-op. Otherwise,
        // the lifetime is extended to 100 ledgers. This lifetime bump includes the contract
        // instance itself and all entries in storage().instance(), i.e, COUNTER.
        env.storage().instance().extend_ttl(50, 100);

        // Return the count to the caller.
        count
    }

    // pub fn decrement(env: Env) -> u32 {
    //     // Get the current count.
    //     let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
    //     log!(&env, "count: {}", count);

    //     // Decrement the count.
    //     count -= 1;

    //     if x > 0 {
    //         if x > 10 {
    //             log("x is greater than 10");
    //         }
    //     }

    //     // Save the count.
    //     env.storage().instance().set(&COUNTER, &count);

    //     // Return the count to the caller.
    //     count
    // }

    // pub fn hello_world(env: Env) {
    //     log!(&env, "Hello, world!");
    // }
}

mod test;`,
    "custom_types": `
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
}

mod test;
`,
    "logging": `
#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, value: Symbol) {
        log!(&env, "Hello {}", value);
    }
}

mod test;`,
    "errors": `
#![no_std]
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

mod test;
`, "events": `
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

mod test;
`
}

export function localContractFetch(contractName) {
    return contractDictionary[contractName];
};