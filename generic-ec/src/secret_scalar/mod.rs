use core::fmt;
use core::iter::{Product, Sum};

use rand_core::{CryptoRng, RngCore};
use subtle::{Choice, ConstantTimeEq};

use crate::{errors::InvalidScalar, Curve, Scalar};

use self::definition::SecretScalar;

pub mod definition;

impl<E: Curve> SecretScalar<E> {
    /// Returns scalar $S = 0$
    pub fn zero() -> Self {
        Self::new(&mut Scalar::zero())
    }

    /// Returns scalar $S = 1$
    pub fn one() -> Self {
        Self::new(&mut Scalar::one())
    }

    /// Returns scalar inverse
    pub fn invert(&self) -> Option<Self> {
        let scalar: Option<Scalar<E>> = self.as_ref().ct_invert().into();
        Some(Self::new(&mut scalar?))
    }

    /// Generates random secret scalar
    pub fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        let mut scalar = Scalar::random(rng);
        Self::new(&mut scalar)
    }

    /// Decodes scalar from its bytes representation in big-endian order
    pub fn from_be_bytes(bytes: &[u8]) -> Result<Self, InvalidScalar> {
        let mut scalar = Scalar::from_be_bytes(bytes)?;
        Ok(Self::new(&mut scalar))
    }

    /// Decodes scalar from its bytes representation in little-endian order
    pub fn from_le_bytes(bytes: &[u8]) -> Result<Self, InvalidScalar> {
        let mut scalar = Scalar::from_le_bytes(bytes)?;
        Ok(Self::new(&mut scalar))
    }
}

impl<E: Curve> ConstantTimeEq for SecretScalar<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_ref().ct_eq(other.as_ref())
    }
}

impl<E: Curve> Sum<SecretScalar<E>> for Scalar<E> {
    fn sum<I: Iterator<Item = SecretScalar<E>>>(iter: I) -> Self {
        iter.fold(Scalar::<E>::zero(), |acc, i| acc + &i)
    }
}

impl<'s, E: Curve> Sum<&'s SecretScalar<E>> for Scalar<E> {
    fn sum<I: Iterator<Item = &'s SecretScalar<E>>>(iter: I) -> Self {
        iter.fold(Scalar::<E>::zero(), |acc, i| acc + i)
    }
}

impl<E: Curve> Product<SecretScalar<E>> for Scalar<E> {
    fn product<I: Iterator<Item = SecretScalar<E>>>(iter: I) -> Self {
        iter.fold(Scalar::<E>::one(), |acc, i| acc * &i)
    }
}

impl<'s, E: Curve> Product<&'s SecretScalar<E>> for Scalar<E> {
    fn product<I: Iterator<Item = &'s SecretScalar<E>>>(iter: I) -> Self {
        iter.fold(Scalar::<E>::one(), |acc, i| acc * i)
    }
}

impl<E: Curve> fmt::Debug for SecretScalar<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretScalar")
    }
}

impl<E: Curve> crate::traits::Samplable for SecretScalar<E> {
    fn random<R: RngCore>(rng: &mut R) -> Self {
        let mut scalar = Scalar::random(rng);
        Self::new(&mut scalar)
    }
}
