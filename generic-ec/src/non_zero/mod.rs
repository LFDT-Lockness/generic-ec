use subtle::{ConstantTimeEq, CtOption};

use crate::{Curve, Point, Scalar};

use self::definition::NonZero;

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
}
