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

/// Generate a monomorphic [BloomFilter] newtypes.
macro_rules! gen_bloom {
    ($name: ident, $n: expr, $k: expr) => {
        #[doc = concat!("A monomorphic wrapper for [BloomFilter]`s with a size of ", stringify!($n), " bytes and ", stringify!($k), " hash functions.")]
        #[wasm_bindgen]
        #[derive(Debug, Default, From, Into)]
        pub struct $name {
            pub(crate) boxed: Box<BloomFilter<$n, $k>>,
        }

        #[wasm_bindgen]
        impl $name {
            #[doc = concat!("Initialize a blank [", stringify!($name), "] (i.e. contains no elements).")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use deterministic_bloom_wasm::", stringify!($name), ";")]
            ///
            #[doc = concat!("let blank = ", stringify!($name), "::new();")]
            #[doc = concat!("assert_eq!(", stringify!($name), "::count_ones(&blank), 0);")]
            /// ```
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Default::default()
            }

            #[doc = concat!("Attempt to initialize a [", stringify!($name), "] with a starting array.")]
            #[doc = concat!("Fails with a [JsError] if the [Vec] is not exactly ", stringify!($n), " bytes long.")]
            pub fn try_from_vec(vec: Vec<u8>) -> Result<$name, JsError> {
                $name::try_from(vec).map_err(|e| JsError::new(&e.to_string()))
            }

            /// The (constant) size of the underlying [BloomFilter] in bytes.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use deterministic_bloom_wasm::", stringify!($name), ";")]
            ///
            #[doc = concat!("let size = ", stringify!($name), "::byte_count();")]
            #[doc = concat!("assert_eq!(size, ", stringify!($k), ");")]
            /// ```
            pub fn byte_count() -> usize {
                $k
            }

            /// The number of hashes used in the underlying [BloomFilter].
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use deterministic_bloom_wasm::", stringify!($name), ";")]
            ///
            #[doc = concat!("assert_eq!(", stringify!($name), "::hash_count(), ", stringify!($n), ");")]
            /// ```
            pub fn hash_count() -> usize {
                $n
            }

            /// Insert a new elememt into the underlying [BloomFilter].
            /// This [Vec] can be of any length.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use deterministic_bloom_wasm::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut bloom = ", stringify!($name), "::new();")]
            /// let item = vec![1, 2, 3, 4, 5];
            /// bloom.insert_vec(item.clone());
            ///
            /// assert!(bloom.contains(item.clone()));
            /// ```
            pub fn insert_vec(&mut self, new_val: Vec<u8>) -> () {
                self.boxed.insert(&new_val);
            }

            /// Check if some [Vec] is in the underlying [BloomFilter].
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use deterministic_bloom_wasm::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut bloom = ", stringify!($name), "::new();")]
            /// let item = vec![1, 2, 3, 4, 5];
            /// bloom.insert_vec(item.clone());
            ///
            /// assert!(bloom.contains(item.clone()));
            /// ```
            pub fn contains(&self, item: Vec<u8>) -> bool {
                self.boxed.contains(&item)
            }

            /// Count how many bits are set to 1 (sometimes called a `popcount`).
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use deterministic_bloom_wasm::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut bloom = ", stringify!($name), "::new();")]
            /// let item1 = vec![1, 2, 3, 4, 5];
            /// bloom.insert_vec(item1.clone());
            /// bloom.insert_vec(item1.clone());
            /// bloom.insert_vec(item1.clone());
            ///
            /// let item2 = vec![6, 7];
            /// bloom.insert_vec(item2.clone());
            ///
            /// let item3 = vec![8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
            /// bloom.insert_vec(item2.clone());
            ///
            #[doc = concat!("assert!(bloom.count_ones() >= ", $k, ");")]
            #[doc = concat!("assert!(bloom.count_ones() <= 3 * ", $k, ");")]
            /// ```
            pub fn count_ones(&self) -> usize {
                self.boxed.count_ones()
            }

            /// Retreive the underlying byte array.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use deterministic_bloom_wasm::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut bloom = ", stringify!($name), "::new();")]
            /// bloom.insert_vec(vec![1, 2, 3, 4, 5]);
            ///
            #[doc = concat!("assert_ne!(bloom.as_bytes(), vec![0; ", $n, "]);")]
            #[doc = concat!("assert_eq!(bloom.as_bytes().len(), ", $n, ");")]
            /// ```
            pub fn as_bytes(&self) -> Vec<u8> {
                self.boxed.as_bytes().to_vec()
            }
        }

        impl From<BloomFilter<$n, $k>> for $name {
            fn from(bloom: BloomFilter<$n, $k>) -> Self {
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

gen_bloom!(SmallBloomFilter, 256, 13);
gen_bloom!(MediumBloomFilter, 4096, 17);
gen_bloom!(LargeBloomFilter, 1048576, 23);
