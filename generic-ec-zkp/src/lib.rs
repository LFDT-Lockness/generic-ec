#![cfg_attr(not(test), forbid(unused_crate_dependencies))]
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]

// We don't want this dependency to trigger unused dep lint
use generic_array as _;

pub mod hash_commitment;
pub mod schnorr_pok;
