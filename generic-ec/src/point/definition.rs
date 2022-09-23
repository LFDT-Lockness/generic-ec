use zeroize::Zeroize;

use crate::{as_raw::AsRaw, ec_core::*};

/// Torsion-free point on elliptic curve `E`
///
/// Any instance of `Point` is guaranteed to be on curve `E` and free of torsion component (when applicable).
///
/// Point implements all necessary arithmetic operations (like points addition, multiplication at scalar, etc.).
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<E: Curve>(E::Point);

impl<E: Curve> Point<E> {
    /// Constructs a point without checking whether it's valid
    ///
    /// Caller **must** guarantee validity of the point. Caller **must** provide a comment
    /// justifying a call and proving that resulting point meets requirements:
    ///
    /// 1. Point is on curve
    /// 2. Point is free of torsion component
    pub(crate) fn from_raw_unchecked(point: E::Point) -> Self {
        Point(point)
    }
}

impl<E: Curve> AsRaw for Point<E> {
    type Raw = E::Point;

    #[inline]
    fn as_raw(&self) -> &E::Point {
        &self.0
    }
}

impl<E: Curve> Zeroize for Point<E> {
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}
