use crate::polynomial::P64;
use const_for::const_for;

pub fn crc32(data: &[u8]) -> u32 {
    const POLYNOMIAL: P64 = P64(0x104c11db7);

    const CRC_TABLE: [u32; 256] = {
        let mut table = [0; 256];
        const_for!(i in 0..table.len() => {
            let x = (i as u32).reverse_bits();
            let x = P64((x as u64) << 8).naive_rem(POLYNOMIAL).0 as u32;
            table[i] = x.reverse_bits();
        });

        table
    };

    let mut crc = 0xffffffff;

    for b in data {
        crc = (crc >> 8) ^ CRC_TABLE[usize::from((crc as u8) ^ b)];
    }

    crc ^ 0xffffffff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let input = b"Hello World!";
        let expected = 0x1c291ca3;
        assert_eq!(crc32(input), expected);
    }
}
