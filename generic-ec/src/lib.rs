#![cfg_attr(not(feature = "std"), no_std)]

use generic_ec_core as ec_core;

mod arithmetic;
pub mod coords;
pub mod errors;
mod point;
mod scalar;
#[cfg(feature = "serde")]
mod serde_support;
mod wrappers;

pub use self::{
    ec_core::Curve,
    point::definition::Point,
    scalar::definition::Scalar,
    wrappers::{NonZero, SecretScalar},
};
