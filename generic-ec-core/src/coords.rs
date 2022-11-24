#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Curve;

pub trait HasAffineX: Curve {
    fn x(point: &Self::Point) -> Option<Self::CoordinateArray>;
}

pub trait HasAffineXAndParity: Curve + HasAffineX {
    fn x_and_parity(point: &Self::Point) -> Option<(Self::CoordinateArray, Parity)>;
    fn from_x_and_parity(x: &Self::CoordinateArray, y_parity: Parity) -> Option<Self::Point>;
}

pub trait HasAffineY: Curve {
    fn y(point: &Self::Point) -> Option<Self::CoordinateArray>;
}

pub trait HasAffineXY: Curve + HasAffineX + HasAffineY {
    fn x_and_y(point: &Self::Point) -> Option<(Self::CoordinateArray, Self::CoordinateArray)>;
    fn from_x_and_y(x: &Self::CoordinateArray, y: &Self::CoordinateArray) -> Option<Self::Point>;
}

pub trait AlwaysHasAffineY: Curve {
    fn y(point: &Self::Point) -> Self::CoordinateArray;
}

pub trait AlwaysHasAffineYAndSign: Curve + AlwaysHasAffineY {
    fn y_and_sign(point: &Self::Point) -> (Sign, Self::CoordinateArray);
    fn from_y_and_sign(x_sign: Sign, y: &Self::CoordinateArray) -> Option<Self::Point>;
}

/// Sign of coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Sign {
    Negative = 0,
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
    Odd = 0,
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
