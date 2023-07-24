use std::{f64::consts::LN_2, fmt::Debug};
use xxhash_rust::xxh3;

/// An iterator that generates indices into some bloom filter based on deterministic hashing of specified item.
///
/// # Examples
///
/// ```
/// use deterministic_bloom::const_size::BloomFilter;
///
/// let filter = BloomFilter::<256, 30>::default();
/// let indices = filter.hash_indices(&[0xF5u8; 32]);
/// let indices = indices.collect::<Vec<_>>();
///
/// assert_eq!(indices.len(), 30);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashIndexIterator<'a, T: AsRef<[u8]>> {
    item: &'a T,
    bit_size: usize,
    index: u64,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BloomParameters {
    /// size of the bloom filter in bytes, non-zero
    pub byte_size: usize,
    /// hashing functions used/number of bits set per element, non-zero
    pub k_hashes: usize,
}

/// Errors for [BloomFilter] operations.
#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    /// Report a size mismatch when importing a Bloom filter from a [Vec].
    #[error("Cannot convert vector to BloomFilter: expected {expected}, but got {actual}")]
    #[diagnostic(url(docsrs))]
    VectorImportSizeMismatch {
        /// The expected size in the [BloomFilter].
        expected: usize,

        /// The actual size of the [Vec].
        actual: usize,
    },
}

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl<'a, T: AsRef<[u8]>> HashIndexIterator<'a, T> {
    /// Creates a new iterator.
    pub fn new(item: &'a T, bit_size: usize) -> Self {
        Self {
            item,
            index: 0,
            bit_size,
        }
    }
}

impl<T: AsRef<[u8]>> Iterator for HashIndexIterator<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let bit_size_po2 = self.bit_size.next_power_of_two();
        loop {
            let hash = xxh3::xxh3_64_with_seed(self.item.as_ref(), self.index) as usize;
            self.index += 1;

            // Rejection sampling for non-power-of-two bit sizes
            let value = hash % bit_size_po2;
            if value < self.bit_size {
                return Some(value);
            }
        }
    }
}

impl BloomParameters {
    pub fn new_from_fpr(n_elems: u64, fpr: f64) -> Self {
        let byte_size = Self::optimal_byte_size(n_elems, fpr);
        let k_hashes = Self::optimal_k_hashes(byte_size * 8, n_elems);

        Self {
            byte_size,
            k_hashes,
        }
    }

    pub fn new_from_fpr_po2(n_elems: u64, fpr: f64) -> Self {
        let byte_size = Self::optimal_byte_size(n_elems, fpr).next_power_of_two();
        let k_hashes = Self::optimal_k_hashes(byte_size * 8, n_elems);

        Self {
            byte_size,
            k_hashes,
        }
    }

    pub fn new_from_size(byte_size: usize, n_elems: u64) -> Self {
        Self {
            byte_size,
            k_hashes: Self::optimal_k_hashes(byte_size * 8, n_elems),
        }
    }

    pub fn false_positive_rate_at(&self, n_elems: u64) -> f64 {
        debug_assert!(n_elems != 0);

        let k = self.k_hashes as f64;
        let ki = self.k_hashes as i32;
        let m = (self.byte_size * 8) as f64;
        let n = n_elems as f64;

        // see https://hur.st/bloomfilter/
        (1.0 - (-k / (m / n)).exp()).powi(ki)
    }

    fn optimal_byte_size(n_elems: u64, fpr: f64) -> usize {
        debug_assert!(n_elems != 0);
        debug_assert!(fpr > 0.0 && fpr < 1.0);

        let n = n_elems as f64;
        let bit_size = n * fpr.ln() / -(LN_2 * LN_2);
        (bit_size / 8.0).ceil() as usize
    }

    fn optimal_k_hashes(bloom_bits: usize, n_elems: u64) -> usize {
        debug_assert!(bloom_bits != 0);
        debug_assert!(n_elems != 0);

        let m = bloom_bits as f64;
        let n = n_elems as f64;
        let k_hashes = ((m / n) * LN_2).ceil() as usize;
        std::cmp::max(k_hashes, 1)
    }
}

#[cfg(test)]
mod proptests {
    use super::BloomParameters;
    use proptest::prop_assert;
    use test_strategy::proptest;

    #[proptest(cases = 10_000)]
    fn bloom_params_fpr_calc_round_trips(
        #[strategy(100u64..1_000_000)] n_elems: u64,
        #[strategy(0.0..0.1)] fpr: f64,
    ) {
        if fpr == 0.0 {
            return Ok(());
        }

        let params = BloomParameters::new_from_fpr(n_elems, fpr);
        let fpr_computed = params.false_positive_rate_at(n_elems);

        // The computed FPR can differ from the target FPR due to
        // rounding errors and the fact that only multiple-of-8
        // bloom sizes are allowed.
        let fpr_diff = (fpr_computed - fpr).abs();
        // We're fine if it's within 15% of a margin-of-error.
        prop_assert!(fpr_diff < fpr * 0.15);
    }
}
