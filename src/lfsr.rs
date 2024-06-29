use crate::polynomial::P128;

#[derive(Debug, Clone, PartialEq)]
pub struct Lfsr(pub u64);

const POLYNOMIAL: P128 = P128(0x1000000000000001b);

impl Lfsr {
    pub fn new(mut seed: u64) -> Self {
        if seed == 0 {
            seed = 1;
        }

        Self(seed)
    }

    pub fn next(&mut self, bits: u64) -> u64 {
        debug_assert!(bits <= 64);
        let mut x = 0;
        for _ in 0..bits {
            let msb = self.0 >> 63;
            x = (x << 1) | msb;
            self.0 = (self.0 << 1) ^ if msb != 0 { POLYNOMIAL.0 as u64 } else { 0 };
        }
        x
    }

    pub fn prev(&mut self, bits: u64) -> u64 {
        debug_assert!(bits <= 64);
        let mut x = 0;
        for _ in 0..bits {
            let lsb = self.0 & 1;
            x = (x >> 1) | (lsb << (bits - 1));
            self.0 = (self.0 >> 1)
                ^ if lsb != 0 {
                    (POLYNOMIAL.0 >> 1) as u64
                } else {
                    0
                };
        }
        x
    }

    pub fn skip(&mut self, bits: u64) {
        // just iterate naively
        for _ in 0..bits {
            let msb = self.0 >> 63;
            self.0 = (self.0 << 1) ^ if msb != 0 { POLYNOMIAL.0 as u64 } else { 0 };
        }
    }

    pub fn skip_backwards(&mut self, bits: u64) {
        // just iterate naively
        for _ in 0..bits {
            let lsb = self.0 & 1;
            self.0 = (self.0 >> 1)
                ^ if lsb != 0 {
                    (POLYNOMIAL.0 >> 1) as u64
                } else {
                    0
                };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn lfsr_naive() {
        let mut lfsr64_naive = Lfsr::new(1);
        let buf = iter::repeat_with(|| lfsr64_naive.next(64))
            .take(8)
            .collect::<Vec<_>>();
        assert_eq!(
            buf,
            &[
                0x0000000000000001,
                0x000000000000001b,
                0x0000000000000145,
                0x0000000000001db7,
                0x0000000000011011,
                0x00000000001ab1ab,
                0x0000000001514515,
                0x000000001c6db6c7
            ]
        );
        let buf = iter::repeat_with(|| lfsr64_naive.prev(64))
            .take(8)
            .collect::<Vec<_>>();
        assert_eq!(
            buf,
            &[
                0x000000001c6db6c7,
                0x0000000001514515,
                0x00000000001ab1ab,
                0x0000000000011011,
                0x0000000000001db7,
                0x0000000000000145,
                0x000000000000001b,
                0x0000000000000001
            ]
        );
    }
}
