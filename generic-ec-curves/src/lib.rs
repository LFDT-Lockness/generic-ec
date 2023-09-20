#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]

#[cfg(feature = "rust-crypto")]
pub mod rust_crypto;

#[cfg(feature = "secp256k1")]
pub use rust_crypto::Secp256k1;

#[cfg(feature = "secp256r1")]
pub use rust_crypto::Secp256r1;

#[cfg(feature = "stark")]
pub use rust_crypto::Stark;
