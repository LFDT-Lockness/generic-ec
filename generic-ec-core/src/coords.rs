use subtle::{Choice, CtOption};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Curve;

pub trait HasAffineX: Curve {
    fn x(point: &Self::Point) -> (Choice, Self::CoordinateArray);
}

pub trait HasAffineXAndParity: Curve + HasAffineX {
    fn x_and_parity(point: &Self::Point) -> (Choice, Parity, Self::CoordinateArray);
    fn from_x_and_parity(x: &Self::CoordinateArray, y_parity: Parity) -> CtOption<Self::Point>;
}

pub trait HasAffineY: Curve {
    fn y(point: &Self::Point) -> (Choice, Self::CoordinateArray);
}

pub trait HasAffineXY: Curve + HasAffineX + HasAffineY {
    fn x_and_y(point: &Self::Point) -> (Choice, Self::CoordinateArray, Self::CoordinateArray);
    fn from_x_and_y(x: &Self::CoordinateArray, y: &Self::CoordinateArray) -> CtOption<Self::Point>;
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
    /// Checks whether coordinate is negative (constant time)
    #[inline(always)]
    pub fn ct_is_negative(&self) -> Choice {
        let is_non_negative = *self as u8;
        !Choice::from(is_non_negative)
    }

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
    /// Checks whether coordinate is odd (constant time)
    #[inline(always)]
    pub fn ct_is_odd(&self) -> Choice {
        !self.ct_is_even()
    }

    /// Checks whether coordinate is even (constant time)
    #[inline(always)]
    pub fn ct_is_even(&self) -> Choice {
        Choice::from(*self as u8)
    }

    /// Checks whether coordinate is odd
    pub fn is_odd(&self) -> bool {
        *self == Self::Odd
    }

    /// Checks whether coordinate is even
    pub fn is_even(&self) -> bool {
        *self == Self::Even
    }
}
