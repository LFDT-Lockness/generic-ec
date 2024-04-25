//! # Elliptic curves
//!
//! This crate contains elliptic curves supported out-of-box by [`generic-ec` crate].
//! Refer to its documentation to learn more.
//!
//! [`generic-ec` crate]: https://docs.rs/generic-ec

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![no_std]

#[cfg(any(feature = "ed25519", feature = "rust-crypto"))]
mod utils;

#[cfg(feature = "ed25519")]
pub mod ed25519;
#[cfg(feature = "rust-crypto")]
pub mod rust_crypto;

#[cfg(feature = "secp256k1")]
pub use rust_crypto::Secp256k1;

#[cfg(feature = "secp256r1")]
pub use rust_crypto::Secp256r1;

#[cfg(feature = "stark")]
pub use rust_crypto::Stark;

#[cfg(feature = "ed25519")]
pub use ed25519::Ed25519;
