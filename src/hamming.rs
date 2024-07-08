#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("hamming-code-venn-diagram", "./images/7-4-hamming-code.svg"),
))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! # Hamming codes
//!
//! Hamming codes are used for error detection and correction. A Hamming code can detect one
//! bit error and correct one bit errors.
//! This module impleemnts a (7, 4) Hamming code, which uses 3 parity bits for every 4 bits of
//! data.
//!
//! ## Why Error detection and correction is important
//!
//! Imagine you want to send a block of data (let's say 4 bits) over a noisy channel, like the
//! internet. Sometimes, the bits will be flipped due to noise in the channel.
//! To deal with this, we could send one copy of the data.
//! Imagine that we want to send the array $[1,1,1,1]$, and we send $[1,0,1,1]$ and $[1,1,1,1]$. We know
//! that the 0th, 2nd, and 3rd bit agree, so we know they're one. But we don't know about the first
//! bit. So our array looks like $[1,?,1,1]$. But we can't tell if the original or the copy's bit was
//! flipped.
//! We can send 3 copies of the data, so we can pick best two out of three.
//! This breaks ties, but is inefficient -- given $n$ bits of data, we have to send $3n$ bits of
//! data, to allow any 1 bit of error correction per array slot. If two bits that correspond to the
//! same slot are flipped, then we'll get the wrong answer, so this scheme is both inefficient
//! memory wise and prone to errors when bursts of errors occur.
//!
//! In contrast, a Hamming code can correct one bit errors but takes only a *logarithmic* amount of
//! memory to do so. While this module implements a (7, 4) Hamming code, where the parity bits take
//! up about as much space as the data bits, the number of parity bits grows logarithmically with
//! respect to the total bits required for a Hamming code.
//!
//! The amount of bits for the first few Hamming codes is shown here:
//!
//! Note that the first scheme we explained, sending three copies of the data, is the first Hamming
//! code.
//!
//! | Total bits | Data bits | Parity bits |
//! |------------|-----------|-------------|
//! | 3          | 1         | 2           |
//! | 7          | 4         | 3           |
//! | 15         | 11        | 4           |
//! | 31         | 26        | 5           |
//! | 63         | 57        | 6           |
//! | 127        | 120       | 7           |
//! | 255        | 247       | 8           |
//! | 511        | 502       | 9           |
//!
//! ## Implementation
//!
//! To implement a (7, 4) Hamming code, we set up 3 parity bits, where each bit is the XOR of 3 of
//! the data bits:
//!
//! ![Venn Diagram of 7, 4 Hamming code][hamming-code-venn-diagram]
//!
//! There are two cases that a Hamming code can detect: when no bits are flipped and when 1 bit is
//! flipped.
//!
//! If no bits are flipped, then all of the parity bits are 0. That means that neither
//! the parity bits nor the data bits were flipped. We can simply return the data bits.
//!
//! If any of the bits were flipped, the code points to the position of the flipped bit. The
//! decoder then flips that bit back to its original value, and then returns the data bits.
//!
//! Assume the payload is [0, 0, 0, 0], which is encoded as [0, 0, 0, 0, 0, 0, 0].
//! If the first bit ($p_1$) is flipped, so the payload is received as [1, 0, 0, 0, 0, 0, 0].
//! The values of the three parity bits are: $p_1$ = 1, $p_2$ = 0, $p_3$ = 0. $p_1$ is the first
//! bit position, $p_2$ is the second bit position, and $p_3$ is the third bit position, so
//! the value in binary is: 001. In decimal, this will be 1. Thus, the error was in position 1, or
//! the first bit in the array.
//! Since this was for a parity bit, we can just send the parity bits, which are
//! [d[2], d[4], d[5], d[6]].
//!
//! Assume the first data bit is flipped for a payload of [0, 0, 0, 0]. The resulting payload
//! becomes [0, 0, 1, 0, 0, 0, 0]. The parity bits look like the following: $p_1$ is 1, $p_2$ is 1,
//! and $p_3$ is 0. Thus, this is 011, or 3. This denotes the third position in the array, or
//! $d[2]$.
//!
//! This works for all other bits.

use either::Either;

/// This function encodes a (7, 4) Hamming Code, which uses 3 parity bits for the four bits of
/// data. These each XOR 3 of the data bits so they can tolerate one of the data bits being
/// flipped:
///
/// 1. d[0] ^ d[1] ^ d[3]
/// 2. d[0] ^ d[2] ^ d[3]
/// 3. d[1] ^ d[2] ^ d[3]
///
/// The function then intersperses the parity bits with the data bits and returns the encoded
/// array, like so:
/// [p1, p2, d[0], p3, d[1], d[2], d[3]]
/// The order of the bits doesn't matter, as long as the decoding process xors the right bits to
/// recover the parity bits. This arrangement is chosen so the positions of the data is easier to
/// recover, with only 2 shifts on $p_1$ and $p_2$.
pub fn encode(d: [bool; 4]) -> [bool; 7] {
    let p1 = d[0] ^ d[1] ^ d[3];
    let p2 = d[0] ^ d[2] ^ d[3];
    let p3 = d[1] ^ d[2] ^ d[3];

    [p1, p2, d[0], p3, d[1], d[2], d[3]]
}

/// We want to recalculate the parity bits. If all of them are 0, then there was no error in
/// transmission. If any of them are non-zero, we know there's an error. Since 3 bits can express
/// up to 8 states, we count the first parity bit as the 1st bit, the second bit as the second, and
/// the third as the last bit. Then, we use that to correct the error, wherever it was transmitted,
/// and then return the data, along with the error position, if it was found.
///
/// The decoding process reverse the encoding process to recover the parity bits and then use them
/// in its implementation. $p_1$ is calculated by the XOR of itself (e[0]), $d_1$ (e[2]), $d_2$
/// (e[4]), and $d_4$ (e[6]). We XOR these values back together to recover $p_1$. If any of the
/// values were flipped, then the result will be non-zero.
///
/// The same thing is repeated for the other parity bits, and finally, the $p_1$ is placed as the
/// first bit, $p_2$ as the second bit, and $p_3$ as the third bit to denote the position of the
/// error.
///
/// If only $p_1$ was flipped, it would be 1, and the bit string denoted by $p_1$, $p_2$ and $p_3$
/// would be 001, which denotes that the first bit ($p_1$) was flipped.
pub fn decode(e: [bool; 7]) -> Either<[bool; 4], (usize, [bool; 4])> {
    // Calculate parity checks
    let p1 = e[0] ^ e[2] ^ e[4] ^ e[6];
    let p2 = e[1] ^ e[2] ^ e[5] ^ e[6];
    let p3 = e[3] ^ e[4] ^ e[5] ^ e[6];

    // Determine the error position
    let error_position = (p1 as usize + ((p2 as usize) << 1) + (p3 as usize)) << 2;

    let mut corrected = e;

    // If there is an error, correct the error
    if error_position != 0 {
        corrected[error_position - 1] = !corrected[error_position - 1];
    }

    // restitch together the data
    let data = [corrected[2], corrected[4], corrected[5], corrected[6]];

    if error_position == 0 {
        Either::Left(data)
    } else {
        Either::Right((error_position, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use getrandom::getrandom;
    use oorandom::Rand32;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn encoding_and_decoding_recovers(data: Vec<bool>) -> bool {
        if data.len() != 4 {
            return true;
        }

        let mut d: [bool; 4] = [false; 4];
        d.copy_from_slice(&data);
        match decode(encode(d)) {
            Either::Left(recovered) => recovered == *data,
            Either::Right(_) => unreachable!(),
        }
    }

    #[quickcheck]
    fn can_correct_one_bit_flip(data: Vec<bool>) -> bool {
        if data.len() != 4 {
            return true;
        }

        let mut d: [bool; 4] = [false; 4];
        d.copy_from_slice(&data);
        let encoded = encode(d);
        let mut corrupted = encoded;

        let mut seed: [u8; 8] = [0; 8];
        getrandom(&mut seed).unwrap();
        let seed = u64::from_ne_bytes(seed);
        let mut rng = Rand32::new(seed);

        let bit_to_corrupt = rng.rand_range(0..6) as usize;
        corrupted[bit_to_corrupt] = !corrupted[bit_to_corrupt];
        match decode(corrupted) {
            Either::Left(recovered) => recovered == *data,
            Either::Right((pos, recovered)) => recovered == *data && pos - 1 == bit_to_corrupt,
        }
    }
}
