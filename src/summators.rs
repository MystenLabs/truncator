use crate::traits::ByteStatisticsSummator;

// const flags used to unset bits of a byte.
const FIRST_4_BITS_SET: u8 = 0b11110000u8;
const LAST_4_BITS_SET: u8 = 0b00001111u8;
const FIRST_2_BITS_SET: u8 = 0b11000000u8;
const SECOND_2_BITS_SET: u8 = 0b00110000u8;
const THIRD_2_BITS_SET: u8 = 0b00001100u8;
const LAST_2_BITS_SET: u8 = 0b00000011u8;

/// Get the num of set bits of a slice, which simulates Lamport+ (Winternitz sum for w = 2).
pub struct SetBitsSummator();

/// Get the sum of a slice at half byte granularity, which simulates Winternitz sum for w = 16.
pub struct W16Summator();

/// Get the sum of a slice at single byte granularity, which simulates Winternitz sum for w = 256.
pub struct W256Summator();

/// Get the num of "00" or "11" in a slice.
pub struct Same2BitSummator();

/// Get the num of "0000" or "1111" in a slice.
pub struct Same4BitSummator();

/// Get the num of "0000_0000" or "1111_1111" bytes in a slice.
pub struct Same8BitSummator();

impl ByteStatisticsSummator for SetBitsSummator {
    fn sum(&self, bytes: &[u8]) -> u32 {
        bytes.iter().map(|b| b.count_ones()).sum()
        // The above is equivalent to this:
        // let mut sum = 0;
        // for b in bytes {
        //     sum += b.count_ones()
        // }
        // sum
    }
}

impl ByteStatisticsSummator for W16Summator {
    fn sum(&self, bytes: &[u8]) -> u32 {
        bytes.iter().map(|b| ((b >> 4) + (b & LAST_4_BITS_SET)) as u32).sum()
        // The above is equivalent to this:
        // let mut sum = 0;
        // for b in bytes {
        //     let left = (b >> 4) as u32;
        //     let right = (b << FOUR_LAST_BITS) as u32;
        //     sum += left + right;
        // }
        // sum
    }
}

impl ByteStatisticsSummator for W256Summator {
    fn sum(&self, bytes: &[u8]) -> u32 {
        bytes.iter().map(|b| *b as u32).sum()
    }
}

impl ByteStatisticsSummator for Same2BitSummator {
    fn sum(&self, bytes: &[u8]) -> u32 {
        bytes.iter().map(|b| {
            // TODO: this can be better optimized via OR statements.
            let mut sum = 0;
            let b0 = (b & FIRST_2_BITS_SET).count_ones();
            let b1 = (b & SECOND_2_BITS_SET).count_ones();
            let b2 = (b & THIRD_2_BITS_SET).count_ones();
            let b3 = (b & LAST_2_BITS_SET).count_ones();

            if b0 == 0 || b0 == 2 {
                sum += 1;
            }
            if b1 == 0 || b1 == 2 {
                sum += 1;
            }
            if b2 == 0 || b2 == 2 {
                sum += 1;
            }
            if b3 == 0 || b3 == 2 {
                sum += 1;
            }
            sum
        }).sum()
    }
}

impl ByteStatisticsSummator for Same4BitSummator {
    fn sum(&self, bytes: &[u8]) -> u32 {
        bytes.iter().map(|b| {
            let mut sum = 0;
            let b0 = (b & FIRST_4_BITS_SET).count_ones();
            let b1 = (b & LAST_4_BITS_SET).count_ones();

            if b0 == 0 || b0 == 4 {
                sum += 1;
            }
            if b1 == 0 || b1 == 4 {
                sum += 1;
            }
            sum
        }).sum()
    }
}

impl ByteStatisticsSummator for Same8BitSummator {
    fn sum(&self, bytes: &[u8]) -> u32 {
        bytes.iter().map(|b| {
            if *b == 0 || *b == 255 {
                1
            } else {
                0
            }
        }).sum()
    }
}
