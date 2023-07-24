#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! Deterministic Bloom filters
//!
//! This Crate is intented as a solid basis for cache reproducability
//! and for underlying certain cryptographic primitives.

/// Some structs and implementations that multiple bloom implementations can depend on
pub mod common;
/// Bloom filters with compile-time-determinted parameters (size & hash count)
pub mod const_size;
/// Bloom filters with runtime-determined parameters. Their size can be chosen
/// arbitrarily at runtime, but not be modified during use (they're not resizable).
pub mod runtime_size;

mod utils;
