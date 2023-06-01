#![no_std]
use soroban_sdk::{contractimpl, log, Env, Symbol};

const COUNTER: Symbol = Symbol::short("COUNTER");

const Q64: u128 = 2u128.pow(112);

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {

    // encode a u64 as a UQ64x64
    pub fn encode(y: u64) -> u128 {
        let y_into128: u128 = y.into();
        let z: u128 = y_into128 * Q64;
        z
    }

    // divide a UQ64x64 by a u64, returning a UQ64x64
    pub fn uqdiv(x: u128, y: u64) -> u128 {
        let y_into128: u128 = y.into();
        let z: u128 = x / y_into128;
        z
    }



    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not u32.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }
}

mod test;
