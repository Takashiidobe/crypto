/// Taken from: https://github.com/5n00py/soft-aes/blob/main/src/aes/aes_core.rs
use const_for::const_for;
use std::error::Error;

pub const AES_BLOCK_SIZE: usize = 16;

pub const AES_128_KEY_SIZE: usize = 16;
pub const AES_192_KEY_SIZE: usize = 24;
pub const AES_256_KEY_SIZE: usize = 32;

const COL_SIZE: usize = 4;
const ROW_SIZE: usize = 4;
pub type AesBlock = [[u8; COL_SIZE]; ROW_SIZE];

// Multiplication in the Galois Field is defined as a * b ^ p
const fn multiply_gf(a: u8, b: u8) -> u8 {
    let (mut a, mut b) = (a, b);
    let mut p = 0x00;

    const_for!(_ in 0..8 => {
        if 0x01 & b != 0 {
            p ^= a; // p + a
        }
        b >>= 0x01;
        let carry = 0x80 & a; // x^7
        a <<= 1;
        if carry != 0 {
            a ^= 0x1b;
        }
    });
    p
}

const fn left_circular_shift(b: u8, shift: i32) -> u8 {
    (b << shift) | (b >> (8 - shift))
}

const fn find_inverse(arr: u8) -> u8 {
    // Inverse over GF(p^n) is a^p^n-2
    let mut result = arr;

    const_for!(_ in 1..254 => {
        result = multiply_gf(result, arr);
    });
    result
}

const fn affine_transform(c: u8) -> u8 {
    let mut x = find_inverse(c);
    let s = x;

    const_for!(i in 1..5 => {
        x ^= left_circular_shift(s, i);
    });
    x ^= 0x63;

    x
}

const fn sub_byte(a: u8) -> u8 {
    affine_transform(a)
}

const S_BOX: [u8; 256] = {
    let mut result: [u8; 256] = [0; 256];

    const_for!(i in 0..256 => {
        result[i] = sub_byte(i as u8)
    });

    result
};

const fn inverse_affine_transform(c: u8) -> u8 {
    let mut x = c;
    let s = x;

    x = left_circular_shift(s, 1);
    x ^= left_circular_shift(s, 3);
    x ^= left_circular_shift(s, 6);
    x ^= 0x05;

    x = find_inverse(x);

    x
}

const INV_S_BOX: [u8; 256] = {
    let mut result = [0; 256];
    const_for!(i in 0..256 => {
        result[i] = inverse_affine_transform(i as u8);
    });
    result
};

const fn rc(i: u8) -> u8 {
    if i == 0x00 {
        return 0x8D;
    }
    if i == 0x01 {
        return 0x01;
    }

    let rc_p = rc(i.wrapping_sub(1));

    if rc_p < 0x80 {
        rc_p.wrapping_mul(2)
    } else if rc_p >= 0x80 {
        let c = rc_p as u16;
        ((c * 2) ^ 0x11B) as u8
    } else {
        0x00
    }
}

const RCON: [u8; 255] = {
    let mut res = [0; 255];
    const_for!(i in 0..255 => {
        res[i] = rc(i as u8);
    });
    res
};

const LOG_TABLE: [u8; 256] = [
    0x00, 0x00, 0x19, 0x01, 0x32, 0x02, 0x1a, 0xc6, 0x4b, 0xc7, 0x1b, 0x68, 0x33, 0xee, 0xdf, 0x03,
    0x64, 0x04, 0xe0, 0x0e, 0x34, 0x8d, 0x81, 0xef, 0x4c, 0x71, 0x08, 0xc8, 0xf8, 0x69, 0x1c, 0xc1,
    0x7d, 0xc2, 0x1d, 0xb5, 0xf9, 0xb9, 0x27, 0x6a, 0x4d, 0xe4, 0xa6, 0x72, 0x9a, 0xc9, 0x09, 0x78,
    0x65, 0x2f, 0x8a, 0x05, 0x21, 0x0f, 0xe1, 0x24, 0x12, 0xf0, 0x82, 0x45, 0x35, 0x93, 0xda, 0x8e,
    0x96, 0x8f, 0xdb, 0xbd, 0x36, 0xd0, 0xce, 0x94, 0x13, 0x5c, 0xd2, 0xf1, 0x40, 0x46, 0x83, 0x38,
    0x66, 0xdd, 0xfd, 0x30, 0xbf, 0x06, 0x8b, 0x62, 0xb3, 0x25, 0xe2, 0x98, 0x22, 0x88, 0x91, 0x10,
    0x7e, 0x6e, 0x48, 0xc3, 0xa3, 0xb6, 0x1e, 0x42, 0x3a, 0x6b, 0x28, 0x54, 0xfa, 0x85, 0x3d, 0xba,
    0x2b, 0x79, 0x0a, 0x15, 0x9b, 0x9f, 0x5e, 0xca, 0x4e, 0xd4, 0xac, 0xe5, 0xf3, 0x73, 0xa7, 0x57,
    0xaf, 0x58, 0xa8, 0x50, 0xf4, 0xea, 0xd6, 0x74, 0x4f, 0xae, 0xe9, 0xd5, 0xe7, 0xe6, 0xad, 0xe8,
    0x2c, 0xd7, 0x75, 0x7a, 0xeb, 0x16, 0x0b, 0xf5, 0x59, 0xcb, 0x5f, 0xb0, 0x9c, 0xa9, 0x51, 0xa0,
    0x7f, 0x0c, 0xf6, 0x6f, 0x17, 0xc4, 0x49, 0xec, 0xd8, 0x43, 0x1f, 0x2d, 0xa4, 0x76, 0x7b, 0xb7,
    0xcc, 0xbb, 0x3e, 0x5a, 0xfb, 0x60, 0xb1, 0x86, 0x3b, 0x52, 0xa1, 0x6c, 0xaa, 0x55, 0x29, 0x9d,
    0x97, 0xb2, 0x87, 0x90, 0x61, 0xbe, 0xdc, 0xfc, 0xbc, 0x95, 0xcf, 0xcd, 0x37, 0x3f, 0x5b, 0xd1,
    0x53, 0x39, 0x84, 0x3c, 0x41, 0xa2, 0x6d, 0x47, 0x14, 0x2a, 0x9e, 0x5d, 0x56, 0xf2, 0xd3, 0xab,
    0x44, 0x11, 0x92, 0xd9, 0x23, 0x20, 0x2e, 0x89, 0xb4, 0x7c, 0xb8, 0x26, 0x77, 0x99, 0xe3, 0xa5,
    0x67, 0x4a, 0xed, 0xde, 0xc5, 0x31, 0xfe, 0x18, 0x0d, 0x63, 0x8c, 0x80, 0xc0, 0xf7, 0x70, 0x07,
];

const ALOG_TABLE: [u8; 256] = [
    0x01, 0x03, 0x05, 0x0f, 0x11, 0x33, 0x55, 0xff, 0x1a, 0x2e, 0x72, 0x96, 0xa1, 0xf8, 0x13, 0x35,
    0x5f, 0xe1, 0x38, 0x48, 0xd8, 0x73, 0x95, 0xa4, 0xf7, 0x02, 0x06, 0x0a, 0x1e, 0x22, 0x66, 0xaa,
    0xe5, 0x34, 0x5c, 0xe4, 0x37, 0x59, 0xeb, 0x26, 0x6a, 0xbe, 0xd9, 0x70, 0x90, 0xab, 0xe6, 0x31,
    0x53, 0xf5, 0x04, 0x0c, 0x14, 0x3c, 0x44, 0xcc, 0x4f, 0xd1, 0x68, 0xb8, 0xd3, 0x6e, 0xb2, 0xcd,
    0x4c, 0xd4, 0x67, 0xa9, 0xe0, 0x3b, 0x4d, 0xd7, 0x62, 0xa6, 0xf1, 0x08, 0x18, 0x28, 0x78, 0x88,
    0x83, 0x9e, 0xb9, 0xd0, 0x6b, 0xbd, 0xdc, 0x7f, 0x81, 0x98, 0xb3, 0xce, 0x49, 0xdb, 0x76, 0x9a,
    0xb5, 0xc4, 0x57, 0xf9, 0x10, 0x30, 0x50, 0xf0, 0x0b, 0x1d, 0x27, 0x69, 0xbb, 0xd6, 0x61, 0xa3,
    0xfe, 0x19, 0x2b, 0x7d, 0x87, 0x92, 0xad, 0xec, 0x2f, 0x71, 0x93, 0xae, 0xe9, 0x20, 0x60, 0xa0,
    0xfb, 0x16, 0x3a, 0x4e, 0xd2, 0x6d, 0xb7, 0xc2, 0x5d, 0xe7, 0x32, 0x56, 0xfa, 0x15, 0x3f, 0x41,
    0xc3, 0x5e, 0xe2, 0x3d, 0x47, 0xc9, 0x40, 0xc0, 0x5b, 0xed, 0x2c, 0x74, 0x9c, 0xbf, 0xda, 0x75,
    0x9f, 0xba, 0xd5, 0x64, 0xac, 0xef, 0x2a, 0x7e, 0x82, 0x9d, 0xbc, 0xdf, 0x7a, 0x8e, 0x89, 0x80,
    0x9b, 0xb6, 0xc1, 0x58, 0xe8, 0x23, 0x65, 0xaf, 0xea, 0x25, 0x6f, 0xb1, 0xc8, 0x43, 0xc5, 0x54,
    0xfc, 0x1f, 0x21, 0x63, 0xa5, 0xf4, 0x07, 0x09, 0x1b, 0x2d, 0x77, 0x99, 0xb0, 0xcb, 0x46, 0xca,
    0x45, 0xcf, 0x4a, 0xde, 0x79, 0x8b, 0x86, 0x91, 0xa8, 0xe3, 0x3e, 0x42, 0xc6, 0x51, 0xf3, 0x0e,
    0x12, 0x36, 0x5a, 0xee, 0x29, 0x7b, 0x8d, 0x8c, 0x8f, 0x8a, 0x85, 0x94, 0xa7, 0xf2, 0x0d, 0x17,
    0x39, 0x4b, 0xdd, 0x7c, 0x84, 0x97, 0xa2, 0xfd, 0x1c, 0x24, 0x6c, 0xb4, 0xc7, 0x52, 0xf6, 0x01,
];

fn mul(a: u8, b: u8) -> u8 {
    if a != 0 && b != 0 {
        let log_a = LOG_TABLE[a as usize] as usize;
        let log_b = LOG_TABLE[b as usize] as usize;
        let log_sum = (log_a + log_b) % 255; // Modulo 255 to keep within bounds
        ALOG_TABLE[log_sum]
    } else {
        0
    }
}

fn expand_key(key: &[u8], nk: usize, nr: usize) -> [u8; 240] {
    let mut expanded_key = [0u8; 240]; // Fixed buffer for expanded key
    let mut temp = [0u8; 4]; // Temporary storage for key schedule

    // Copy the initial key as the first round key
    for i in 0..nk {
        expanded_key[i * 4..(i + 1) * 4].copy_from_slice(&key[i * 4..(i + 1) * 4]);
    }

    let mut i = nk; // Initialize `i` to number of words in the original key

    while i < COL_SIZE * (nr + 1) {
        // Load the last word from the previous round key into `temp`
        for j in 0..4 {
            temp[j] = expanded_key[(i - 1) * 4 + j];
        }

        if i % nk == 0 {
            // Perform the RotWord operation for the first word in each new key
            let k = temp[0];
            temp.rotate_left(1); // Rotate the 4 bytes of the word to the left
            temp[3] = k;

            // SubWord operation: Substitute each byte in `temp` using the S-Box
            for j in 0..4 {
                temp[j] = S_BOX[temp[j] as usize];
            }

            // XOR the first byte of `temp` with the round constant (RCON)
            temp[0] ^= RCON[i / nk];
        } else if nk > 6 && i % nk == 4 {
            // For AES-256, apply SubWord operation every fourth word
            for j in 0..4 {
                temp[j] = S_BOX[temp[j] as usize];
            }
        }

        // Generate the next word of the round key
        for j in 0..4 {
            expanded_key[i * 4 + j] = expanded_key[(i - nk) * 4 + j] ^ temp[j];
        }
        i += 1;
    }
    expanded_key
}

fn add_round_key(round: usize, state: &mut AesBlock, expanded_key: &[u8; 240]) {
    for i in 0..4 {
        for j in 0..4 {
            state[j][i] ^= expanded_key[round * COL_SIZE * 4 + i * COL_SIZE + j];
        }
    }
}

fn sub_bytes(state: &mut AesBlock) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = S_BOX[state[i][j] as usize];
        }
    }
}

fn inv_sub_bytes(state: &mut AesBlock) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = INV_S_BOX[state[i][j] as usize];
        }
    }
}

// Note: Doing this manually seems to generate better code on my computer.
// I prefer the rotates, but it generates a loop.
pub fn shift_rows(state: &mut [[u8; 4]; 4]) {
    state[1].rotate_left(1);
    state[2].rotate_left(2);
    state[3].rotate_left(3);
}

fn inv_shift_rows(state: &mut AesBlock) {
    state[1].rotate_right(1);
    state[2].rotate_right(2);
    state[3].rotate_right(3);
}

fn mix_columns(state: &mut AesBlock) {
    for i in 0..4 {
        // Iterate over each column
        let t = state[0][i];
        let tmp = state[0][i] ^ state[1][i] ^ state[2][i] ^ state[3][i];

        let mut tm = state[0][i] ^ state[1][i];
        tm = mul(tm, 2);
        state[0][i] ^= tm ^ tmp;

        tm = state[1][i] ^ state[2][i];
        tm = mul(tm, 2);
        state[1][i] ^= tm ^ tmp;

        tm = state[2][i] ^ state[3][i];
        tm = mul(tm, 2);
        state[2][i] ^= tm ^ tmp;

        tm = state[3][i] ^ t;
        tm = mul(tm, 2);
        state[3][i] ^= tm ^ tmp;
    }
}

fn inv_mix_columns(state: &mut AesBlock) {
    for i in 0..4 {
        // Save original state for column i
        let (a, b, c, d) = (state[0][i], state[1][i], state[2][i], state[3][i]);

        // Perform the inverse mix column operation on each element of the column
        state[0][i] = mul(a, 0x0e) ^ mul(b, 0x0b) ^ mul(c, 0x0d) ^ mul(d, 0x09);
        state[1][i] = mul(a, 0x09) ^ mul(b, 0x0e) ^ mul(c, 0x0b) ^ mul(d, 0x0d);
        state[2][i] = mul(a, 0x0d) ^ mul(b, 0x09) ^ mul(c, 0x0e) ^ mul(d, 0x0b);
        state[3][i] = mul(a, 0x0b) ^ mul(b, 0x0d) ^ mul(c, 0x09) ^ mul(d, 0x0e);
    }
}

fn copy_block_to_state(block: &[u8; AES_BLOCK_SIZE]) -> AesBlock {
    let mut state = [[0u8; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            state[j][i] = block[i * 4 + j];
        }
    }

    state
}
fn copy_state_to_block(state: &AesBlock) -> [u8; AES_BLOCK_SIZE] {
    let mut block = [0u8; AES_BLOCK_SIZE];

    for i in 0..4 {
        for j in 0..4 {
            block[i * 4 + j] = state[j][i];
        }
    }

    block
}
fn calculate_parameters(key_length_bytes: usize) -> (usize, usize) {
    let words_in_key = key_length_bytes / 4; // 1 word = 4 bytes
    let encryption_rounds = match words_in_key {
        4 => 10, // 128-bit key
        6 => 12, // 192-bit key
        8 => 14, // 256-bit key
        _ => panic!(
            "AES CORE PANIC: Invalid AES key length: {}",
            key_length_bytes
        ),
    };

    (words_in_key, encryption_rounds)
}

fn validate_key_len(key_len: usize) -> Result<(), Box<dyn Error>> {
    match key_len {
        AES_128_KEY_SIZE | AES_192_KEY_SIZE | AES_256_KEY_SIZE => Ok(()),
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "AES CORE ERROR: Invalid key length. Expected 16, 24, or 32 bytes, got {} bytes",
                key_len,
            ),
        ))),
    }
}

pub fn aes_enc_block(
    block: &[u8; AES_BLOCK_SIZE],
    key: &[u8],
) -> Result<[u8; AES_BLOCK_SIZE], Box<dyn Error>> {
    let key_len = key.len();

    validate_key_len(key_len)?;

    let (nk, nr) = calculate_parameters(key_len);

    let mut state = copy_block_to_state(block);

    let expanded_key = expand_key(key, nk, nr);

    // Add the first round key to the state before starting the rounds
    add_round_key(0, &mut state, &expanded_key);

    // Main rounds
    for round in 1..nr {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(round, &mut state, &expanded_key);
    }

    // Final round (without mix_columns)
    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(nr, &mut state, &expanded_key);

    Ok(copy_state_to_block(&state))
}

pub fn aes_dec_block(
    ciphertext: &[u8; AES_BLOCK_SIZE],
    key: &[u8],
) -> Result<[u8; AES_BLOCK_SIZE], Box<dyn Error>> {
    let key_len = key.len();

    validate_key_len(key_len)?;

    let (nk, nr) = calculate_parameters(key_len);

    let mut state = copy_block_to_state(ciphertext);

    let expanded_key = expand_key(key, nk, nr);

    // Add the last round key to the state before starting the rounds
    add_round_key(nr, &mut state, &expanded_key);

    // Main rounds
    for round in (1..nr).rev() {
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(round, &mut state, &expanded_key);
        inv_mix_columns(&mut state);
    }

    // Final round (without inv_mix_columns)
    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    add_round_key(0, &mut state, &expanded_key);

    Ok(copy_state_to_block(&state))
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn ex() {
        // Test vectors for AES-128
        let plaintext: [u8; AES_BLOCK_SIZE] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let key: [u8; AES_128_KEY_SIZE] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        let expected_ciphertext: [u8; AES_BLOCK_SIZE] = [
            0xfd, 0xe4, 0xfb, 0xae, 0x4a, 0x09, 0xe0, 0x20, 0xef, 0xf7, 0x22, 0x96, 0x9f, 0x83,
            0x83, 0x2b,
        ];

        // Perform AES-128 encryption
        let ciphertext = aes_enc_block(&plaintext, &key).expect("Encryption failed");
        assert_eq!(ciphertext, expected_ciphertext);

        // Perform AES-128 decryption
        let decrypted = aes_dec_block(&ciphertext, &key).expect("Decryption failed");
        assert_eq!(decrypted, plaintext);
    }

    #[quickcheck]
    fn enc_and_dec(plaintext: Vec<u8>, key: Vec<u8>) -> bool {
        // we need enough bytes to generate the plaintext and key
        if plaintext.len() < 16 || key.len() < 16 {
            return true;
        }
        let plaintext: &[u8; 16] = &plaintext[..16].try_into().unwrap();
        let key: &[u8; 16] = &key[..16].try_into().unwrap();
        let ciphertext = aes_enc_block(plaintext, key).expect("Encryption failed");
        let decrypted = aes_dec_block(&ciphertext, key).expect("Decryption failed");
        decrypted == *plaintext
    }
}
