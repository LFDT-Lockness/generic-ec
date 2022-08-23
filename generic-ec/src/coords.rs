//! # Elliptic points coordinates
//!
//! Elliptic points are defined differently for different types of curves:
//! * Curves in non-complete form (Weierstrass or Montgomery curves): \
//!   Points have $(x, y)$ coordinates that must satisfy curve equation unless it's **point at infinity**
//!   that has no coordinates (see [points at infinity](crate::TODO))
//! * Curves in complete form (Edwards curves): \
//!   Points always have $(x, y)$ coordinates that must satisfy curve equation
//!
//! ## Usage
//! This module provides various traits that can be used to retrieve coordinates. Refer to curve documenation
//! to see what coordinates it exposes.
//!
//! ```rust
//! use generic_ec::{Point, coords::HasAffineX, curves::Secp256k1};
//!
//! let point = Point::<Secp256k1>::generator();
//! let x = point.x();
//! ```
//!
//! ### In generic code
//! Generic code needs to explicitly state that it needs access to coordinates by specifying it in bounds:
//! ```rust
//! use generic_ec::{Point, Curve, coords::HasAffineX};
//!
//! fn func_that_accesses_x_coord<E: Curve>(point: &Point<E>)
//! where
//!     Point<E>: HasAffineX<E>
//! {
//!     let x = point.x();
//!     // ...
//! }
//! ```
//!
//! _Note:_ it's not recommended to access points coordinates in generic code unless it's really necessary.
//! Practically it lessens variety of curves that can work with your code. If you need unique representation
//! of a point, use [its byte representation](crate::Point::to_bytes).
//!
//! ## Curves support
//! Some curve implementations intentionally chosen not to expose coordinates, so they, for instance, can
//! expose $y$ coordinate but hide $x$. [Ed25519] is such curve (backed by [curve25519_dalek]) that doesn't
//! allow you to access $x$ coordinate, though you can access $y$ coordinate and sign of $x$ coordinate
//! through [`AlwaysHasAffineYAndSign`] which uniquely represents a point.
//!
//! [Ed25519]: crate::curves::Ed25519
//! [curve25519_dalek]: https://github.com/dalek-cryptography/curve25519-dalek

use generic_array::GenericArray;
use subtle::CtOption;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[doc(inline)]
pub use crate::ec_core::coords::{Parity, Sign};
use crate::{ec_core::Curve, errors::InvalidCoordinate};

/// Affine $x, y$ coordinates of a point on elliptic curve
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(bound = ""))]
pub struct Coordinates<E: Curve> {
    pub x: Coordinate<E>,
    pub y: Coordinate<E>,
}

/// Affine coordinate of a point on elliptic curve
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinate<E: Curve>(CoordinateBytes<E>);

/// Bytes array that can fit a serialized coordinate
pub type CoordinateBytes<E> = GenericArray<u8, <E as Curve>::CoordinateSize>;

impl<E: Curve> Coordinate<E> {
    /// Bytes representation of a coordinate
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Parses bytes representation of a coordinate
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidCoordinate> {
        let mut coord = Self::default();
        if coord.as_bytes().len() != bytes.len() {
            return Err(InvalidCoordinate);
        }
        coord.as_mut().copy_from_slice(bytes);
        Ok(coord)
    }

    /// Constructs a coordinate from a byte array
    pub fn new(bytes: CoordinateBytes<E>) -> Self {
        Self(bytes)
    }

    /// Bytes representation of a coordinate
    pub fn as_array(&self) -> &CoordinateBytes<E> {
        &self.0
    }
}

impl<E: Curve> AsRef<[u8]> for Coordinate<E> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<E: Curve> AsMut<[u8]> for Coordinate<E> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<E: Curve> Default for Coordinate<E> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(feature = "serde")]
impl<E: Curve> Serialize for Coordinate<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use crate::serde_utils::Bytes;
        use serde_with::SerializeAs;

        <Bytes as SerializeAs<Coordinate<E>>>::serialize_as(self, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, E: Curve> Deserialize<'de> for Coordinate<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use crate::serde_utils::{expectation, Bytes};
        use serde_with::DeserializeAs;

        <Bytes<expectation::Coordinate> as DeserializeAs<'de, Coordinate<E>>>::deserialize_as(
            deserializer,
        )
    }
}

mod sealed {
    pub trait Sealed {}

    impl<E: crate::ec_core::Curve> Sealed for crate::wrappers::Point<E> {}
}

/// Point has affine $x$ coordinate
pub trait HasAffineX<E: Curve>: sealed::Sealed {
    /// Retrieves affine $x$ coordinate
    ///
    /// Returns `None` if it's `Point::zero()`
    fn x(&self) -> CtOption<Coordinate<E>>;
}

/// Point has affine $y$ coordinate
pub trait HasAffineY<E: Curve>: sealed::Sealed {
    /// Retrieves affine $y$ coordinate
    ///
    /// Returns `None` if it's `Point::zero()`
    fn y(&self) -> CtOption<Coordinate<E>>;
}

/// Point is uniquely represented by $x$ coordinate and parity of $y$ coordinate
pub trait HasAffineXAndParity<E: Curve>: HasAffineX<E>
where
    Self: Sized,
{
    /// Retrieves affine $x$ coordinate and parity of $y$ coordinate
    ///
    /// Returns `None` if it's `Point::zero()`
    fn x_and_parity(&self) -> CtOption<(Parity, Coordinate<E>)>;
    /// Constructs point from its $x$ coordinate and parity of $y$ coordinate
    ///
    /// Returns `None` if arguments do not represent a valid `Point<E>`
    fn from_x_and_parity(x: Coordinate<E>, y_parity: Parity) -> CtOption<Self>;
}

/// Point is uniquely represented by affine $x, y$ coordinates
pub trait HasAffineXY<E: Curve>: HasAffineX<E> + HasAffineY<E>
where
    Self: Sized,
{
    /// Retrieves affine $x, y$ coordinates
    ///
    /// Returns `None` if it's `Point::zero()`
    fn coords(&self) -> CtOption<Coordinates<E>>;
    /// Constructs point from its $x, y$ coordinates
    ///
    /// Returns `None` if coordinates do not represent a valid `Point<E>`
    fn from_coords(coords: &Coordinates<E>) -> CtOption<Self>;
}

/// Point _always_ has affine $y$ coordinate (for Edwards curves)
pub trait AlwaysHasAffineY<E: Curve>: sealed::Sealed {
    /// Retrieves affine $y$ coordinate
    fn y(&self) -> Coordinate<E>;
}

/// Point is uniquely represented by affine $y$ coordinate and sign of $x$ coordinate (for Edwards curves)
pub trait AlwaysHasAffineYAndSign<E: Curve>: AlwaysHasAffineY<E>
where
    Self: Sized,
{
    /// Retrieves affine $y$ coordinate and sign of $x$ coordinate
    fn y_and_sign(&self) -> (Sign, Coordinate<E>);
    /// Constructs point from its $y$ coordinate and sign of $x$ coordinate
    ///
    /// Returns `None` if input arguments do not represent a valid `Point<E>`
    fn from_y_and_sign(x_sign: Sign, y: &Coordinate<E>) -> Option<Self>;
}
