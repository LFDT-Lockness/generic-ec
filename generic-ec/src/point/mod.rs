use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::{
    as_raw::{AsRaw, FromRaw},
    ec_core::*,
    errors::InvalidPoint,
    EncodedPoint, Generator,
};

use self::definition::Point;

pub mod coords;
pub mod definition;

impl<E: Curve> Point<E> {
    pub fn generator() -> Generator<E> {
        Generator::default()
    }

    pub fn zero() -> Self {
        // Correctness:
        // 1. Zero point belongs to curve by definition
        // 2. Zero point is free of any component (including torsion component)
        Self::from_raw_unchecked(E::Point::zero())
    }

    pub fn to_bytes(&self, compressed: bool) -> EncodedPoint<E> {
        if compressed {
            let bytes = self.as_raw().encode();
            EncodedPoint::new_compressed(bytes)
        } else {
            let bytes = self.as_raw().encode();
            EncodedPoint::new_uncompressed(bytes)
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidPoint> {
        E::Point::decode(bytes)
            .and_then(Self::from_raw)
            .ok_or(InvalidPoint)
    }
}

impl<E: Curve> FromRaw for Point<E> {
    fn from_raw(point: E::Point) -> Option<Self> {
        Self::ct_from_raw(point).into()
    }

    fn ct_from_raw(point: E::Point) -> CtOption<Self> {
        let is_on_curve = point.is_on_curve();
        let is_torsion_free = point.is_torsion_free();
        let is_valid = is_on_curve & is_torsion_free;

        // Correctness: we checked validity of the point. Although invalid point
        // is still given to `from_raw_unchecked`, it's never exposed by CtOption,
        // so no one can obtain "invalid" instance of `Point`.
        CtOption::new(Point::from_raw_unchecked(point), is_valid)
    }
}

impl<E: Curve> ConditionallySelectable for Point<E> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        // Correctness: both `a` and `b` have to be valid points by construction
        Point::from_raw_unchecked(<E::Point as ConditionallySelectable>::conditional_select(
            &a.as_raw(),
            &b.as_raw(),
            choice,
        ))
    }
}

impl<E: Curve> ConstantTimeEq for Point<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_raw().ct_eq(other.as_raw())
    }
}

impl<E: Curve> AsRef<Point<E>> for Point<E> {
    fn as_ref(&self) -> &Point<E> {
        self
    }
}
