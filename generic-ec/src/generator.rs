use phantom_type::PhantomType;

use crate::{ec_core::*, NonZero, Point};

/// Generator of curve `E`
///
/// Curve generator is a point on curve defined in curve specs. For some curves,
/// generator multiplication may be optimized, so `Scalar<E> * Generator<E>`
/// would more efficient than `Scalar<E> * Point<E>`. That's the only purpose
/// of `Generator<E>` structure: to distinguish generator multiplication and
/// potentially use more efficient algorithm.
///
/// Curve generator `Generator<E>` should be obtained by calling [`Point::generator()`].
/// You may convert `Generator<E>` to `Point<E>` or `NonZero<Point<E>>` by calling
/// [`.to_point()`] or [`.to_nonzero_point()`], but then multiplication may be less
/// efficient.
///
/// [`Point::generator()`]: Point::generator
/// [`.to_point()`]: Generator::to_point
/// [`.to_nonzero_point()`]: Generator::to_nonzero_point
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Generator<E: Curve>(PhantomType<E>);

impl<E: Curve> Generator<E> {
    /// Returns a point corresponding to curve generator
    pub fn to_point(&self) -> Point<E> {
        (*self).into()
    }

    /// Returns a non-zero point corresponding to curve generator
    pub fn to_nonzero_point(&self) -> NonZero<Point<E>> {
        (*self).into()
    }
}

impl<E: Curve> From<Generator<E>> for Point<E> {
    #[inline]
    fn from(_: Generator<E>) -> Self {
        // Correctness: generator has to be a point on curve free of torsion component
        Point::from_raw_unchecked(E::Point::from(CurveGenerator))
    }
}

impl<E: Curve> From<Generator<E>> for NonZero<Point<E>> {
    #[inline]
    fn from(g: Generator<E>) -> Self {
        // Correctness: generator has to be non-zero point
        NonZero::new_unchecked(g.into())
    }
}

impl<E: Curve> Default for Generator<E> {
    fn default() -> Self {
        Self(PhantomType::new())
    }
}
