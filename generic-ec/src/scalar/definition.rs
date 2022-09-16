use subtle::CtOption;
use zeroize::Zeroize;

use crate::ec_core::*;

/// Scalar modulo curve `E` group order
///
/// Scalar is guaranteed to be non-negative integer modulo curve `E` group order.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroize)]
pub struct Scalar<E: Curve>(E::Scalar);

impl<E: Curve> Scalar<E> {
    /// Constructs a scalar from instance of scalar from backend library
    ///
    /// Returns `None` if scalar is not valid
    pub fn from_raw(scalar: E::Scalar) -> Option<Self> {
        Self::ct_from_raw(scalar).into()
    }

    /// Constructs a scalar from instance of scalar from backend library (constant time)
    ///
    /// Returns `None` if scalar is not valid
    pub fn ct_from_raw(scalar: E::Scalar) -> CtOption<Self> {
        let is_canonical = scalar.is_canonical();

        // Correctness: we checked validity of scalar. Although invalid scalar
        // is still constructed, it's never exposed by CtOption, so no one can
        // obtain "invalid" instance of scalar.
        CtOption::new(Scalar::from_raw_unchecked(scalar), is_canonical)
    }

    /// Returns a wrapped instance of scalar from backend library
    pub fn as_raw(&self) -> &E::Scalar {
        &self.0
    }

    /// Constructs a scalar without checking whether it's valid
    ///
    /// Caller **must** guarantee validity of the scalar. Caller **must** provide a comment
    /// justifying a call and proving that resulting scalar is valid.
    pub(crate) fn from_raw_unchecked(scalar: E::Scalar) -> Self {
        Self(scalar)
    }
}
