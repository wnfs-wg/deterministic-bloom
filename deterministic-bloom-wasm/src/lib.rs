#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! deterministic-bloom-wasm

use derive_more::{From, Into};
use deterministic_bloom::BloomFilter;
use std::boxed::Box;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

macro_rules! gen_bloom {
    ($name: ident, $n: tt, $k: tt) => {
        #[wasm_bindgen]
        #[derive(Debug, Default, From, Into)]
        pub struct $name {
            pub(crate) boxed: Box<BloomFilter<$n, $k>>,
        }

        #[wasm_bindgen]
        impl $name {
            pub fn new() -> $name {
                Default::default()
            }

            pub fn try_from_vec(vec: Vec<u8>) -> Result<$name, JsError> {
                $name::try_from(vec).map_err(|_| JsError::new("err"))
            }

            pub fn byte_count() -> usize {
                $k
            }

            pub fn num_iterations() -> usize {
                $n
            }

            pub fn insert(bloom: &mut $name, new_val: Vec<u8>) -> () {
                bloom.boxed.insert(&new_val);
            }

            pub fn contains(bloom: &$name, item: Vec<u8>) -> bool {
                bloom.boxed.contains(&item)
            }

            pub fn count_ones(bloom: &$name) -> usize {
                bloom.boxed.count_ones()
            }

            pub fn as_bytes(bloom: &$name) -> Vec<u8> {
                bloom.boxed.as_bytes().to_vec()
            }
        }

        impl TryFrom<Vec<u8>> for $name {
            type Error = anyhow::Error;

            fn try_from(vec: Vec<u8>) -> Result<Self, anyhow::Error> {
                Ok($name {
                    boxed: Box::new(<BloomFilter<$n, $k>>::try_from(vec)?),
                })
            }
        }
    };
}

gen_bloom!(Oh, 32, 2);
gen_bloom!(Hai, 256, 30);
gen_bloom!(Thar, 1024, 128);

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
