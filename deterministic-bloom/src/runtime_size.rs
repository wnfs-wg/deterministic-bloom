use crate::{
    common::{BloomParams, HashIndexIterator},
    utils::HexFieldDebug,
};
use bitvec::{prelude::Lsb0, view::BitView};
use std::fmt::Debug;

//------------------------------------------------------------------------------
// Type Definitions
//------------------------------------------------------------------------------

/// An implementation of a basic [bloom filter].
///
/// Its size can be chosen (or made optimal for given parameters) at creation time,
/// but its size will have to stay the same during usage. I.e. you need to know
/// what your target capacity and false positive rates should be in advance.
///
/// Unlike the [`const_size::BloomFilter`](crate::const_size::BloomFilter) however,
/// this implementation doesn't require you to know the parameters at compile time.
///
/// # Example
///
/// ```
/// use deterministic_bloom::runtime_size::BloomFilter;
///
/// let mut filter = BloomFilter::new_from_fpr(1_000, 1.0 / 1_000_000.0);
/// filter.insert(b"Hello, World!");
///
/// assert!(filter.contains(b"Hello, World!"));
/// assert!(!filter.contains(b"Hello?")); // true in all but 1 in a million cases
/// ```
///
/// [bloom filter]: https://en.wikipedia.org/wiki/Bloom_filter
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BloomFilter {
    k_hashes: usize,
    bytes: Box<[u8]>,
}

impl BloomFilter {
    /// Construct a bloom filter with optimal parameters for given maximum capacity `n_elems`
    /// and false positive rate `fpr`.
    ///
    /// `n_elems` must be non-zero and `fpr` must be a number between 0 and 1 exclusive.
    ///
    /// # Example
    ///
    /// ```
    /// use deterministic_bloom::runtime_size::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new_from_fpr(1_000_000, 1.0 / 1_000_000.0);
    ///
    /// assert_eq!(filter.as_bytes().len(), 3594397);
    /// assert_eq!(filter.hash_count(), 20);
    /// ```
    pub fn new_from_fpr(n_elems: u64, fpr: f64) -> Self {
        let params = BloomParams::new_from_fpr(n_elems, fpr);
        let bits = Box::from(vec![0u8; params.byte_size].as_ref());
        Self {
            k_hashes: params.k_hashes,
            bytes: bits,
        }
    }

    /// Construct an optimal power-of-two (po2) sized bloom filter for given maximum capacity
    /// `n_elems` and false positive rate `fpr`.
    ///
    /// `n_elems` must be non-zero and `fpr` must be a number between 0 and 1 exclusive.
    ///
    /// Using a power-of-two size can be beneficial due to not requiring rejection sampling
    /// when generating the hash indicies for items inserted into the filter.
    ///
    /// # Example
    ///
    /// ```
    /// use deterministic_bloom::runtime_size::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new_from_fpr_po2(10_000_000, 0.01);
    ///
    /// assert_eq!(filter.as_bytes().len(), 16_777_216);
    /// assert_eq!(filter.as_bytes().len().count_ones(), 1); // size is a power of two
    /// assert_eq!(filter.hash_count(), 10);
    /// ```
    pub fn new_from_fpr_po2(n_elems: u64, fpr: f64) -> Self {
        let params = BloomParams::new_from_fpr_po2(n_elems, fpr);
        let bits = Box::from(vec![0u8; params.byte_size].as_ref());
        Self {
            k_hashes: params.k_hashes,
            bytes: bits,
        }
    }

    /// Construct a bloom filter with given target size and target capacity, both must
    /// be non-zero.
    ///
    /// This will compute the optimal number of hash evaluations per item inserted, but the
    /// false positive rate completely depends on the given filter size to maximum capacity
    /// ratio.
    ///
    /// # Example
    ///
    /// ```
    /// use deterministic_bloom::runtime_size::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new_from_size(1000, 1000);
    ///
    /// // False positive rate isn't controlled though:
    /// assert!((filter.false_positive_rate_at(1000) - 0.0215).abs() < 1e-4);
    /// ```
    pub fn new_from_size(bloom_bytes: usize, n_elems: u64) -> Self {
        let params = BloomParams::new_from_size(bloom_bytes, n_elems);
        let bits = Box::from(vec![0u8; params.byte_size].as_ref());
        Self {
            k_hashes: params.k_hashes,
            bytes: bits,
        }
    }

    /// Construct the bloom filter from existing components.
    ///
    /// This is useful when e.g. deserializing a bloom filter.
    ///
    /// # Example
    ///
    /// ```
    /// use deterministic_bloom::runtime_size::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new_from_fpr_po2(100_000, 0.01);
    ///
    /// filter.insert(b"Hello, World!");
    ///
    /// // Serialize
    /// let k_hashes = filter.hash_count();
    /// let bytes = Box::from(filter.as_bytes());
    ///
    /// // Deserialize
    /// let filter2 = BloomFilter::new_with(k_hashes, bytes);
    /// assert_eq!(filter, filter2);
    /// ```
    pub fn new_with(k_hashes: usize, bytes: Box<[u8]>) -> Self {
        Self { k_hashes, bytes }
    }

    /// Compute the bloom parameters for this bloom filter.
    /// This contains information about its size and hash function evaluations per
    /// item (`k_hashes`).
    pub fn get_bloom_params(&self) -> BloomParams {
        BloomParams {
            k_hashes: self.k_hashes,
            byte_size: self.bytes.len(),
        }
    }

    /// Get the approximate false positive rate at given capacity for this bloom filter.
    /// Returns a number between 0 and 1.
    pub fn false_positive_rate_at(&self, n_elems: u64) -> f64 {
        self.get_bloom_params().false_positive_rate_at(n_elems)
    }

    /// Get the approximate false positive rate at the current capacity of this bloom filter.
    /// Returns a number between 0 and 1.
    pub fn current_false_positive_rate(&self) -> f64 {
        let m = (self.bytes.len() * 8) as f64;
        let m_set = self.count_ones() as f64;
        let load = m_set / m;
        load.powi(self.hash_count() as i32)
    }

    /// Counts the amount of bits set in the bloom filter.
    pub fn count_ones(&self) -> usize {
        self.bytes.view_bits::<Lsb0>().count_ones()
    }

    /// Insert an element into the bloom filter.
    ///
    /// The element will be hashed, thus it needs to be representable as bytes.
    ///
    /// Note: If you're using the bloom filter in a non-trusted
    /// environment, so e.g. the items can be chosen by an adversary, please
    /// make sure to pre-hash your items with a cryptographic hashing function
    /// like SHA-256 or BLAKE3.
    /// Otherwise an adversary will be able to generate elements that cause
    /// the bloom filter to e.g. be unusually full with an unusually high false
    /// positive rate or cheaply generate elements that are false positives.
    ///
    /// # Example
    ///
    /// ```
    /// use deterministic_bloom::runtime_size::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new_from_fpr(1000, 0.0001);
    ///
    /// for i in 0u32..1000 {
    ///     filter.insert(&i.to_le_bytes());
    /// }
    ///
    /// // Slightly more than half filled with zeros
    /// assert_eq!(filter.as_bytes().len() / 2 * 8, filter.count_ones() - 322);
    ///
    /// assert!(filter.contains(&10u32.to_le_bytes()));
    /// assert!(!filter.contains(&1001u32.to_le_bytes())); // Except in 0.01%
    /// ```
    pub fn insert(&mut self, item: &impl AsRef<[u8]>) {
        for i in self.hash_indices(item) {
            self.bytes.view_bits_mut::<Lsb0>().set(i, true);
        }
    }

    /// Check whether an element was added into the bloom filter.
    ///
    /// This will always return true if the element was indeed added before,
    /// but it *may* sometimes return true, even if it wasn't.
    /// This is called a false positive and the false positive rate
    /// at certain capacities can be checked with [`false_positive_rate_at`](BloomFilter::false_positive_rate_at)
    /// and a desired false positive rate can be configured on creation with
    /// [`new_from_fpr`](BloomFilter::new_from_fpr) or [`new_from_fpr_po2`](BloomFilter::new_from_fpr_po2).
    ///
    /// # Example
    ///
    /// ```
    /// use deterministic_bloom::runtime_size::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new_from_fpr(100, 0.1); // very high false-positive rate
    ///
    /// for i in 0u32..100 {
    ///     filter.insert(&i.to_le_bytes());
    /// }
    ///
    /// // Inserted items will always return true
    /// assert!(filter.contains(&50u32.to_le_bytes()));
    /// // Non-inserted items mostly return false, but sometimes true
    /// assert!(!filter.contains(&101u32.to_le_bytes()));
    /// // But sometimes there exist false positives (in this case 10% of the time)
    /// assert!(filter.contains(&106u32.to_le_bytes()));
    /// ```
    pub fn contains(&self, item: &impl AsRef<[u8]>) -> bool {
        for i in self.hash_indices(item) {
            if !self.bytes.view_bits::<Lsb0>()[i] {
                return false;
            }
        }
        true
    }

    /// Returns how many hash function invocations are used pre item inserted
    pub fn hash_count(&self) -> usize {
        self.k_hashes
    }

    /// Return the underlying array used to store the bloom bits (always on the heap)
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the indices that a given element would set in the filter
    pub fn hash_indices<'a>(&self, item: &'a impl AsRef<[u8]>) -> impl Iterator<Item = usize> + 'a {
        HashIndexIterator::new(item, self.bytes.len() * 8).take(self.hash_count())
    }
}

impl Debug for BloomFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BloomFilter")
            .field("k_hashes", &self.k_hashes)
            .field("bytes", &HexFieldDebug(&self.bytes))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::BloomFilter;

    #[test]
    fn serialization_round_trip() {
        let mut filter = BloomFilter::new_from_fpr(100, 0.001);
        filter.insert(b"Hello");
        filter.insert(b"World!");
        let serialized_bytes = filter.as_bytes();
        let serialized_k = filter.hash_count();
        let deserialized = BloomFilter::new_with(serialized_k, Box::from(serialized_bytes));
        assert!(deserialized.contains(b"Hello"));
        assert!(!deserialized.contains(b"abc"));
        assert_eq!(deserialized, filter);
    }

    #[test]
    fn empty_bloom_filter() {
        let filter = BloomFilter::new_with(3, Box::new([]));
        // Technically an empty bloom "contains" anything, since everything is a false positive.
        assert!(filter.contains(&[1, 2, 3]));
    }
}

#[cfg(test)]
mod proptests {
    use super::BloomFilter;
    use proptest::prop_assert;
    use test_strategy::proptest;

    #[proptest]
    fn inserted_always_contained(items: Vec<u64>, #[strategy(100usize..10_000)] size: usize) {
        let capacity = std::cmp::max(items.len() as u64, 1);
        let mut filter = BloomFilter::new_from_size(size, capacity);

        for item in items.iter() {
            filter.insert(&item.to_le_bytes());
        }

        for item in items.iter() {
            prop_assert!(filter.contains(&item.to_le_bytes()));
        }
    }

    #[proptest]
    fn false_positive_rate_as_predicted(
        #[strategy(100u64..1_000)] n_elems: u64,
        #[strategy(100.0..10_000.0)] inv_fpr: f64,
    ) {
        let fpr = 1.0 / inv_fpr;
        let mut filter = BloomFilter::new_from_fpr(n_elems, fpr);

        for i in 0..n_elems {
            filter.insert(&i.to_le_bytes());
        }

        let measurements = 100_000;
        let mut false_positives = 0;

        for i in n_elems..n_elems + measurements {
            if filter.contains(&i.to_le_bytes()) {
                false_positives += 1;
            }
        }

        let computed_fpr = false_positives as f64 / measurements as f64;
        // The actual FPR should be pretty close
        prop_assert!((computed_fpr - fpr).abs() < 1.5e-3);
    }
}
