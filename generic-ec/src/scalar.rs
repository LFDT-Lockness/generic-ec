use core::hash::{self, Hash};
use core::{fmt, iter};

use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::Zeroize;

use crate::NonZero;
use crate::{
    as_raw::{AsRaw, FromRaw},
    core::*,
    encoded::EncodedScalar,
    errors::InvalidScalar,
};

/// Scalar modulo curve `E` group order
///
/// Scalar is an integer modulo curve `E` group order.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Scalar<E: Curve>(E::Scalar);

impl<E: Curve> Scalar<E> {
    /// Returns scalar $S = 0$
    ///
    /// ```rust
    /// use generic_ec::{Scalar, curves::Secp256k1};
    /// use rand::rngs::OsRng;
    ///
    /// let s = Scalar::<Secp256k1>::random(&mut OsRng);
    /// assert_eq!(s * Scalar::zero(), Scalar::zero());
    /// assert_eq!(s + Scalar::zero(), s);
    /// ```
    pub fn zero() -> Self {
        Self::from_raw(E::Scalar::zero())
    }

    /// Returns scalar $S = 1$
    ///
    /// ```rust
    /// use generic_ec::{Scalar, curves::Secp256k1};
    /// use rand::rngs::OsRng;
    ///
    /// let s = Scalar::<Secp256k1>::random(&mut OsRng);
    /// assert_eq!(s * Scalar::one(), s);
    /// ```
    pub fn one() -> Self {
        Self::from_raw(E::Scalar::one())
    }

    /// Returns scalar inverse $S^{-1}$
    ///
    /// Inverse of scalar $S$ is a scalar $S^{-1}$ such as $S \cdot S^{-1} = 1$. Inverse doesn't
    /// exist only for scalar $S = 0$, so function returns `None` if scalar is zero.
    ///
    /// ```rust
    /// # fn func() -> Option<()> {
    /// use generic_ec::{Scalar, curves::Secp256k1};
    /// use rand::rngs::OsRng;
    ///
    /// let s = Scalar::<Secp256k1>::random(&mut OsRng);
    /// let s_inv = s.invert()?;
    /// assert_eq!(s * s_inv, Scalar::one());
    /// # Some(()) }
    /// # func();
    /// ```
    pub fn invert(&self) -> Option<Self> {
        self.ct_invert().into()
    }

    /// Returns scalar inverse $S^{-1}$ (constant time)
    ///
    /// Same as [`Scalar::invert`] but performs constant-time check on whether it's zero
    /// scalar
    pub fn ct_invert(&self) -> CtOption<Self> {
        let inv = Invertible::invert(self.as_raw());
        inv.map(Self::from_raw)
    }

    /// Encodes scalar as bytes in big-endian order
    ///
    /// ```rust
    /// use generic_ec::{Scalar, curves::Secp256k1};
    /// use rand::rngs::OsRng;
    ///
    /// let s = Scalar::<Secp256k1>::random(&mut OsRng);
    /// let bytes = s.to_be_bytes();
    /// println!("Scalar hex representation: {}", hex::encode(bytes));
    /// ```
    pub fn to_be_bytes(&self) -> EncodedScalar<E> {
        let bytes = self.as_raw().to_be_bytes();
        EncodedScalar::new(bytes)
    }

    /// Encodes scalar as bytes in little-endian order
    pub fn to_le_bytes(&self) -> EncodedScalar<E> {
        let bytes = self.as_raw().to_le_bytes();
        EncodedScalar::new(bytes)
    }

    /// Decodes scalar from its representation as bytes in big-endian order
    ///
    /// Returns error if encoded integer is larger than group order.
    ///
    /// ```rust
    /// use generic_ec::{Scalar, curves::Secp256k1};
    /// use rand::rngs::OsRng;
    ///
    /// let s = Scalar::<Secp256k1>::random(&mut OsRng);
    /// let s_bytes = s.to_be_bytes();
    /// let s_decoded = Scalar::from_be_bytes(&s_bytes)?;
    /// assert_eq!(s, s_decoded);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_be_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, InvalidScalar> {
        let bytes = bytes.as_ref();
        let mut bytes_array = E::ScalarArray::zeroes();
        let bytes_array_len = bytes_array.as_ref().len();
        if bytes_array_len < bytes.len() {
            return Err(InvalidScalar);
        }
        bytes_array.as_mut()[bytes_array_len - bytes.len()..].copy_from_slice(bytes);

        let scalar = E::Scalar::from_be_bytes_exact(&bytes_array).ok_or(InvalidScalar)?;
        Ok(Scalar::from_raw(scalar))
    }

    /// Decodes scalar from its representation as bytes in little-endian order
    ///
    /// Returns error if encoded integer is larger than group order.
    pub fn from_le_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, InvalidScalar> {
        let bytes = bytes.as_ref();
        let mut bytes_array = E::ScalarArray::zeroes();
        let bytes_array_len = bytes_array.as_ref().len();
        if bytes_array_len < bytes.len() {
            return Err(InvalidScalar);
        }
        bytes_array.as_mut()[..bytes.len()].copy_from_slice(bytes);

        let scalar = E::Scalar::from_le_bytes_exact(&bytes_array).ok_or(InvalidScalar)?;
        Ok(Scalar::from_raw(scalar))
    }

    /// Interprets provided bytes as integer $i$ in big-endian order, returns scalar $s = i \mod q$
    pub fn from_be_bytes_mod_order(bytes: impl AsRef<[u8]>) -> Self {
        let scalar_0x100 = Scalar::from(0x100_u16);
        bytes
            .as_ref()
            .iter()
            .fold(Scalar::<E>::zero(), |acc, byte| {
                acc * scalar_0x100 + Scalar::from(*byte)
            })
    }

    /// Interprets provided bytes as integer $i$ in little-endian order, returns scalar $s = i \mod q$
    pub fn from_le_bytes_mod_order(bytes: impl AsRef<[u8]>) -> Self {
        let scalar_0x100 = Scalar::from(0x100_u16);
        bytes
            .as_ref()
            .iter()
            .rev()
            .fold(Scalar::<E>::zero(), |acc, byte| {
                acc * scalar_0x100 + Scalar::from(*byte)
            })
    }

    /// Generates random non-zero scalar
    ///
    /// Algorithm is based on rejection sampling: we sample a scalar, if it's zero try again.
    /// It may be considered constant-time as zero scalar appears with $2^{-256}$ probability
    /// which is considered to be negligible.
    ///
    /// ## Panics
    /// Panics if randomness source returned 100 zero scalars in a row. It happens with
    /// $2^{-25600}$ probability, which practically means that randomness source is broken.
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        NonZero::<Scalar<E>>::random(rng).into()
    }

    /// Returns size of bytes buffer that can fit serialized scalar
    pub fn serialized_len() -> usize {
        E::ScalarArray::zeroes().as_ref().len()
    }
}

impl<E: Curve> AsRaw for Scalar<E> {
    type Raw = E::Scalar;

    #[inline]
    fn as_raw(&self) -> &E::Scalar {
        &self.0
    }
}

impl<E: Curve> Zeroize for Scalar<E> {
    #[inline]
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}

impl<E: Curve> FromRaw for Scalar<E> {
    fn from_raw(scalar: E::Scalar) -> Self {
        Self(scalar)
    }
}

impl<E: Curve> ConditionallySelectable for Scalar<E> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Scalar::from_raw(<E::Scalar as ConditionallySelectable>::conditional_select(
            a.as_raw(),
            b.as_raw(),
            choice,
        ))
    }
}

impl<E: Curve> ConstantTimeEq for Scalar<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_raw().ct_eq(other.as_raw())
    }
}

impl<E: Curve> AsRef<Scalar<E>> for Scalar<E> {
    fn as_ref(&self) -> &Scalar<E> {
        self
    }
}

impl<E: Curve> crate::traits::IsZero for Scalar<E> {
    fn is_zero(&self) -> bool {
        *self == Scalar::zero()
    }
}

impl<E: Curve> crate::traits::Zero for Scalar<E> {
    fn zero() -> Self {
        Scalar::zero()
    }

    fn is_zero(x: &Self) -> Choice {
        x.ct_eq(&Self::zero())
    }
}

impl<E: Curve> crate::traits::One for Scalar<E> {
    fn one() -> Self {
        Self::one()
    }

    fn is_one(x: &Self) -> Choice {
        x.ct_eq(&Self::one())
    }
}

impl<E: Curve> crate::traits::Samplable for Scalar<E> {
    fn random<R: RngCore>(rng: &mut R) -> Self {
        Self::random(rng)
    }
}

impl<E: Curve> iter::Sum for Scalar<E> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + x)
    }
}

impl<'a, E: Curve> iter::Sum<&'a Scalar<E>> for Scalar<E> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + x)
    }
}

impl<E: Curve> iter::Product for Scalar<E> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Scalar::one(), |acc, x| acc * x)
    }
}

impl<'a, E: Curve> iter::Product<&'a Scalar<E>> for Scalar<E> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Scalar::one(), |acc, x| acc * x)
    }
}

macro_rules! impl_from_primitive_integer {
    ($($int:ident),+) => {$(
        impl<E: Curve> From<$int> for Scalar<E> {
            fn from(i: $int) -> Self {
                Scalar::from_le_bytes(&i.to_le_bytes())
                    .expect("scalar should be large enough to fit a primitive integer")
            }
        }
    )+};
}

macro_rules! impl_from_signed_integer {
    ($($iint:ident),+) => {$(
        impl<E: Curve> From<$iint> for Scalar<E> {
            fn from(i: $iint) -> Self {
                use subtle::{ConditionallyNegatable, Choice};
                // TODO: what's a better way to do that check in constant time?
                let is_neg = Choice::from(u8::from(i.is_negative()));
                let i = i.unsigned_abs();
                let mut i = Scalar::from(i);
                i.conditional_negate(is_neg);
                i
            }
        }
    )+};
}

impl_from_primitive_integer! {
    u8, u16, u32, u64, u128, usize
}
impl_from_signed_integer! {
    i8, i16, i32, i64, i128
}

impl<E: Curve> fmt::Debug for Scalar<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Scalar");
        s.field("curve", &E::CURVE_NAME);
        #[cfg(feature = "std")]
        {
            let scalar_hex = hex::encode(self.to_be_bytes());
            s.field("value", &scalar_hex);
        }
        #[cfg(not(feature = "std"))]
        {
            s.field("value", &"...");
        }
        s.finish()
    }
}

#[allow(clippy::derived_hash_with_manual_eq)]
impl<E: Curve> Hash for Scalar<E> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(self.to_be_bytes().as_bytes())
    }
}

impl<E: Curve> PartialOrd for Scalar<E> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.to_be_bytes()
            .as_bytes()
            .partial_cmp(other.to_be_bytes().as_bytes())
    }
}

impl<E: Curve> Ord for Scalar<E> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_be_bytes()
            .as_bytes()
            .cmp(other.to_be_bytes().as_bytes())
    }
}

#[cfg(feature = "udigest")]
impl<E: Curve> udigest::Digestable for Scalar<E> {
    fn unambiguously_encode<B>(&self, encoder: udigest::encoding::EncodeValue<B>)
    where
        B: udigest::Buffer,
    {
        let mut s = encoder.encode_struct();
        s.add_field("curve").encode_leaf().chain(E::CURVE_NAME);
        s.add_field("scalar")
            .encode_leaf()
            .chain(self.to_be_bytes());
        s.finish();
    }
}
