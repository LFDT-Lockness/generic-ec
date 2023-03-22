use core::iter::{Product, Sum};

use subtle::{ConstantTimeEq, CtOption};

use crate::{Curve, Point, Scalar};

use self::definition::NonZero;

pub mod coords;
pub mod definition;

impl<E: Curve> NonZero<Point<E>> {
    /// Constructs non-zero point
    ///
    /// Returns `None` if point is zero
    pub fn from_point(point: Point<E>) -> Option<Self> {
        Self::ct_from_point(point).into()
    }

    /// Constructs non-zero point (constant time)
    ///
    /// Returns `None` if point is zero
    pub fn ct_from_point(point: Point<E>) -> CtOption<Self> {
        let zero = Point::zero();
        let is_non_zero = !point.ct_eq(&zero);

        // Correctness: although we technically construct `NonZero` regardless if
        // it's actually non-zero, `CtOption` never exposes it, so `NonZero` with
        // zero value is not accessible by anyone
        CtOption::new(Self::new_unchecked(point), is_non_zero)
    }
}

impl<E: Curve> NonZero<Scalar<E>> {
    /// Constructs $S = 1$
    pub fn one() -> Self {
        // Correctness: constructed scalar = 1, so it's non-zero
        Self::new_unchecked(Scalar::one())
    }

    /// Constructs non-zero scalar
    ///
    /// Returns `None` if scalar is zero
    pub fn from_scalar(scalar: Scalar<E>) -> Option<Self> {
        Self::ct_from_scalar(scalar).into()
    }

    /// Constructs non-zero scalar (constant time)
    ///
    /// Returns `None` if scalar is zero
    pub fn ct_from_scalar(scalar: Scalar<E>) -> CtOption<Self> {
        let zero = Scalar::zero();
        let is_non_zero = !scalar.ct_eq(&zero);

        // Correctness: although we technically construct `NonZero` regardless if
        // it's actually non-zero, `CtOption` never exposes it, so `NonZero` with
        // zero value is not accessible by anyone
        CtOption::new(Self::new_unchecked(scalar), is_non_zero)
    }

    /// Returns scalar inverse $S^{-1}$
    ///
    /// Similar to [Scalar::invert], but this function is always defined as inverse is defined for all
    /// non-zero scalars
    pub fn invert(&self) -> NonZero<Scalar<E>> {
        #[allow(clippy::expect_used)]
        let inv = (**self)
            .invert()
            .expect("nonzero scalar always has an invert");
        // Correctness: `inv` is nonzero by definition
        Self::new_unchecked(inv)
    }
}

impl<E: Curve> Sum<NonZero<Scalar<E>>> for Scalar<E> {
    fn sum<I: Iterator<Item = NonZero<Scalar<E>>>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + x)
    }
}

impl<'s, E: Curve> Sum<&'s NonZero<Scalar<E>>> for Scalar<E> {
    fn sum<I: Iterator<Item = &'s NonZero<Scalar<E>>>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + x)
    }
}

impl<E: Curve> Product<NonZero<Scalar<E>>> for NonZero<Scalar<E>> {
    fn product<I: Iterator<Item = NonZero<Scalar<E>>>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}

impl<'s, E: Curve> Product<&'s NonZero<Scalar<E>>> for NonZero<Scalar<E>> {
    fn product<I: Iterator<Item = &'s NonZero<Scalar<E>>>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}
