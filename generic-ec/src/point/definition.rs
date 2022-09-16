use subtle::CtOption;
use zeroize::Zeroize;

use crate::ec_core::*;

/// Torison-free point on elliptic curve `E`
///
/// Any instance of `Point` is guaranteed to be on curve `E` and free of torison component (when applicable).
///
/// Point implements all necessary arithmetic operations (like points addition, multiplication at scalar, etc.).
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroize)]
pub struct Point<E: Curve>(E::Point);

impl<E: Curve> Point<E> {
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
        let is_torison_free = point.is_torsion_free();
        let is_valid = is_on_curve & is_torison_free;

        // Correctness: we checked validity of the point. Although invalid point
        // is still given to `from_raw_unchecked`, it's never exposed by CtOption,
        // so no one can obtain "invalid" instance of `Point`.
        CtOption::new(Point::from_raw_unchecked(point), is_valid)
    }

    /// Returns a wrapped instance of point from backend library
    pub fn as_raw(&self) -> &E::Point {
        &self.0
    }

    /// Constructs a point without checking whether it's valid
    ///
    /// Caller **must** guarantee validity of the point. Caller **must** provide a comment
    /// justifying a call and proving that resulting point is valid.
    pub(crate) fn from_raw_unchecked(point: E::Point) -> Self {
        Point(point)
    }
}
