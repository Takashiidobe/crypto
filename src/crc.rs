use const_for::const_for;
use gf256::p64;

/// The CRC-32 polynomial used in this implementation.
/// This is the standard polynomial `0x104C11DB7` used in Ethernet, ZIP, and other applications.
const POLYNOMIAL: p64 = p64(0x104c11db7);

/// A lookup table for fast CRC-32 computation.
const CRC_TABLE: [u32; 256] = {
    let mut table = [0; 256];
    const_for!(i in 0..table.len() => {
        let x = (i as u32).reverse_bits();
        let x = p64((x as u64) << 8).naive_rem(POLYNOMIAL).0 as u32;
        table[i] = x.reverse_bits();
    });

    table
};

/// Computes the CRC-32 checksum of the given data slice.
///
/// # Arguments
///
/// * `data` - A slice of bytes for which the CRC-32 checksum is to be calculated.
///
/// # Returns
///
/// The computed CRC-32 checksum as a `u32`.
///
/// # Example
///
/// ```
/// use crypto::crc::crc32;
/// 
/// let input = b"Hello World!";
/// let checksum = crc32(input);
/// assert_eq!(checksum, 0x1c291ca3);
/// ```
pub fn crc32(data: &[u8]) -> u32 {
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
