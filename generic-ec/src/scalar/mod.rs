use core::iter;

use rand_core::RngCore;
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
    /// Returns scalar $S = 0$
    ///
    /// ```rust
    /// let s = Scalar::random();
    /// assert_eq!(s * Scalar::zero(), Scalar::zero());
    /// assert_eq!(s + Scalar::zero(), s);
    /// ```
    pub fn zero() -> Self {
        // Correctness: zero is always less than group order
        Self::from_raw_unchecked(E::Scalar::zero())
    }

    /// Returns scalar $S = 1$
    ///
    /// ```rust
    /// let s = Scalar::random();
    /// assert_eq!(s * Scalar::one(), s);
    /// ```
    pub fn one() -> Self {
        // Correctness: one is always less than group order
        Self::from_raw_unchecked(E::Scalar::one())
    }

    /// Returns scalar inverse $S^-1$
    ///
    /// Inverse of scalar $S$ is a scalar $S^-1$ such as $S S^-1 = 1$. Inverse doesn't
    /// exist only for scalar $S = 0$, so function returns `None` if scalar is zero.
    ///
    /// ```rust
    /// let s = Scalar::random();
    /// let s_inv = s.invert()?;
    /// assert_eq!(s * s_inv, Scalar::one());
    /// ```
    pub fn invert(&self) -> Option<Self> {
        self.ct_invert().into()
    }

    /// Returns scalar inverse $S^-1$ (constant time)
    ///
    /// Same as [`Scalar::invert`] but performs constant-time check on whether it's zero
    /// scalar
    pub fn ct_invert(&self) -> CtOption<Self> {
        let inv = Invertible::invert(self.as_raw());
        // Correctness: inv is reduced
        inv.map(|inv| Self::from_raw_unchecked(inv.reduce()))
    }

    /// Encodes scalar as bytes
    ///
    /// ```rust
    /// let s = Scalar::random();
    /// let bytes = s.to_bytes();
    /// println!("Scalar hex representation: {}", hex::encode(bytes));
    /// ```
    pub fn to_bytes(&self) -> EncodedScalar<E> {
        let bytes = self.as_raw().encode();
        EncodedScalar::new(bytes)
    }

    /// Decodes scalar from bytes
    ///
    /// ```rust
    /// let s = Scalar::random();
    /// let s_bytes = s.to_bytes();
    /// let s_decoded = Scalar::from_bytes(&bytes);
    /// assert_eq!(s, s_decoded);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidScalar> {
        E::Scalar::decode(bytes)
            .and_then(Self::from_raw)
            .ok_or(InvalidScalar)
    }

    /// Generates random non-zero scalar
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        match iter::repeat_with(|| E::Scalar::random(rng).reduce())
            .take(100)
            .find(|s| bool::from(!Zero::is_zero(s)))
        {
            Some(s) => {
                // Correctness: `s` is reduced
                Scalar::from_raw_unchecked(s)
            }
            None => panic!("defected source of randomness"),
        }
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

impl<E: Curve> iter::Sum for Scalar<E> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + x)
    }
}

impl<'a, E: Curve> iter::Sum<&'a Scalar<E>> for Scalar<E> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + x)
    }
}

impl<E: Curve> iter::Product for Scalar<E> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Scalar::one(), |acc, x| acc * x)
    }
}

impl<'a, E: Curve> iter::Product<&'a Scalar<E>> for Scalar<E> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Scalar::one(), |acc, x| acc * x)
    }
}
