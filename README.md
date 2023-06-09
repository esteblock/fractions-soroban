# fractions-soroban
Exploring non-integer types in Soroban
Inspired by: 
[UniswapV2 QU112X112](https://github.com/Uniswap/v2-core/blob/master/contracts/libraries/UQ112x112.sol), [Compound Finance](https://github.com/compound-finance/open-oracle/blob/master/contracts/Uniswap/UniswapLib.sol) and [hroussille](https://ethereum.stackexchange.com/questions/113130/what-does-decode112with18-do)

# Instructions:
```
bash run.sh
cd uq64x64
make build
make test
```


# UQ64X64
In the context of handling binary fixed-point numbers, an experiment was conducted utilizing the QU64X64 format (as described on https://en.wikipedia.org/wiki/Q_(number_format)).

To elaborate, the QU64X64 format signifies that fixed-point numbers represented in this format have 64 bits allocated for the integer part and 64 bits allocated for the fractional part. The letter "U" can be added as a prefix to "Q" to indicate an unsigned binary fixed-point format.

In this case, the QU64X64 object is stored using a 128-bit primitive, specifically a u128.

## Functions:

1.- encode(u64)
Encode a u64 as a UQ64X64
```rust
pub fn encode(y: u64) -> u128 {
        let y_into128: u128 = y.into();
        let z: u128 = y_into128 * Q64;
        z
    }
```

2.- fraction
Returns a UQ64x64 which represents the ratio of the x to y
```rust
pub fn fraction(x: u64, y: u64) -> u128 {
        if y == 0 {
            panic!("DIV_BY_ZERO")
        }
        Self::uqdiv(Self::encode(x),y)
    }
```



3.- uqdiv
Divide a UQ64x64 by a u64, returning a UQ64x64
Because dividing two UQ64x64, the number will no longer be multiplied by 2^64, won't be a UQ64x64 anymore... to have a UQ64x64 as a result, a UQ64x64 must be divided by a u64

```rust
pub fn uqdiv(x: u128, y: u64) -> u128 {
    if y == 0 {
        panic!("DIV_BY_ZERO")
    }
    let y_into128: u128 = y.into();
    let z: u128 = x / y_into128;
    z
}
```

4.- integer_part
```rust
pub fn integer_part(x: u128) -> u64 {
    (x >> 64) as u64
} 
```

5.- fractional_part
To do...

6.- decode_with_7_decimals
Decodes a UQ112x112 into a u128 with 7 decimals of precision

to get close to: (x * 1e7) / 2^64 without risk of overflowing we do:
        = (x) * (2**log2(1e7)) / 2^64
        = (x) / (2 ** (64 - log2(1e7)))
        ≈ (x) / (1.8446744073709551616 × 10^12 )
        ≈ (x) / 1844674407370


```rust
pub fn decode_with_7_decimals(x: u128) -> u128 {
    x / 1844674407370
}
```


