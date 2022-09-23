use zeroize::Zeroize;

use crate::{as_raw::AsRaw, ec_core::*};

/// Scalar modulo curve `E` group order
///
/// Scalar is guaranteed to be non-negative integer modulo curve `E` group order.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Scalar<E: Curve>(E::Scalar);

impl<E: Curve> Scalar<E> {
    /// Constructs a scalar without checking whether it's valid
    ///
    /// Caller **must** guarantee validity of the scalar. Caller **must** provide a comment
    /// justifying a call and proving that resulting scalar meets requirements:
    ///
    /// 1. Scalar is canonical
    #[inline]
    pub(crate) fn from_raw_unchecked(scalar: E::Scalar) -> Self {
        Self(scalar)
    }
}

impl<E: Curve> AsRaw for Scalar<E> {
    type Raw = E::Scalar;

    #[inline]
    fn as_raw(&self) -> &E::Scalar {
        &self.0
    }
}

impl<E: Curve> Zeroize for Scalar<E> {
    #[inline]
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}
