use subtle::CtOption;

use crate::ec_core::*;

use self::definition::Scalar;

mod ct;
pub mod definition;

impl<E: Curve> Scalar<E> {
    pub fn zero() -> Self {
        // Correctness: zero is always less than group order
        Self::from_raw_unchecked(E::Scalar::zero())
    }

    pub fn one() -> Self {
        // Correctness: one is always less than group order
        Self::from_raw_unchecked(E::Scalar::one())
    }

    pub fn invert(&self) -> Option<Self> {
        self.ct_invert().into()
    }

    pub fn ct_invert(&self) -> CtOption<Self> {
        let inv = Invertible::invert(self.as_raw());
        // Correctness: inv is reduced
        inv.map(|inv| Self::from_raw_unchecked(inv.reduce()))
    }

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
        self.as_ref()
    }
}
