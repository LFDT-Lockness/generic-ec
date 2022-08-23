#![cfg_attr(not(feature = "std"), no_std)]

use generic_ec_core as ec_core;

pub mod coords;
pub mod dummy_curve;
pub mod errors;
#[cfg(feature = "serde")]
mod serde_utils;
pub mod wrappers;
