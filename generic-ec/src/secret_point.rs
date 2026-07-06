use core::fmt;
use core::iter::Sum;

use crate::{errors::InvalidPoint, secret, Curve, EncodedSecretPoint, Point};
use subtle::{Choice, ConstantTimeEq};

/// Point representing sensitive information (like a derived secret)
///
/// Secret point should be treated with an extra care. You shouldn't do any
/// branching (e.g. `Eq`, `Ord`) on the secret to avoid timing side-channel
/// attacks, so it implements only constant time traits (like [`ConstantTimeEq`]).
///
/// Also, when `alloc` feature is enabled, we enforce extra measures:
///
/// * Secret point leaves no trace in RAM after it's dropped \
///   Memory is zeroized after use
/// * All clones of a secret point refer to the same region in the memory \
///   I.e. there will always be only one instance of the point in the memory
///   no matter how many clones you make
///
/// All these guarantees can be bypassed by calling `.as_ref()` and obtaining
/// `&Point<E>` that is not protected from timing attacks, leaving traces in
/// the memory, etc.
///
/// [`ConstantTimeEq`]: subtle::ConstantTimeEq
pub struct SecretPoint<E: Curve>(secret::Secret<Point<E>>);

impl<E: Curve> Point<E> {
    /// Convert this value into a [`SecretPoint`]. You should do this at the end
    /// of computations that produce a secret, like a key exchange
    #[inline(always)] // Prevent a byte copy in most cases
    pub fn into_secret(mut self) -> SecretPoint<E> {
        SecretPoint::new(&mut self)
    }
}

impl<E: Curve> SecretPoint<E> {
    /// Constructs a new secret point
    ///
    /// Takes the original point by mutable reference instead of taking by value
    /// to avoid leaving copies of the point on stack. Point behind the
    /// reference will be zeroized after the function has returned.
    pub fn new(point: &mut Point<E>) -> Self {
        Self(secret::new(point))
    }

    /// Returns the generator defined in the curve specs
    pub fn generator() -> Self {
        Self::new(&mut Point::generator().into())
    }

    /// Returns identity point $\O$
    pub fn zero() -> Self {
        Self::new(&mut Point::zero())
    }

    /// Obtain the reference to the point, which you can use for any operations
    /// necessary. This bypasses all the secrecy guarantees, so be careful when
    /// handling the resulting value; ideally this reference should not be held
    /// for longer than one expression
    ///
    /// The [`AsRef`] impl uses this method under the hood, and is provided as a
    /// less explicit but very convenient alternative
    pub fn as_nonsecret(&self) -> &Point<E> {
        secret::inner_ref(&self.0)
    }

    /// Encodes a point as bytes
    pub fn to_bytes(&self, compressed: bool) -> EncodedSecretPoint<E> {
        let bytes = self.as_ref().to_bytes(compressed);
        EncodedSecretPoint::new(bytes)
    }

    /// Decodes a point from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidPoint> {
        let mut point = Point::from_bytes(bytes)?;
        Ok(Self::new(&mut point))
    }
}

impl<E: Curve> AsRef<Point<E>> for SecretPoint<E> {
    fn as_ref(&self) -> &Point<E> {
        self.as_nonsecret()
    }
}

impl<E: Curve> Clone for SecretPoint<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
