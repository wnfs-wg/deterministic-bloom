//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use deterministic_bloom_wasm;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(deterministic_bloom_wasm::add(3, 2), 5);
    deterministic_bloom_wasm::console_log!("{}", "Test passes!");
}
