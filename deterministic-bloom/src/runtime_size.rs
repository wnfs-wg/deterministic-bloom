use crate::{
    common::{BloomParams, HashIndexIterator},
    utils::HexFieldDebug,
};
use bitvec::{prelude::Lsb0, view::BitView};
use std::fmt::Debug;

//------------------------------------------------------------------------------
// Type Definitions
//------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BloomFilter {
    k_hashes: usize,
    bytes: Box<[u8]>,
}

impl BloomFilter {
    pub fn new_from_fpr(n_elems: u64, fpr: f64) -> Self {
        let params = BloomParams::new_from_fpr(n_elems, fpr);
        let bits = Box::from(vec![0u8; params.byte_size].as_ref());
        Self {
            k_hashes: params.k_hashes,
            bytes: bits,
        }
    }

    pub fn new_from_fpr_po2(n_elems: u64, fpr: f64) -> Self {
        let params = BloomParams::new_from_fpr_po2(n_elems, fpr);
        let bits = Box::from(vec![0u8; params.byte_size].as_ref());
        Self {
            k_hashes: params.k_hashes,
            bytes: bits,
        }
    }

    pub fn new_from_size(bloom_bytes: usize, n_elems: u64) -> Self {
        let params = BloomParams::new_from_size(bloom_bytes, n_elems);
        let bits = Box::from(vec![0u8; params.byte_size].as_ref());
        Self {
            k_hashes: params.k_hashes,
            bytes: bits,
        }
    }

    pub fn get_bloom_params(&self) -> BloomParams {
        BloomParams {
            k_hashes: self.k_hashes,
            byte_size: self.bytes.len(),
        }
    }

    pub fn false_positive_rate_at(&self, n_elems: u64) -> f64 {
        self.get_bloom_params().false_positive_rate_at(n_elems)
    }

    pub fn current_false_positive_rate(&self) -> f64 {
        let m = (self.bytes.len() * 8) as f64;
        let m_set = self.count_ones() as f64;
        let load = m_set / m;
        load.powi(self.hash_count() as i32)
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

    pub fn hash_count(&self) -> usize {
        self.k_hashes
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn hash_indices<'a>(&self, item: &'a impl AsRef<[u8]>) -> impl Iterator<Item = usize> + 'a {
        HashIndexIterator::new(item, self.bytes.len() * 8).take(self.hash_count())
    }
}

impl Debug for BloomFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynBloomFilter")
            .field("k_hashes", &self.k_hashes)
            .field("bits", &HexFieldDebug(&self.bytes))
            .finish()
    }
}
