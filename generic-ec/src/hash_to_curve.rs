//! Hash to curve primitive according to [draft-irtf-cfrg-hash-to-curve](https://datatracker.ietf.org/doc/draft-irtf-cfrg-hash-to-curve/)
//!
//! Some curves have hash to curve primitive implemented. For those curves, `Point<E>` and `Scalar<E>` implement
//! [`FromHash` trait](FromHash).
//!
//! ## Example
//! ```rust
//! use generic_ec::{Point, Scalar, curves::Secp256k1};
//! use generic_ec::hash_to_curve::{FromHash, Tag};
//!
//! // Domain separation tag. Should be unique per application.
//! const TAG: Tag = Tag::new_unwrap(b"MYAPP-v0.1.0");
//!
//! let point = Point::<Secp256k1>::hash(TAG, b"data to be hashed")?;
//! let scalar = Scalar::<Secp256k1>::hash(TAG, b"other data to be hashed")?;
//!
//! # Ok::<_, generic_ec::errors::HashError>(())
//! ```

use crate::as_raw::{FromRaw, TryFromRaw};
use crate::core::hash_to_curve::HashToCurve;
use crate::core::Curve;
use crate::errors::{HashError, HashErrorReason};
use crate::{Point, Scalar};

#[doc(inline)]
pub use crate::core::hash_to_curve::Tag;

/// Hash to curve primitive
pub trait FromHash
where
    Self: Sized,
{
    /// Computes `H(message)`
    #[inline]
    fn hash(tag: Tag, message: &[u8]) -> Result<Self, HashError> {
        Self::hash_concat(tag, &[message])
    }
    /// Computes `H(message[0] || ... || message[len - 1])`
    fn hash_concat(tag: Tag, message: &[&[u8]]) -> Result<Self, HashError>;
}

impl<E> FromHash for Point<E>
where
    E: Curve + HashToCurve,
{
    #[inline]
    fn hash_concat(tag: Tag, message: &[&[u8]]) -> Result<Self, HashError> {
        let point =
            E::hash_to_curve(tag, message).or(Err(HashError(HashErrorReason::HashFailed)))?;
        Point::try_from_raw(point).ok_or(HashError(HashErrorReason::ProducedValueInvalid))
    }
}

impl<E> FromHash for Scalar<E>
where
    E: Curve + HashToCurve,
{
    #[inline]
    fn hash_concat(tag: Tag, message: &[&[u8]]) -> Result<Self, HashError> {
        let scalar =
            E::hash_to_scalar(tag, message).or(Err(HashError(HashErrorReason::HashFailed)))?;
        Ok(Scalar::from_raw(scalar))
    }
}
