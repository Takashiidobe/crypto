#[derive(Default, Clone, Copy, PartialEq)]
pub struct Sha1;

impl Sha1 {
    // SHA-1 hashing algorithm initial hash values.
    // These constants are derived from the fractional parts of the square roots of the first five primes.
    const H0: u32 = 0x67452301;
    const H1: u32 = 0xEFCDAB89;
    const H2: u32 = 0x98BADCFE;
    const H3: u32 = 0x10325476;
    const H4: u32 = 0xC3D2E1F0;

    /// Computes the SHA-1 hash of the input string by taking in either a String of str type.
    pub fn hash(key: &[u8]) -> [u8; 20] {
        // 1. Initialize variables to the SHA-1's initial hash values.
        let (mut h0, mut h1, mut h2, mut h3, mut h4) =
            (Self::H0, Self::H1, Self::H2, Self::H3, Self::H4);

        // 2. Pad the key
        let msg = Self::pad_message(key);

        // 3. Process each 512-bit chunk of the padded message.
        for chunk in msg.chunks(64) {
            // 4. Get the message schedule and copies initial SHA-1 values.
            let schedule = Self::build_schedule(chunk);

            // 5. initialize the schedule
            let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);

            // 6. Main loop of the SHA-1 algorithm using predefind values based on primes numbers.
            for i in 0..80 {
                let (f, k) = match i {
                    0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                    20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                    40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                    _ => (b ^ c ^ d, 0xCA62C1D6),
                };

                // 7. Update the temporary variable and then update the hash values
                // in a manner that enforces both diffusion and confusion. Note
                // how the "scrambled" data trickles through the variables as we
                // loop through.
                let temp = a
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(k)
                    .wrapping_add(schedule[i]);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }

            // 8. Add the compressed chunk to the current hash value.
            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
        }

        // 9. Produce the final hash value as a 20-byte array.
        let mut hash = [0u8; 20];

        for (i, h) in [h0, h1, h2, h3, h4].iter().enumerate() {
            let (start, end) = (i * 4, (i + 1) * 4);
            hash[start..end].copy_from_slice(&h.to_be_bytes());
        }

        hash
    }

    /// Pads the input message according to SHA-1 specifications.
    fn pad_message(input: &[u8]) -> Vec<u8> {
        // 1. turn the input into a vec.
        let mut bytes = input.to_vec();

        // 2. Save the original message length
        let original_bit_length = bytes.len() as u64 * 8;

        // 3. Append the byte 0x80 (1) at the end to delineate the padding start
        bytes.push(0x80);

        // 4. Pad with '0' bytes until the message's length in bits % 512 is 448.
        while (bytes.len() * 8) % 512 != 448 {
            bytes.push(0);
        }

        // 5. Append the original message length.
        bytes.extend_from_slice(&original_bit_length.to_be_bytes());

        bytes
    }

    /// Builds the message schedule array from a 512-bit chunk.
    fn build_schedule(chunk: &[u8]) -> [u32; 80] {
        // 1. Initialize an empty schedule
        let mut schedule = [0u32; 80];

        // 2. Initialize the first 16 words in the array from the chunk.
        for (i, block) in chunk.chunks(4).enumerate() {
            schedule[i] = u32::from_be_bytes(block.try_into().unwrap());
        }

        // 3. Extend the schedule array using previously defined values and the XOR (^) operation.
        for i in 16..80 {
            schedule[i] = schedule[i - 3] ^ schedule[i - 8] ^ schedule[i - 14] ^ schedule[i - 16];
            schedule[i] = schedule[i].rotate_left(1);
        }

        schedule
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let res = Sha1::hash(b"abc");
        assert_eq!(
            res,
            [
                0xA9, 0x99, 0x3E, 0x36, // easier to see in fours
                0x47, 0x06, 0x81, 0x6A, // second
                0xBA, 0x3E, 0x25, 0x71, // third
                0x78, 0x50, 0xc2, 0x6c, // fourth
                0x9c, 0xd0, 0xd8, 0x9d, // fifth
            ]
        );
    }

    #[test]
    fn empty() {
        let res = Sha1::hash(b"");
        assert_eq!(
            res,
            [
                0xda, 0x39, 0xa3, 0xee, // first
                0x5e, 0x6b, 0x4b, 0x0d, // second
                0x32, 0x55, 0xbf, 0xef, // third
                0x95, 0x60, 0x18, 0x90, // fourth
                0xaf, 0xd8, 0x07, 0x09, // fifth
            ]
        );
    }

    #[test]
    fn ex1() {
        let res = Sha1::hash(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
        assert_eq!(
            res,
            [
                0x84, 0x98, 0x3e, 0x44, // first
                0x1c, 0x3b, 0xd2, 0x6e, // second
                0xba, 0xae, 0x4a, 0xa1, // third
                0xf9, 0x51, 0x29, 0xe5, // fourth
                0xe5, 0x46, 0x70, 0xf1, // fifth
            ]
        );
    }
}
