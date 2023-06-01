#![no_std]
use soroban_sdk::{contractimpl};

const Q64: u128 = 2u128.pow(64);


pub struct UQ64x64Contract;

#[contractimpl]
impl UQ64x64Contract {

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



}

mod test;
