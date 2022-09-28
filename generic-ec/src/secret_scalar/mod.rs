use rand_core::{CryptoRng, RngCore};
use subtle::{Choice, ConstantTimeEq};

use crate::{encoded::EncodedScalar, errors::InvalidScalar, Curve, Scalar};

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

    /// Decodes scalar from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidScalar> {
        let mut scalar = Scalar::from_bytes(bytes)?;
        Ok(Self::new(&mut scalar))
    }

    /// Encodes scalar as bytes
    pub fn to_bytes(&self) -> EncodedScalar<E> {
        self.as_ref().to_bytes()
    }
}

impl<E: Curve> ConstantTimeEq for SecretScalar<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_ref().ct_eq(other.as_ref())
    }
}
