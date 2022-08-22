use zeroize::Zeroizing;

use crate::traits::Curve;

pub struct Point<E: Curve>(E::Point);

pub struct Scalar<E: Curve>(E::Scalar);

#[cfg(feature = "std")]
pub struct SecretScalar<E: Curve>(Box<Zeroizing<E::Scalar>>);

pub struct NonZero<T>(T);
