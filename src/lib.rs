#![cfg_attr(feature = "default", doc = include_str!("../README.md"))]

/// Re-export of the [`rusqlite`](https://crates.io/crates/rusqlite) crate to avoid version conflicts.
pub use rusqlite;

mod core;

#[cfg(feature = "md5")]
mod md5;

#[cfg(feature = "md5")]
pub use crate::md5::register_md5_function;

#[cfg(feature = "sha1")]
mod sha1;

#[cfg(feature = "sha1")]
pub use crate::sha1::register_sha1_function;

#[cfg(feature = "sha256")]
mod sha256;

#[cfg(feature = "sha256")]
pub use crate::sha256::register_sha256_function;

#[cfg(feature = "sha512")]
mod sha512;

#[cfg(feature = "sha512")]
pub use crate::sha512::register_sha512_function;
