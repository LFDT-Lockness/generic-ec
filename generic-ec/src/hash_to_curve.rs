use crate::as_raw::{FromRaw, TryFromRaw};
use crate::ec_core::hash_to_curve::HashToCurve;
use crate::ec_core::Curve;
use crate::errors::{HashError, HashErrorReason};
use crate::{Point, Scalar};

#[doc(inline)]
pub use crate::ec_core::hash_to_curve::Tag;

pub trait FromHash
where
    Self: Sized,
{
    #[inline]
    fn hash(tag: Tag, message: &[u8]) -> Result<Self, HashError> {
        Self::hash_concat(tag, &[message])
    }
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
