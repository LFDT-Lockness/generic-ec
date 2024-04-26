//! Exposes information about point coordinates
//!
//! `Curve` by default does not require that points expose their coordinates.
//! Some curves implementations intentionally don't expose points coordinates,
//! and, generally, most of the EC algorithms don't need them. Each curve
//! implementation may optionally expose affine coordinates by implementing
//! trait from this module.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Curve;

/// A point that has affine x coordinate
pub trait HasAffineX: Curve {
    /// Returns point affine x coordinate
    ///
    /// Returns `None` if it's point at infinity
    fn x(point: &Self::Point) -> Option<Self::CoordinateArray>;
}

/// A point that has affine x coordinate and parity of y coordinate
pub trait HasAffineXAndParity: Curve + HasAffineX {
    /// Returns point x coordinate and parity of y coordinate
    ///
    /// Returns `None` if it's point at infinity
    fn x_and_parity(point: &Self::Point) -> Option<(Self::CoordinateArray, Parity)>;
    /// Construct a point from x coordinate and parity of y coordinate
    ///
    /// Returns `None` if input does not correspond to a valid point (but you
    /// still need to check that the point is [on curve](super::OnCurve) and
    /// has [no small component](SmallFactor))
    fn from_x_and_parity(x: &Self::CoordinateArray, y_parity: Parity) -> Option<Self::Point>;
}

/// A point that has affine y coordinate
pub trait HasAffineY: Curve {
    /// Returns point affine y coordinate
    ///
    /// Returns `None` if it's point at infinity
    fn y(point: &Self::Point) -> Option<Self::CoordinateArray>;
}

/// A point that has affine x and y coordinates
pub trait HasAffineXY: Curve + HasAffineX + HasAffineY {
    /// Returns point affine x and y coordinates
    ///
    /// Returns `None` if it's point at infinity
    fn x_and_y(point: &Self::Point) -> Option<(Self::CoordinateArray, Self::CoordinateArray)>;
    /// Construct a point from x and y coordinates
    ///
    /// Returns `None` if input does not correspond to a valid point (but you
    /// still need to check that the point is [on curve](super::OnCurve) and
    /// has [no small component](SmallFactor))
    fn from_x_and_y(x: &Self::CoordinateArray, y: &Self::CoordinateArray) -> Option<Self::Point>;
}

/// A point that always has affine y coordinate
pub trait AlwaysHasAffineY: Curve {
    /// Returns point affine y coordinate
    fn y(point: &Self::Point) -> Self::CoordinateArray;
}

/// A point that always has affine y coordinate and sign of x coordinate
pub trait AlwaysHasAffineYAndSign: Curve + AlwaysHasAffineY {
    /// Returns y coordinate and sign of x coordinate
    fn y_and_sign(point: &Self::Point) -> (Sign, Self::CoordinateArray);
    /// Constructs a point from y coordinate and sign of x coordinate
    ///
    /// Returns `None` if input does not correspond to a valid point (but you
    /// still need to check that the point is [on curve](super::OnCurve) and
    /// has [no small component](SmallFactor))
    fn from_y_and_sign(x_sign: Sign, y: &Self::CoordinateArray) -> Option<Self::Point>;
}

/// Sign of coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Sign {
    /// Coordinate has negative sign
    Negative = 0,
    /// Coordinate has non-negative sign
    NonNegative = 1,
}

impl Sign {
    /// Checks whether coordinate is negative
    pub fn is_negative(&self) -> bool {
        *self == Self::Negative
    }
}

/// Parity of coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Parity {
    /// Coordinate is odd
    Odd = 0,
    /// Coordinate is even
    Even = 1,
}

impl Parity {
    /// Checks whether coordinate is odd
    pub fn is_odd(&self) -> bool {
        *self == Self::Odd
    }

    /// Checks whether coordinate is even
    pub fn is_even(&self) -> bool {
        *self == Self::Even
    }
}
