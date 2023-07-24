#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! Deterministic Bloom filters
//!
//! This Crate is intented as a solid basis for cache reproducability
//! and for underlying certain cryptographic primitives.

pub mod utils;

use crate::utils::ByteArrayVisitor;
use bitvec::{
    prelude::{BitArray, Lsb0},
    view::BitView,
};
use serde::{Deserialize, Serialize};
use std::{f64::consts::LN_2, fmt::Debug, ops::Index};
use xxhash_rust::xxh3;

//------------------------------------------------------------------------------
// Type Definitions
//------------------------------------------------------------------------------

/// The bloom filter is a probabilistic data structure that can be used to store a set of hashes.
///
/// `N` is the size of the bloom filter in bytes.
///
/// `K` is the number of bits to be set with each insert operation.
///
/// # Examples
///
/// ```
/// use deterministic_bloom::BloomFilter;
///
/// let mut filter = BloomFilter::<256, 30>::default();
/// filter.insert(&[0xF5u8; 32]);
///
/// assert!(filter.contains(&[0xF5u8; 32]));
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct BloomFilter<const N: usize, const K: usize> {
    /// The underlying `BitArray`
    pub bits: BitArray<[u8; N]>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynBloomFilter {
    parameters: BloomParameters,
    bytes: Box<[u8]>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BloomParameters {
    k_hashes: usize,
}

impl BloomParameters {
    pub fn new_from_fpr(n_elems: u64, fpr: f64) -> (usize, Self) {
        let byte_size = Self::optimal_byte_size(n_elems, fpr);
        let k_hashes = Self::optimal_k_hashes(byte_size * 8, n_elems);

        (byte_size, Self { k_hashes })
    }

    pub fn new_from_fpr_po2(n_elems: u64, fpr: f64) -> (usize, Self) {
        let byte_size = Self::optimal_byte_size(n_elems, fpr).next_power_of_two();
        let k_hashes = Self::optimal_k_hashes(byte_size * 8, n_elems);

        (byte_size, Self { k_hashes })
    }

    pub fn new_from_size(bloom_bytes: usize, n_elems: u64) -> Self {
        Self {
            k_hashes: Self::optimal_k_hashes(bloom_bytes * 8, n_elems),
        }
    }

    pub fn false_positive_rate(&self, bloom_bytes: usize, n_elems: u64) -> f64 {
        debug_assert!(bloom_bytes != 0);
        debug_assert!(n_elems != 0);

        let k = self.k_hashes as f64;
        let ki = self.k_hashes as i32;
        let m = (bloom_bytes * 8) as f64;
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

impl DynBloomFilter {
    pub fn new_from_fpr(n_elems: u64, fpr: f64) -> Self {
        let (bloom_bytes, parameters) = BloomParameters::new_from_fpr(n_elems, fpr);
        let bits = Box::from(vec![0u8; bloom_bytes].as_ref());
        Self {
            parameters,
            bytes: bits,
        }
    }

    pub fn new_from_fpr_po2(n_elems: u64, fpr: f64) -> Self {
        let (bloom_bytes, parameters) = BloomParameters::new_from_fpr_po2(n_elems, fpr);
        let bits = Box::from(vec![0u8; bloom_bytes].as_ref());
        Self {
            parameters,
            bytes: bits,
        }
    }

    pub fn new_from_size(bloom_bytes: usize, n_elems: u64) -> Self {
        let parameters = BloomParameters::new_from_size(bloom_bytes, n_elems);
        let bits = Box::from(vec![0u8; bloom_bytes].as_ref());
        Self {
            parameters,
            bytes: bits,
        }
    }

    pub fn false_positive_rate_at(&self, n_elems: u64) -> f64 {
        self.parameters
            .false_positive_rate(self.bytes.len(), n_elems)
    }

    pub fn current_false_positive_rate(&self) -> f64 {
        let m = (self.bytes.len() * 8) as f64;
        let m_set = self.count_ones() as f64;
        let load = m_set / m;
        load.powi(self.parameters.k_hashes as i32)
    }

    pub fn count_ones(&self) -> usize {
        self.bytes.view_bits::<Lsb0>().count_ones()
    }

    pub fn insert(&mut self, item: &impl AsRef<[u8]>) {
        for i in self.hash_indices(item) {
            self.bytes.view_bits_mut::<Lsb0>().set(i, true);
        }
    }

    pub fn contains(&self, item: &impl AsRef<[u8]>) -> bool {
        for i in self.hash_indices(item) {
            if !self.bytes.view_bits::<Lsb0>()[i] {
                return false;
            }
        }
        true
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn hash_indices<'a>(&self, item: &'a impl AsRef<[u8]>) -> impl Iterator<Item = usize> + 'a {
        HashIndexIterator::new(item, self.bytes.len() * 8).take(self.parameters.k_hashes)
    }
}

/// An iterator that generates indices into some bloom filter based on deterministic hashing of specified item.
///
/// `N` is the number of bytes in the bloom filter.
/// This is used to restrict generated value within bloomfilter index space bounds.
///
/// # Examples
///
/// ```
/// use deterministic_bloom::BloomFilter;
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

impl<const N: usize, const K: usize> BloomFilter<N, K> {
    /// Creates a new bloom filter with all bits unset.
    ///
    /// # Examples
    ///
    /// ```
    /// use deterministic_bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::new();
    /// filter.insert(&[0xF5u8; 32]);
    ///
    /// assert!(filter.contains(&[0xF5u8; 32]));
    /// ```
    pub fn new() -> Self {
        Self {
            bits: Default::default(),
        }
    }

    /// Inserts an item to the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use deterministic_bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    /// filter.insert(&[0xF5u8; 32]);
    ///
    /// assert!(filter.contains(&[0xF5u8; 32]));
    /// ```
    pub fn insert<T>(&mut self, item: &T)
    where
        T: AsRef<[u8]>,
    {
        for i in self.hash_indices(item) {
            self.bits.set(i, true);
        }
    }

    /// Returns the number of hash iterations the bloom filter uses to set bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use deterministic_bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    ///
    /// assert_eq!(filter.hash_count(), 30);
    /// ```
    pub const fn hash_count(&self) -> usize {
        K
    }

    /// Checks if the item is in the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use deterministic_bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    /// filter.insert(&[0xF5u8; 32]);
    ///
    /// assert!(filter.contains(&[0xF5u8; 32]));
    /// ```
    pub fn contains<T>(&self, item: &T) -> bool
    where
        T: AsRef<[u8]>,
    {
        self.hash_indices(item).all(|i| self.bits[i])
    }

    /// Counts the number of bits set in the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use deterministic_bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    /// filter.insert(&[0xF5u8; 32]);
    ///
    /// assert_eq!(filter.count_ones(), 30);
    /// ```
    pub fn count_ones(&self) -> usize {
        self.bits.count_ones()
    }

    /// Returns the indices of the bits that would be set if the item was inserted to the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use deterministic_bloom::BloomFilter;
    ///
    /// let filter = BloomFilter::<256, 30>::default();
    /// let indices = filter.hash_indices(&[0xF5u8; 32]);
    /// let indices = indices.collect::<Vec<_>>();
    ///
    /// assert_eq!(indices.len(), 30);
    /// ```
    #[inline]
    pub fn hash_indices<'a, T>(&self, item: &'a T) -> impl Iterator<Item = usize> + 'a
    where
        T: AsRef<[u8]>,
    {
        HashIndexIterator::new(item, N * 8).take(self.hash_count())
    }

    /// Get the bytes of the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use deterministic_bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    /// filter.insert(&[0xF5u8; 32]);
    ///
    /// let bytes = filter.as_bytes();
    /// assert_eq!(bytes.len(), 256);
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }
}

impl<const N: usize, const K: usize> TryFrom<Vec<u8>> for BloomFilter<N, K> {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let bits = BitArray::<[u8; N]>::new(bytes.try_into().map_err(|vec: Vec<u8>| {
            Error::VectorImportSizeMismatch {
                expected: N,
                actual: vec.len(),
            }
        })?);

        Ok(Self { bits })
    }
}

impl<const N: usize, const K: usize> Index<usize> for BloomFilter<N, K> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

impl<const N: usize, const K: usize> Default for BloomFilter<N, K> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, const K: usize> Serialize for BloomFilter<N, K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.bits.as_raw_slice())
    }
}

impl<'de, const N: usize, const K: usize> Deserialize<'de> for BloomFilter<N, K> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(BloomFilter::<N, K> {
            bits: BitArray::<[u8; N]>::new(deserializer.deserialize_bytes(ByteArrayVisitor::<N>)?),
        })
    }
}

impl<const N: usize, const K: usize> AsRef<[u8]> for &BloomFilter<N, K> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Helper newtype for rendering given debug field as hex string
struct HexFieldDebug<A: AsRef<[u8]>>(A);

impl<A: AsRef<[u8]>> Debug for HexFieldDebug<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in self.0.as_ref().iter() {
            write!(f, "{byte:02X}")?;
        }

        Ok(())
    }
}

impl<const N: usize, const K: usize> Debug for BloomFilter<N, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BloomFilter")
            .field(&HexFieldDebug(self))
            .finish()
    }
}

impl Debug for DynBloomFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynBloomFilter")
            .field("parameters", &self.parameters)
            .field("bits", &HexFieldDebug(&self.bytes))
            .finish()
    }
}

//------------------------------------------------------------------------------
// Tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloom_filter_can_insert_and_validate_item_existence() {
        let mut bloom = BloomFilter::<256, 30>::new();
        let items: Vec<String> = vec!["first".into(), "second".into(), "third".into()];
        items.iter().for_each(|item| {
            bloom.insert(item);
        });

        items.iter().for_each(|item| {
            assert!(bloom.contains(item));
        });

        assert!(!bloom.contains(b"irst"));
        assert!(!bloom.contains(b"secnd"));
        assert!(!bloom.contains(b"tird"));
    }

    #[test]
    fn serialized_bloom_filter_can_be_deserialized_correctly() {
        let mut bloom = BloomFilter::<256, 30>::new();
        let items: Vec<String> = vec!["first".into(), "second".into(), "third".into()];
        items.iter().for_each(|item| {
            bloom.insert(item);
        });

        let ipld = libipld::serde::to_ipld(&bloom).unwrap();
        let deserialized: BloomFilter<256, 30> = libipld::serde::from_ipld(ipld).unwrap();

        assert_eq!(deserialized, bloom);
    }
}

#[cfg(test)]
mod proptests {
    use super::HashIndexIterator;
    use crate::{BloomFilter, BloomParameters};
    use proptest::{collection::vec, prop_assert};
    use test_strategy::proptest;

    #[proptest]
    fn iterator_can_give_unbounded_number_of_indices(#[strategy(0usize..500)] count: usize) {
        let iter = HashIndexIterator::new(&"hello", 200);

        let indices = (0..20)
            .map(|_| (iter.clone().take(count).collect::<Vec<_>>(), count))
            .collect::<Vec<_>>();

        for (indices, count) in indices {
            assert_eq!(indices.len(), count);
        }
    }

    #[proptest(cases = 1000)]
    fn test_contains(#[strategy(vec(vec(0..255u8, 0..100), 26))] values: Vec<Vec<u8>>) {
        let mut bloom = BloomFilter::<256, 30>::new();

        for v in values.iter() {
            bloom.insert(v);
        }

        for v in values.iter() {
            assert!(bloom.contains(v));
        }
    }

    #[proptest(cases = 10_000)]
    fn bloom_params_fpr_calc_round_trips(
        #[strategy(100u64..1_000_000)] n_elems: u64,
        #[strategy(0.0..0.1)] fpr: f64,
    ) {
        if fpr == 0.0 {
            return Ok(());
        }

        let (size, params) = BloomParameters::new_from_fpr(n_elems, fpr);
        let fpr_computed = params.false_positive_rate(size, n_elems);

        // The computed FPR can differ from the target FPR due to
        // rounding errors and the fact that only multiple-of-8
        // bloom sizes are allowed.
        let fpr_diff = (fpr_computed - fpr).abs();
        // We're fine if it's within 15% of a margin-of-error.
        prop_assert!(fpr_diff < fpr * 0.15);
    }
}
