/*
Inspired in:
https://ethereum.stackexchange.com/questions/113130/what-does-decode112with18-do
https://github.com/Uniswap/v2-core/blob/master/contracts/libraries/UQ112x112.sol


*/

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

    /*
    For this test we will use 22/7 (Pi aproximation)
    22/7 ≈ 3.142857142857143
    
    */

    let input: u64 = 10;
    let expected: u128 = 10 * Q64;
    let result = client.encode(&input);
    assert_eq!(result, expected);

    // Here we want to divide 10/3 which should be 3.33333
    let x: u64 = 22;
    let y: u64 = 7;
    let x_encoded: u128 = client.encode(&x);
    assert_eq!(x_encoded, 22 * Q64);

    let y_into: u128 = y.into();
    let expected_div: u128 = x_encoded / y_into;

    let result_div = client.uqdiv(&x_encoded, &y);
    /*  This will produce a fixed point unsigned Q64x64
        number that represents 22/7. This number is 
        represented as a u128 where the upper 64 bits
        represent the integer amount, and the lower 64 bits
        represent the fractional amount. */

    assert_eq!(result_div, expected_div);

    // Fraction is the same as uwdiv(encode(x), y)
    let fraction = client.fraction(&x,&y);
    assert_eq!(result_div, fraction);

    let integer_part: u64 = client.integer_part(&result_div);
    assert_eq!(integer_part, 3);

    
    let decoded_with_7_decimals = client.decode_with_7_decimals(&result_div);
    // 22/7 ≈ 3.1428571
    assert_eq!(decoded_with_7_decimals, 31428571);


}
