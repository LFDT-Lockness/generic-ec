use generic_array::GenericArray;
use subtle::{Choice, CtOption};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Curve;

pub trait HasAffineX: Curve {
    fn x(point: &Self::Point) -> (Choice, GenericArray<u8, Self::CoordinateSize>);
}

pub trait HasAffineXAndParity: Curve + HasAffineX {
    fn x_and_parity(
        point: &Self::Point,
    ) -> (Choice, Parity, GenericArray<u8, Self::CoordinateSize>);
    fn from_x_and_parity(
        y_parity: Parity,
        x: &GenericArray<u8, Self::CoordinateSize>,
    ) -> CtOption<Self::Point>;
}

pub trait HasAffineY: Curve {
    fn y(point: &Self::Point) -> (Choice, GenericArray<u8, Self::CoordinateSize>);
}

pub trait HasAffineXY: Curve + HasAffineX + HasAffineY {
    fn x_and_y(
        point: &Self::Point,
    ) -> (
        Choice,
        GenericArray<u8, Self::CoordinateSize>,
        GenericArray<u8, Self::CoordinateSize>,
    );
    fn from_x_and_y(
        x: &GenericArray<u8, Self::CoordinateSize>,
        y: &GenericArray<u8, Self::CoordinateSize>,
    ) -> CtOption<Self::Point>;
}

pub trait AlwaysHasAffineY: Curve {
    fn y(point: &Self::Point) -> GenericArray<u8, Self::CoordinateSize>;
}

pub trait AlwaysHasAffineYAndSign: Curve + AlwaysHasAffineY {
    fn y_and_sign(point: &Self::Point) -> (Sign, GenericArray<u8, Self::CoordinateSize>);
    fn from_y_and_sign(
        x_sign: Sign,
        y: &GenericArray<u8, Self::CoordinateSize>,
    ) -> Option<Self::Point>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Sign {
    Negative = 0,
    NonNegative = 1,
}

impl Sign {
    #[inline(always)]
    pub fn is_negative(&self) -> Choice {
        let is_non_negative = *self as u8;
        !Choice::from(is_non_negative)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Parity {
    Odd = 0,
    Even = 1,
}

impl Parity {
    #[inline(always)]
    pub fn is_odd(&self) -> Choice {
        !self.is_even()
    }

    #[inline(always)]
    pub fn is_even(&self) -> Choice {
        Choice::from(*self as u8)
    }
}
