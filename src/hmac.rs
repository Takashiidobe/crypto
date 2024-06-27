use std::cmp::Ordering;

use crate::sha1::Sha1;

pub struct HMAC;

impl HMAC {
    /// This implements HMAC for SHA1.
    /// The function takes in the bytes to hash and a secret key used to hash.
    /// The high level overview looks like this:
    /// input: &[u8]: the message to hash
    /// secret_key: &[u8]: the secret key to use in hashing.
    /// 1. The secret key is sized appropriately. (64 bytes in SHA-1)
    /// - If the key is too long or too short, it is set to 64 bytes.
    /// - If too short, it is padded with zeroes on the right
    /// - if too long, it is hashed and then padded with zeroes on the right
    /// 2. Two keys are generated
    /// - An outer key, which takes the sized key and xors it with 0x5c
    /// - And inner key, which is xored with 0x36.
    /// 3. The inner key is concatenated with the input and then hashed.
    /// 4. And then the hash is calculated of the outer key concatenated by that result.
    pub fn mac(input: &[u8], secret_key: &[u8]) -> [u8; 20] {
        // 1. If the secret key is too long, it is shortened by hashing it.
        // Otherwise, the key can be used as is.
        let block_sized_key = Self::block_size_key(secret_key);
        // 2. Next, generate two keys.
        // The first key, the outer key, is xored with 0x36.
        let mut padded = [0x36; 40];
        for (p, &k) in padded.iter_mut().zip(block_sized_key.iter()) {
            *p ^= k;
        }

        let mut ih_input = padded.to_vec();
        ih_input.extend(input);
        let ih = Sha1::hash(&ih_input);

        for p in padded.iter_mut() {
            *p ^= 0x6a;
        }
        // 3. The key is hashed with the inner key first then the outer key hashes that.
        let mut oh_input = padded.to_vec();
        oh_input.extend(&ih);
        Sha1::hash(&oh_input)
    }

    fn block_size_key(secret_key: &[u8]) -> [u8; 64] {
        match secret_key.len().cmp(&64) {
            Ordering::Less => {
                let mut res = [0; 64];
                for (i, b) in secret_key.iter().enumerate() {
                    res[i] = *b;
                }
                res
            }
            Ordering::Equal => {
                let mut res = [0; 64];
                res.copy_from_slice(secret_key);
                res
            }
            Ordering::Greater => {
                let mut res = [0; 64];
                for (i, b) in Sha1::hash(secret_key).iter().enumerate() {
                    res[i] = *b;
                }
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let h = HMAC::mac(b"", &[]);
        assert_eq!(
            h,
            [
                0x2c, 0x4c, 0x5d, 0xb0, // first
                0x09, 0x76, 0xff, 0xdb, // second
                0x10, 0xdb, 0xd5, 0x32, // third
                0xe2, 0x78, 0x35, 0xa9, // fourth
                0x84, 0x8e, 0x6c, 0xef, // fifth
            ]
        );
    }
}
