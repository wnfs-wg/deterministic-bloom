#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! deterministic-bloom-wasm

use derive_more::{From, Into};
use deterministic_bloom::BloomFilter;
use std::boxed::Box;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct WasmBloomFilter {
    bytes: Box<BloomFilter<256, 30>>,
}

#[wasm_bindgen]
pub fn insert(mut bloom: WasmBloomFilter, new_val: Vec<u8>) -> Result<WasmBloomFilter, JsValue> {
    let new_slice: [u8; 32] = new_val.try_into().map_err(|_| JsValue::from_str("nope"))?;
    bloom.bytes.add(&new_slice);
    Ok(bloom)
}

// impl<T> TryFrom<T> for WasmBloomFilter
// where
//     BloomFilter<256, 30>: TryFrom<T>,
// {
//     type Error = <BloomFilter<256, 30> as TryFrom<T>>::Error;
//
//     fn try_from(t: T) -> Result<Self, Self::Error> {
//         WasmBloomFilter {
//             bytes: t.try_into()?,
//         }
//     }
// }

// #[wasm_bindgen]
// pub fn init(size: usize) -> WasmBloomFilter {
//     let bytes = Vec::with_capacity(size);
//     WasmBloomFilter {
//         bytes: Rc::new(RefCell::new(bytes.as_slice())),
//     }
// }

//------------------------------------------------------------------------------
// Utilities
//------------------------------------------------------------------------------

/// Panic hook lets us get better error messages if our Rust code ever panics.
///
/// For more details see
/// <https://github.com/rustwasm/console_error_panic_hook#readme>
#[wasm_bindgen(js_name = "setPanicHook")]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    // For alerting
    pub(crate) fn alert(s: &str);

    // For logging in the console.
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

//------------------------------------------------------------------------------
// Macros
//------------------------------------------------------------------------------

/// Return a representation of an object owned by JS.
#[macro_export]
macro_rules! value {
    ($value:expr) => {
        wasm_bindgen::JsValue::from($value)
    };
}

/// Calls the wasm_bindgen console.log.
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}
