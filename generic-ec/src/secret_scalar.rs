use core::fmt;
use core::iter::{Product, Sum};

use rand_core::{CryptoRng, RngCore};
use subtle::{Choice, ConstantTimeEq};

use crate::EncodedSecretScalar;
use crate::{errors::InvalidScalar, secret, Curve, Scalar};

/// Scalar representing sensitive information (like secret key)
///
/// Secret scalar should be treated with an extra care. You shouldn't do any
/// branching (e.g. `Eq`, `Ord`) on the secret to avoid timing side-channel
/// attacks, so it implements only constant time traits (like [`ConstantTimeEq`]).
///
/// Also, when `alloc` feature is enabled, we enforce extra measures:
///
/// * Secret scalar leaves no trace in RAM after it's dropped \
///   Memory is zeroized after use
/// * All clones of secret scalar refer to the same region in the memory \
///   I.e. there will always be only one instance of the scalar in the memory
///   no matter how many clones you make
///
/// All these guarantees can be bypassed by calling `.as_ref()` and obtaining
/// `&Scalar<E>` that is not protected from timing attacks, leaving traces in
/// the memory, etc.
///
/// [`ConstantTimeEq`]: subtle::ConstantTimeEq
pub struct SecretScalar<E: Curve>(secret::Secret<Scalar<E>>);

impl<E: Curve> Scalar<E> {
    /// Convert this value into a [`SecretScalar`]. You should do this at the end
    /// of computations that produce a secret, like a key exchange
    #[inline(always)] // Prevent a byte copy in most cases
    pub fn into_secret(mut self) -> SecretScalar<E> {
        SecretScalar::new(&mut self)
    }
}

impl<E: Curve> SecretScalar<E> {
    /// Constructs a new secret scalar
    ///
    /// Takes the original scalar by mutable reference instead of taking by value to
    /// avoid leaving copies of the scalar on stack. Scalar behind the reference will
    /// be zeroized after the function has returned.
    pub fn new(scalar: &mut Scalar<E>) -> Self {
        Self(secret::new(scalar))
    }

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

    #[doc = include_str!("../../docs/hash_to_scalar.md")]
    ///
    /// ## Example
    /// ```rust
    /// use generic_ec::{SecretScalar, curves::Secp256k1};
    /// use sha2::Sha256;
    ///
    /// #[derive(udigest::Digestable)]
    /// struct Data<'a> {
    ///     nonce: &'a [u8],
    ///     param_a: &'a str,
    ///     param_b: u128,
    ///     // ...
    /// }
    ///
    /// let scalar = SecretScalar::<Secp256k1>::from_hash::<Sha256>(&Data {
    ///     nonce: b"some data",
    ///     param_a: "some other data",
    ///     param_b: 12345,
    ///     // ...
    /// });
    /// ```
    #[cfg(feature = "hash-to-scalar")]
    pub fn from_hash<D: digest::Digest>(data: &impl udigest::Digestable) -> Self {
        let mut rng = rand_hash::HashRng::<D, _>::from_seed(data);
        Self::random(&mut rng)
    }

    /// Obtain the reference to the scalar, which you can use for any operations
    /// necessary. This bypasses all the secrecy guarantees, so be careful when
    /// handling the resulting value; ideally this reference should not be held
    /// for longer than one expression
    ///
    /// The [`AsRef`] impl uses this method under the hood, and is provided as a
    /// less explicit but very convenient alternative
    pub fn as_nonsecret(&self) -> &Scalar<E> {
        secret::inner_ref(&self.0)
    }

    /// Encodes scalar as bytes in big-endian order
    pub fn to_be_bytes(&self) -> EncodedSecretScalar<E> {
        let bytes = self.as_ref().to_be_bytes();
        EncodedSecretScalar::new(bytes)
    }

    /// Encodes scalar as bytes in little-endian order
    pub fn to_le_bytes(&self) -> EncodedSecretScalar<E> {
        let bytes = self.as_ref().to_le_bytes();
        EncodedSecretScalar::new(bytes)
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

impl<E: Curve> AsRef<Scalar<E>> for SecretScalar<E> {
    fn as_ref(&self) -> &Scalar<E> {
        self.as_nonsecret()
    }
}

impl<E: Curve> Clone for SecretScalar<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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

    fn random_vartime<R: rand_core::RngCore>(rng: &mut R) -> Self {
        let mut scalar = Scalar::random_vartime(rng);
        Self::new(&mut scalar)
    }
}
