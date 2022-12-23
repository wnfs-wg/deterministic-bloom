//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

// use deterministic_bloom::{test_utils::Rvg, BloomFilter};
use deterministic_bloom_wasm::SmallBloomFilter;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_contains() {
    let mut bloom = SmallBloomFilter::new();
    // let mut rvg = Rvg::new();

    let new_val: Vec<u8> = vec![1, 2, 3]; // rvg.sample(&(..255u8))];
    bloom.insert_vec(new_val.clone());

    for _ in 1..25 {
        bloom.insert_vec(vec![4, 5, 6]); // rvg.sample(&(..255u8))]);
    }

    assert!(bloom.contains(new_val.clone()));
}
