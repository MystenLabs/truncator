pub trait ByteStatisticsSummator {
    /// Output the sum of any summation strategy function over a bytes slice.
    fn sum(&self, bytes: &[u8]) -> u32;
}
