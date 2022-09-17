use zeroize::Zeroize;

use crate::ec_core::*;

/// Scalar modulo curve `E` group order
///
/// Scalar is guaranteed to be non-negative integer modulo curve `E` group order.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroize)]
pub struct Scalar<E: Curve>(E::Scalar);

impl<E: Curve> Scalar<E> {
    /// Constructs a scalar without checking whether it's valid
    ///
    /// Caller **must** guarantee validity of the scalar. Caller **must** provide a comment
    /// justifying a call and proving that resulting scalar meets requirements:
    ///
    /// 1. Scalar is canonical
    pub(crate) fn from_raw_unchecked(scalar: E::Scalar) -> Self {
        Self(scalar)
    }
}

impl<E: Curve> AsRef<E::Scalar> for Scalar<E> {
    fn as_ref(&self) -> &E::Scalar {
        &self.0
    }
}
