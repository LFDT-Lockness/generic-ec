use subtle::CtOption;

use crate::{ec_core::*, errors::InvalidPoint, EncodedPoint, Generator};

use self::definition::Point;

pub mod coords;
pub mod ct;
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
        let point = E::Point::decode(bytes).ok_or(InvalidPoint)?;
        Self::from_raw(point).ok_or(InvalidPoint)
    }

    /// Constructs a point from instance of point from backend library
    ///
    /// Returns `None` if point is not valid.
    pub fn from_raw(point: E::Point) -> Option<Self> {
        Self::ct_from_raw(point).into()
    }

    /// Constructs a point from instance of point from backend library (constant time)
    ///
    /// Returns `None` if point is not valid.
    pub fn ct_from_raw(point: E::Point) -> CtOption<Self> {
        let is_on_curve = point.is_on_curve();
        let is_torsion_free = point.is_torsion_free();
        let is_valid = is_on_curve & is_torsion_free;

        // Correctness: we checked validity of the point. Although invalid point
        // is still given to `from_raw_unchecked`, it's never exposed by CtOption,
        // so no one can obtain "invalid" instance of `Point`.
        CtOption::new(Point::from_raw_unchecked(point), is_valid)
    }

    /// Returns a wrapped instance of point from backend library
    pub fn as_raw(&self) -> &E::Point {
        self.as_ref()
    }
}
