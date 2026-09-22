use num_bigint::BigUint;

/// alpha^x mod p 
/// output = n ^ exp mod p 
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    n.modpow(exponent, modulus)

}

/// Output = s = k - c * x mod p
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
    if *k >= c * x {
        k - c * x
    }
}