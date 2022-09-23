use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::{
    as_raw::{AsRaw, FromRaw},
    ec_core::*,
    encoded::EncodedScalar,
    errors::InvalidScalar,
};

use self::definition::Scalar;

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

    pub fn to_bytes(&self) -> EncodedScalar<E> {
        let bytes = self.as_raw().encode();
        EncodedScalar::new(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidScalar> {
        E::Scalar::decode(bytes)
            .and_then(Self::from_raw)
            .ok_or(InvalidScalar)
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
}

impl<E: Curve> FromRaw for Scalar<E> {
    fn from_raw(scalar: E::Scalar) -> Option<Self> {
        Self::ct_from_raw(scalar).into()
    }

    fn ct_from_raw(scalar: E::Scalar) -> CtOption<Self> {
        let is_canonical = scalar.is_canonical();

        // Correctness: we checked validity of scalar. Although invalid scalar
        // is still constructed, it's never exposed by CtOption, so no one can
        // obtain "invalid" instance of scalar.
        CtOption::new(Scalar::from_raw_unchecked(scalar), is_canonical)
    }
}

impl<E: Curve> ConditionallySelectable for Scalar<E> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        // Correctness: both `a` and `b` have to be valid points by construction
        Scalar::from_raw_unchecked(<E::Scalar as ConditionallySelectable>::conditional_select(
            &a.as_raw(),
            &b.as_raw(),
            choice,
        ))
    }
}

impl<E: Curve> ConstantTimeEq for Scalar<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_raw().ct_eq(other.as_raw())
    }
}

impl<E: Curve> AsRef<Scalar<E>> for Scalar<E> {
    fn as_ref(&self) -> &Scalar<E> {
        self
    }
}
