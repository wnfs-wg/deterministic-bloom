//! Test suite for the Web and headless browsers.

use deterministic_bloom_wasm::add;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(add(3, 2), 5);
    // console_log!("{}", "Test passes!");
}
