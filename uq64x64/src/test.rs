#![cfg(test)]

use super::{UQ64x64Contract, UQ64x64ContractClient};
use soroban_sdk::{ Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UQ64x64Contract);
    let client = UQ64x64ContractClient::new(&env, &contract_id);

    const Q64: u128 = 2u128.pow(64);

    let input: u64 = 10;
    let expected: u128 = 10 * Q64;
    let result = client.encode(&input);
    assert_eq!(result, expected);

    // Here we want to divide 10/3 which should be 3.33333
    /*
    
    */
    let x: u128 = 10 * Q64;
    let y: u64 = 3;
    let expected_div: u128 = 10 * Q64 / 3;
    let result_div = client.uqdiv(&x, &y);
    /*  This will produce a fixed point unsigned Q64x64
        number that represents 1/3. This number is 
        represented as a u128 where the upper 64 bits
        represent the integer amount, and the lower 64 bits
        represent the fractional amount. */

    assert_eq!(result_div, expected_div);

    let integer_part: u64 = (result_div >> 64) as u64;
    //let fractional_mask: u128 = (1u128 << 64) - 1;
    //let fractional_part: u64 = (result_div & fractional_mask) as u64;
    let fractional_part: u64 = (result_div << 65 ) as u64;

    assert_eq!(integer_part, 3);
    assert_eq!(fractional_part, 3333);



    // The result is not "just" 10/3 converted
    //let new_input: u64 = 10/3;
   // assert_eq!(new_input, 1);


}
