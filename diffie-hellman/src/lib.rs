use std::convert::TryInto;
use std::mem::size_of;

use num::BigUint;
use rand::prelude::*;

/// Generates a random private key in the range [2; p].
pub fn private_key(p: u64) -> u64 {
    (2..p).choose(&mut thread_rng()).unwrap()
}

/// Computes the public key from the given prime numbers `p` and `g` and from
/// `a`, one of the public keys previously generated.
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

/// Computes the shared secret from the prime `p` used as modulus and the two
/// public keys `a` and `b` exchanged during the Diffie-Hellman procedure.
pub fn secret(p: u64, b: u64, a: u64) -> u64 {
    mod_pow(b, a, p)
}

/// Calculates the modular exponentiation `x^exp mod modl` using big integers,
/// which ensures the ability to handle exponents larger than `u32`.
fn mod_pow(x: u64, exp: u64, modl: u64) -> u64 {
    // Conversion to unsigned big integers.
    let x = BigUint::from_bytes_be(&x.to_be_bytes());
    let exp = BigUint::from_bytes_be(&exp.to_be_bytes());
    let modl = BigUint::from_bytes_be(&modl.to_be_bytes());

    // Compute the modular power and pad it to the correct size.
    let mut big_res = x.modpow(&exp, &modl).to_bytes_le();
    big_res.append(&mut vec![
        0u8;
        size_of::<u64>().saturating_sub(big_res.len())
    ]);
    big_res.reverse();

    // Truncate the big result back to a u64.
    u64::from_be_bytes(big_res[..size_of::<u64>()].try_into().unwrap())
}
