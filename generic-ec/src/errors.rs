//! When something goes wrong

#[cfg(feature = "std")]
use std::error::Error;

use core::fmt;

/// Indicates that provided integer (usually encoded as bytes) can't be a valid coordinate of a point on curve `E`
#[derive(Debug, Clone, Copy)]
pub struct InvalidCoordinate;

impl fmt::Display for InvalidCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid coordinate")
    }
}

#[cfg(feature = "std")]
impl Error for InvalidCoordinate {}

/// Indicates that point is not valid
#[derive(Debug, Clone, Copy)]
pub struct InvalidPoint;

impl fmt::Display for InvalidPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid point")
    }
}

#[cfg(feature = "std")]
impl Error for InvalidPoint {}

/// Indicates that scalar is not valid
#[derive(Debug, Clone, Copy)]
pub struct InvalidScalar;

impl fmt::Display for InvalidScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid scalar")
    }
}

#[cfg(feature = "std")]
impl Error for InvalidScalar {}

/// Appeared zero point is not expected/accepted
#[derive(Debug, Clone, Copy)]
pub struct ZeroPoint;

impl fmt::Display for ZeroPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("zero point")
    }
}

#[cfg(feature = "std")]
impl Error for ZeroPoint {}

/// Appeared zero scalar is not expected/accepted
#[derive(Debug, Clone, Copy)]
pub struct ZeroScalar;

impl fmt::Display for ZeroScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("zero scalar")
    }
}

#[cfg(feature = "std")]
impl Error for ZeroScalar {}
