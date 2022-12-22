#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! Wasm/JS bindings for [BloomFilter]

use derive_more::{From, Into};
use deterministic_bloom::BloomFilter;
use std::boxed::Box;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

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

//------------------------------------------------------------------------------
// Macros
//------------------------------------------------------------------------------

/// Generate a monomorphic [BloomFilter] instance.
macro_rules! gen_bloom {
    ($name: ident, $n: expr, $k: expr) => {
        #[doc = concat!("A monomorphic wrapper for [BloomFilter]`<", stringify!($n), ", ", stringify!($k), ">`.")]
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
                $name::try_from(vec).map_err(|e| JsError::new(&e.to_string()))
            }

            #[doc = r"The (constant) size of the underlying [BloomFilter] in bytes"]
            #[doc = "```"]
            #[doc = concat!("use deterministic_bloom_wasm::", stringify!($name), ";")]
            #[doc = ""]
            #[doc = concat!("let size = ", stringify!($name), "::byte_count();")]
            #[doc = concat!("assert!(size.eq(&", stringify!($k), "));")]
            #[doc = "```"]
            pub fn byte_count() -> usize {
                $k
            }

            /// The number of hashes used in the underlying [BloomFilter]
            pub fn hash_count() -> usize {
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

        impl From<BloomFilter<$n, $k>> for $name {
            fn from(bloom: BloomFilter<$n, $k>) -> $name {
                $name {
                    boxed: Box::new(bloom)
                }
            }
        }

        impl TryFrom<Vec<u8>> for $name {
            type Error = deterministic_bloom::Error;

            fn try_from(vec: Vec<u8>) -> Result<Self, deterministic_bloom::Error> {
                <BloomFilter<$n, $k>>::try_from(vec).map($name::from)
            }
        }
    };
}

gen_bloom!(Oh, 32, 2);
gen_bloom!(Hai, 256, 30);
gen_bloom!(Thar, 1024, 128);
