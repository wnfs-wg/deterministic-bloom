#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! Deterministic Bloom filters
//!
//! This Crate is intented as a solid basis for cache reproducability
//! and for underlying certain cryptographic primitives.

pub mod common;
pub mod const_size;
pub mod runtime_size;

mod utils;
