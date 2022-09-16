use zeroize::Zeroizing;

use crate::ec_core::Curve;

#[cfg(feature = "std")]
pub struct SecretScalar<E: Curve>(Box<Zeroizing<E::Scalar>>);

pub struct NonZero<T>(T);
