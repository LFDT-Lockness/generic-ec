//! Generic purpose elliptic curve cryptography

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use generic_ec_core as ec_core;

mod arithmetic;
mod as_raw;
pub mod coords;
mod encoded;
pub mod errors;
mod generator;
pub mod hash_to_curve;
mod non_zero;
mod point;
mod scalar;
mod secret_scalar;
#[cfg(feature = "serde")]
pub mod serde;

pub use self::{
    ec_core::Curve,
    encoded::{EncodedPoint, EncodedScalar},
    generator::Generator,
    non_zero::definition::NonZero,
    point::definition::Point,
    scalar::Scalar,
    secret_scalar::definition::SecretScalar,
};

#[cfg(feature = "curves")]
pub mod curves {
    #[cfg(feature = "curve-secp256k1")]
    pub use generic_ec_curves::Secp256k1;
    #[cfg(feature = "curve-secp256r1")]
    pub use generic_ec_curves::Secp256r1;
}
