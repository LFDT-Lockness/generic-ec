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
pub struct InvalidScalar;

impl fmt::Display for InvalidScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid scalar")
    }
}

#[cfg(feature = "std")]
impl Error for InvalidScalar {}

#[derive(Debug, Clone, Copy)]
pub struct HashError(pub(crate) HashErrorReason);

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            HashErrorReason::HashFailed => {
                f.write_str("couldn't perform hash to curve/scalar operation")
            }
            HashErrorReason::ProducedValueInvalid => {
                f.write_str("hash to curve/scalar produced invalid point/scalar")
            }
        }
    }
}

#[cfg(feature = "std")]
impl Error for HashError {}

#[derive(Debug, Clone, Copy)]
pub(crate) enum HashErrorReason {
    HashFailed,
    ProducedValueInvalid,
}
