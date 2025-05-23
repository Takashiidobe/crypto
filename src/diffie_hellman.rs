/// Diffie-Hellman key exchange is used to share secrets.
/// First, we need to generate two prime numbers, `p` and `g`.
/// Second, Alice picks a private key `a` which is greater than `1` but less than `p`.
/// Bob does the same, to pick his private key, `b`.
/// Alice calculates a public key `A`, by calculating it with this formula: `A = g^a mod p`.
/// Likewise, Bob crates his key, `B`: `B = g^b mod p`.
/// Finally, Alice and Bob exchange their public keys, `A` and `B`.
/// Alice calculates `s = B^a mod p` and Bob calculates `A^b mod p`.
/// These result in the same result, the same secret between the pair.
use getrandom::getrandom;
use oorandom::Rand64;

pub fn private_key(p: u64) -> u64 {
    let mut seed: [u8; 16] = [0; 16];
    getrandom(&mut seed).unwrap();

    Rand64::new(u128::from_ne_bytes(seed)).rand_range(2..p)
}

fn modular_exponentiation(base: u128, exp: u64, modular: u64) -> u64 {
    let mut e = exp;
    let mut b = base;

    let mut result = 1;

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modular as u128;
        }

        b = (b * b) % modular as u128;

        e /= 2;
    }

    result as u64
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g as u128, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub as u128, a, p)
}

/// These tests come from exercism.io's Diffie Hellman tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_key_in_range_key() {
        let primes: Vec<u64> = vec![
            5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
        ];
        let private_keys: Vec<u64> = primes.iter().map(|x| private_key(*x)).collect();

        for i in 0..primes.len() {
            assert!(1 < private_keys[i] && private_keys[i] < primes[i]);
        }
    }

    #[test]
    fn public_key_correct() {
        let p: u64 = 23;
        let g: u64 = 5;

        let private_key: u64 = 6;
        let expected: u64 = 8;

        assert_eq!(public_key(p, g, private_key), expected);
    }

    #[test]
    fn secret_key_correct() {
        let p: u64 = 11;

        let private_key_a = 7;
        let public_key_b = 8;
        let secret = secret(p, public_key_b, private_key_a);
        let expected = 2;

        assert_eq!(secret, expected);
    }

    #[test]
    fn public_key_correct_big_numbers() {
        let p: u64 = 4_294_967_299;

        let g: u64 = 8;

        let private_key: u64 = 4_294_967_296;

        let expected: u64 = 4096;

        assert_eq!(public_key(p, g, private_key), expected);
    }

    #[test]
    fn secret_key_correct_big_numbers() {
        let p: u64 = 4_294_967_927;

        let private_key_a = 4_294_967_300;

        let public_key_b = 843;

        let secret = secret(p, public_key_b, private_key_a);

        let expected = 1_389_354_282;

        assert_eq!(secret, expected);
    }

    // two biggest 64bit primes
    const PRIME_64BIT_1: u64 = 0xFFFF_FFFF_FFFF_FFC5;
    const PRIME_64BIT_2: u64 = 0xFFFF_FFFF_FFFF_FFAC;
    const PRIVATE_KEY_64BIT: u64 = 0xFFFF_FFFF_FFFF_FFC3;
    const PUBLIC_KEY_64BIT: u64 = 0xB851_EB85_1EB8_51C1;

    #[test]
    fn public_key_correct_biggest_numbers() {
        assert_eq!(
            public_key(PRIME_64BIT_1, PRIME_64BIT_2, PRIVATE_KEY_64BIT),
            PUBLIC_KEY_64BIT
        );
    }

    #[test]
    fn secret_key_correct_biggest_numbers() {
        let private_key_b = 0xEFFF_FFFF_FFFF_FFC0;
        let public_key_b = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_b);

        let expected_b = 4_340_425_873_327_658_043;
        assert_eq!(public_key_b, expected_b);

        let expected_key = 12_669_955_479_143_291_250;

        let secret_key = secret(PRIME_64BIT_1, public_key_b, PRIVATE_KEY_64BIT);

        assert_eq!(secret_key, expected_key);

        let secret_key = secret(PRIME_64BIT_1, PUBLIC_KEY_64BIT, private_key_b);

        assert_eq!(secret_key, expected_key);
    }

    #[test]
    fn changed_secret_key_biggest_numbers() {
        let private_key_a = private_key(PRIME_64BIT_1);
        let public_key_a = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_a);

        let private_key_b = private_key(PRIME_64BIT_1);
        let public_key_b = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_b);

        let secret_a = secret(PRIME_64BIT_1, public_key_b, private_key_a);
        let secret_b = secret(PRIME_64BIT_1, public_key_a, private_key_b);

        assert_eq!(secret_a, secret_b);
    }

    #[test]
    fn changed_secret_key() {
        let p: u64 = 13;
        let g: u64 = 11;

        let private_key_a = private_key(p);
        let private_key_b = private_key(p);

        let public_key_a = public_key(p, g, private_key_a);
        let public_key_b = public_key(p, g, private_key_b);

        // Key exchange
        let secret_a = secret(p, public_key_b, private_key_a);
        let secret_b = secret(p, public_key_a, private_key_b);

        assert_eq!(secret_a, secret_b);
    }
}
