use crate::{
    common::{Error, HashIndexIterator},
    utils::{ByteArrayVisitor, HexFieldDebug},
};
use bitvec::prelude::BitArray;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Index};

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
/// use deterministic_bloom::const_size::BloomFilter;
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

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl<const N: usize, const K: usize> BloomFilter<N, K> {
    /// Creates a new bloom filter with all bits unset.
    ///
    /// # Examples
    ///
    /// ```
    /// use deterministic_bloom::const_size::BloomFilter;
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
    /// use deterministic_bloom::const_size::BloomFilter;
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
    /// use deterministic_bloom::const_size::BloomFilter;
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
    /// use deterministic_bloom::const_size::BloomFilter;
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
    /// use deterministic_bloom::const_size::BloomFilter;
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
    /// use deterministic_bloom::const_size::BloomFilter;
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
    /// use deterministic_bloom::const_size::BloomFilter;
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

impl<const N: usize, const K: usize> Debug for BloomFilter<N, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BloomFilter")
            .field(&HexFieldDebug(self))
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
    use super::BloomFilter;
    use crate::common::HashIndexIterator;
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
}
