use core::fmt;
use core::iter::Sum;

use crate::{errors::InvalidPoint, Curve, Point};
use subtle::{Choice, ConstantTimeEq};

use self::definition::SecretPoint;

pub mod definition;

impl<E: Curve> Point<E> {
    /// Convert this value into a [`SecretPoint`]. You should do this at the end
    /// of computations that produce a secret, like a key exchange
    #[inline(always)] // Prevent a byte copy in most cases
    pub fn into_secret(mut self) -> SecretPoint<E> {
        SecretPoint::new(&mut self)
    }
}

impl<E: Curve> SecretPoint<E> {
    /// Returns the generator defined in the curve specs
    pub fn generator() -> Self {
        Self::new(&mut Point::generator().into())
    }

    /// Returns identity point $\O$
    pub fn zero() -> Self {
        Self::new(&mut Point::zero())
    }

    /// Encodes a point as bytes
    #[cfg(feature = "alloc")]
    pub fn to_bytes(
        &self,
        compressed: bool,
    ) -> alloc::boxed::Box<zeroize::Zeroizing<crate::EncodedPoint<E>>> {
        let bytes = zeroize::Zeroizing::new(self.as_ref().to_bytes(compressed));
        alloc::boxed::Box::new(bytes)
    }

    /// Decodes a point from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidPoint> {
        let mut point = Point::from_bytes(bytes)?;
        Ok(Self::new(&mut point))
    }
}

impl<E: Curve> ConstantTimeEq for SecretPoint<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_ref().ct_eq(other.as_ref())
    }
}

impl<E: Curve> Sum<SecretPoint<E>> for Point<E> {
    fn sum<I: Iterator<Item = SecretPoint<E>>>(iter: I) -> Self {
        iter.fold(Point::<E>::zero(), |acc, i| acc + &i)
    }
}

impl<'s, E: Curve> Sum<&'s SecretPoint<E>> for Point<E> {
    fn sum<I: Iterator<Item = &'s SecretPoint<E>>>(iter: I) -> Self {
        iter.fold(Point::<E>::zero(), |acc, i| acc + i)
    }
}

impl<E: Curve> fmt::Debug for SecretPoint<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretPoint")
    }
}
