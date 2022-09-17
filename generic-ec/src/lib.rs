#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(__docs, feature(doc_cfg, doc_auto_cfg))]

use generic_ec_core as ec_core;

mod arithmetic;
pub mod coords;
mod encoded;
pub mod errors;
mod generator;
mod non_zero;
mod point;
mod scalar;
#[cfg(feature = "serde")]
mod serde_support;
mod wrappers;

pub use self::{
    ec_core::Curve, encoded::EncodedPoint, generator::Generator, non_zero::definition::NonZero,
    point::definition::Point, scalar::definition::Scalar, wrappers::SecretScalar,
};
