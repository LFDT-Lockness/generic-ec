#[cfg(feature = "std")]
use std::error::Error;

use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct InvalidCoordinate;

impl fmt::Display for InvalidCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid coordinate")
    }
}

#[cfg(feature = "std")]
impl Error for InvalidCoordinate {}

#[derive(Debug, Clone, Copy)]
pub struct InvalidPoint;

impl fmt::Display for InvalidPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid point")
    }
}

#[cfg(feature = "std")]
impl Error for InvalidPoint {}

#[derive(Debug, Clone, Copy)]
pub struct HashToCurveError(());

impl fmt::Display for HashToCurveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("hash to curve error")
    }
}

#[cfg(feature = "std")]
impl Error for HashToCurveError {}
