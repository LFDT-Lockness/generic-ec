use phantom_type::PhantomType;

use crate::{ec_core::*, NonZero, Point};

/// Generator of curve `E`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Generator<E: Curve>(PhantomType<E>);

impl<E: Curve> Generator<E> {
    pub fn to_point(&self) -> Point<E> {
        (*self).into()
    }

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
