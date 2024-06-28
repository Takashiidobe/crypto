pub struct MD5 {}

const SHIFTS: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, // r1
    5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, // r2
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, // r3
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, // r4
];

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

impl MD5 {
    fn pad_message(message: &[u8]) -> Vec<u8> {
        let message_length: u64 = message.len() as u64 * 8;
        let mut result = message.to_owned();

        result.push(0x80);

        while (result.len() * 8 + 64) % 512 != 0 {
            result.push(0);
        }

        for b in 0..8 {
            result.push((message_length >> (b * 8)) as u8);
        }

        result
    }

    pub fn hash(input: &[u8]) -> [u8; 16] {
        let mut a0 = 0x67452301u32;
        let mut b0 = 0xefcdab89u32;
        let mut c0 = 0x98badcfeu32;
        let mut d0 = 0x10325476u32;

        let padded_msg = Self::pad_message(input);

        for chunk in padded_msg.chunks(64) {
            // Little endian
            let m: Vec<u32> = chunk
                .chunks(4)
                .map(|b| {
                    ((b[3] as u32) << 24)
                        | ((b[2] as u32) << 16)
                        | ((b[1] as u32) << 8)
                        | (b[0] as u32)
                })
                .collect();

            let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);

            for i in 0..64 {
                let (mut f, g) = match i {
                    0..=15 => ((b & c) | ((!b) & d), i),
                    16..=31 => ((d & b) | ((!d) & c), (5 * i + 1) % 16),
                    32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                    _ => (c ^ (b | (!d)), (7 * i) % 16),
                };

                f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(m[g]);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(f.rotate_left(SHIFTS[i]));
            }

            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }

        let mut result = [0; 16];

        for (i, v) in [a0, b0, c0, d0].into_iter().enumerate() {
            result[i * 4] = v as u8;
            result[i * 4 + 1] = (v >> 8) as u8;
            result[i * 4 + 2] = (v >> 16) as u8;
            result[i * 4 + 3] = (v >> 24) as u8;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let res = MD5::hash(b"abc");
        assert_eq!(
            res,
            [
                0x90, 0x01, 0x50, 0x98, // first
                0x3c, 0xd2, 0x4f, 0xb0, // second
                0xd6, 0x96, 0x3f, 0x7d, // third
                0x28, 0xe1, 0x7f, 0x72, // fourth
            ]
        );
    }

    #[test]
    fn empty() {
        let res = MD5::hash(b"");
        assert_eq!(
            res,
            [
                0xd4, 0x1d, 0x8c, 0xd9, // first
                0x8f, 0x00, 0xb2, 0x04, // second
                0xe9, 0x80, 0x09, 0x98, // third
                0xec, 0xf8, 0x42, 0x7e, // fourth
            ]
        );
    }

    #[test]
    fn ex1() {
        let res = MD5::hash(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
        assert_eq!(
            res,
            [
                0x82, 0x15, 0xef, 0x07, // first
                0x96, 0xa2, 0x0b, 0xca, // second
                0xaa, 0xe1, 0x16, 0xd3, // third
                0x87, 0x6c, 0x66, 0x4a, // fourth
            ]
        );
    }
}
