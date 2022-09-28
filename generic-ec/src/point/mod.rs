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
    /// Curve generator
    ///
    /// Curve generator is a regular point defined in curve specs. See [`Generator<E>`](Generator).
    pub fn generator() -> Generator<E> {
        Generator::default()
    }

    /// Returns identity point $\O$ (sometimes called as _point at infinity_)
    ///
    /// Identity point has special properties:
    ///
    /// $$\forall P \in \G: P + \O = P$$
    /// $$\forall s \in \Zq: s \cdot \O = \O$$
    ///
    /// When you validate input from user or message received on wire, you should bear in mind that
    /// any `Point<E>` may be zero. If your algorithm does not accept identity points, you may check
    /// whether point is zero by calling [`.is_zero()`](Point::is_zero). Alternatively, you may accept
    /// [`NonZero<Point<E>>`](crate::NonZero) instead, which is guaranteed to be non zero.
    pub fn zero() -> Self {
        // Correctness:
        // 1. Zero point belongs to curve by definition
        // 2. Zero point is free of any component (including torsion component)
        Self::from_raw_unchecked(E::Point::zero())
    }

    /// Indicates whether it's [identity point](Self::zero)
    ///
    /// ```rust
    /// assert_eq!(Point::zero().is_zero());
    /// assert_eq!(!Point::generator().to_point().is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.ct_is_zero().into()
    }

    /// Indicates whether it's [identity point](Self::zero) (constant time)
    ///
    /// Same as [`.is_zero()`](Self::is_zero) but performs constant-time comparison.
    pub fn ct_is_zero(&self) -> Choice {
        Zero::is_zero(self.as_raw())
    }

    /// Encodes a point as bytes
    ///
    /// Function can return both compressed and uncompressed bytes representation of a point.
    /// Compressed bytes representation is more compact, but parsing takes a little bit more
    /// time. On other hand, uncompressed representation takes ~twice more space, but parsing
    /// is instant.
    ///
    /// For some curves, `compressed` parameter may be ignored, and same bytes representation
    /// is returned.
    ///
    /// ```rust
    /// let random_point = Point::generator() * Scalar::random();
    /// let point_bytes = point.to_bytes(false);
    /// let point_decoded = Point::from_bytes(&point_bytes);
    /// assert_eq!(random_point, point_decoded);
    /// ```
    pub fn to_bytes(&self, compressed: bool) -> EncodedPoint<E> {
        if compressed {
            let bytes = self.as_raw().encode();
            EncodedPoint::new_compressed(bytes)
        } else {
            let bytes = self.as_raw().encode();
            EncodedPoint::new_uncompressed(bytes)
        }
    }

    /// Decodes a point from bytes
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
