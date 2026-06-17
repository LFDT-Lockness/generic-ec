use core::{
    cmp,
    iter::{self, Product, Sum},
};

use rand_core::{CryptoRng, RngCore};
use subtle::{ConstantTimeEq, CtOption};

use crate::{
    as_raw::FromRaw,
    core::{ByteArray, FromUniformBytes},
    errors::{ZeroPoint, ZeroScalar},
    Curve, Point, Scalar, SecretPoint, SecretScalar,
};

use self::definition::NonZero;

pub mod coords;
pub mod definition;

impl<E: Curve> NonZero<Point<E>> {
    /// Constructs non-zero point
    ///
    /// Returns `None` if point is zero
    pub fn from_point(point: Point<E>) -> Option<Self> {
        Self::ct_from_point(point).into()
    }

    /// Constructs non-zero point (constant time)
    ///
    /// Returns `None` if point is zero
    pub fn ct_from_point(point: Point<E>) -> CtOption<Self> {
        let zero = Point::zero();
        let is_non_zero = !point.ct_eq(&zero);

        // Correctness: although we technically construct `NonZero` regardless if
        // it's actually non-zero, `CtOption` never exposes it, so `NonZero` with
        // zero value is not accessible by anyone
        CtOption::new(Self::new_unchecked(point), is_non_zero)
    }

    /// Convert this value into a `NonZero<SecretPoint<E>>`. You should do this at the end
    /// of computations that produce a secret, like a key exchange
    pub fn into_secret(self) -> NonZero<SecretPoint<E>> {
        let mut point = self.into_inner();
        let secret_point = SecretPoint::new(&mut point);
        // Correctness: `point` was checked to be nonzero
        NonZero::new_unchecked(secret_point)
    }
}

impl<E: Curve> NonZero<SecretPoint<E>> {
    /// Returns the generator defined in the curve specs
    pub fn generator() -> Self {
        // Correctness: generator of a non-degenerate group is not zero
        Self::new_unchecked(SecretPoint::generator())
    }

    /// Constructs non-zero point
    ///
    /// Returns `None` if point is zero
    pub fn from_secret_point(point: SecretPoint<E>) -> Option<Self> {
        Self::ct_from_secret_point(point).into()
    }

    /// Constructs non-zero point (constant time)
    ///
    /// Returns `None` if point is zero
    pub fn ct_from_secret_point(secret_point: SecretPoint<E>) -> CtOption<Self> {
        let zero = Point::zero();
        let is_non_zero = !secret_point.as_ref().ct_eq(&zero);

        // Correctness: although we technically construct `NonZero` regardless if
        // it's actually non-zero, `CtOption` never exposes it, so `NonZero` with
        // zero value is not accessible by anyone
        CtOption::new(Self::new_unchecked(secret_point), is_non_zero)
    }
}

impl<E: Curve> NonZero<Scalar<E>> {
    #[doc = include_str!("../../docs/nonzero_scalar_random.md")]
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        match iter::repeat_with(|| {
            let mut bytes = <<E::Scalar as FromUniformBytes>::Bytes as ByteArray>::zeroes();
            rng.fill_bytes(bytes.as_mut());
            <E::Scalar as FromUniformBytes>::from_uniform_bytes(&bytes)
        })
        .take(100)
        .flat_map(|s| NonZero::from_scalar(Scalar::from_raw(s)))
        .next()
        {
            Some(s) => s,
            None => panic!("defected source of randomness"),
        }
    }

    #[doc = include_str!("../../docs/nonzero_scalar_random_vartime.md")]
    pub fn random_vartime<R: RngCore>(rng: &mut R) -> Self {
        match iter::repeat_with(|| {
            <E::Scalar as generic_ec_core::SamplableVartime>::random_vartime(rng)
        })
        .take(100)
        .flat_map(|s| NonZero::from_scalar(Scalar::from_raw(s)))
        .next()
        {
            Some(s) => s,
            None => panic!("defected source of randomness"),
        }
    }

    #[doc = include_str!("../../docs/hash_to_scalar.md")]
    ///
    /// ## Example
    /// ```rust
    /// use generic_ec::{Scalar, NonZero, curves::Secp256k1};
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
    /// let scalar = NonZero::<Scalar<Secp256k1>>::from_hash::<Sha256>(&Data {
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

    /// Constructs $S = 1$
    pub fn one() -> Self {
        // Correctness: constructed scalar = 1, so it's non-zero
        Self::new_unchecked(Scalar::one())
    }

    /// Constructs non-zero scalar
    ///
    /// Returns `None` if scalar is zero
    pub fn from_scalar(scalar: Scalar<E>) -> Option<Self> {
        Self::ct_from_scalar(scalar).into()
    }

    /// Constructs non-zero scalar (constant time)
    ///
    /// Returns `None` if scalar is zero
    pub fn ct_from_scalar(scalar: Scalar<E>) -> CtOption<Self> {
        let zero = Scalar::zero();
        let is_non_zero = !scalar.ct_eq(&zero);

        // Correctness: although we technically construct `NonZero` regardless if
        // it's actually non-zero, `CtOption` never exposes it, so `NonZero` with
        // zero value is not accessible by anyone
        CtOption::new(Self::new_unchecked(scalar), is_non_zero)
    }

    /// Returns scalar inverse $S^{-1}$
    ///
    /// Similar to [Scalar::invert], but this function is always defined as inverse is defined for all
    /// non-zero scalars
    pub fn invert(&self) -> NonZero<Scalar<E>> {
        #[allow(clippy::expect_used)]
        let inv = (**self)
            .invert()
            .expect("nonzero scalar always has an invert");
        // Correctness: `inv` is nonzero by definition
        Self::new_unchecked(inv)
    }

    /// Upgrades the non-zero scalar into non-zero [`SecretScalar`]
    pub fn into_secret(self) -> NonZero<SecretScalar<E>> {
        let mut scalar = self.into_inner();
        let secret_scalar = SecretScalar::new(&mut scalar);
        // Correctness: `scalar` was checked to be nonzero
        NonZero::new_unchecked(secret_scalar)
    }
}

impl<E: Curve> NonZero<SecretScalar<E>> {
    #[doc = include_str!("../../docs/nonzero_scalar_random.md")]
    pub fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        <Self as crate::traits::Samplable>::random(rng)
    }

    #[doc = include_str!("../../docs/nonzero_scalar_random_vartime.md")]
    pub fn random_vartime<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        <Self as crate::traits::Samplable>::random_vartime(rng)
    }

    /// Constructs $S = 1$
    pub fn one() -> Self {
        // Correctness: constructed scalar = 1, so it's non-zero
        Self::new_unchecked(SecretScalar::one())
    }

    /// Constructs non-zero scalar
    ///
    /// Returns `None` if scalar is zero
    pub fn from_secret_scalar(scalar: SecretScalar<E>) -> Option<Self> {
        Self::ct_from_secret_scalar(scalar).into()
    }

    /// Constructs non-zero scalar (constant time)
    ///
    /// Returns `None` if scalar is zero
    pub fn ct_from_secret_scalar(secret_scalar: SecretScalar<E>) -> CtOption<Self> {
        let zero = Scalar::zero();
        let is_non_zero = !secret_scalar.as_ref().ct_eq(&zero);

        // Correctness: although we technically construct `NonZero` regardless if
        // it's actually non-zero, `CtOption` never exposes it, so `NonZero` with
        // zero value is not accessible by anyone
        CtOption::new(Self::new_unchecked(secret_scalar), is_non_zero)
    }

    /// Returns scalar inverse $S^{-1}$
    ///
    /// Similar to [SecretScalar::invert], but this function is always defined as inverse is defined for all
    /// non-zero scalars
    pub fn invert(&self) -> NonZero<SecretScalar<E>> {
        #[allow(clippy::expect_used)]
        let inv = (**self)
            .invert()
            .expect("nonzero scalar always has an invert");
        // Correctness: `inv` is nonzero by definition
        Self::new_unchecked(inv)
    }
}

impl<E: Curve> From<NonZero<Point<E>>> for Point<E> {
    fn from(point: NonZero<Point<E>>) -> Self {
        point.into_inner()
    }
}

impl<E: Curve> From<NonZero<SecretPoint<E>>> for SecretPoint<E> {
    fn from(secret_point: NonZero<SecretPoint<E>>) -> Self {
        secret_point.into_inner()
    }
}

impl<E: Curve> From<NonZero<Scalar<E>>> for Scalar<E> {
    fn from(scalar: NonZero<Scalar<E>>) -> Self {
        scalar.into_inner()
    }
}

impl<E: Curve> From<NonZero<SecretScalar<E>>> for SecretScalar<E> {
    fn from(secret_scalar: NonZero<SecretScalar<E>>) -> Self {
        secret_scalar.into_inner()
    }
}

impl<E: Curve> TryFrom<Point<E>> for NonZero<Point<E>> {
    type Error = ZeroPoint;

    fn try_from(point: Point<E>) -> Result<Self, Self::Error> {
        Self::from_point(point).ok_or(ZeroPoint)
    }
}

impl<E: Curve> TryFrom<SecretPoint<E>> for NonZero<SecretPoint<E>> {
    type Error = ZeroPoint;

    fn try_from(secret_point: SecretPoint<E>) -> Result<Self, Self::Error> {
        Self::from_secret_point(secret_point).ok_or(ZeroPoint)
    }
}

impl<E: Curve> TryFrom<Scalar<E>> for NonZero<Scalar<E>> {
    type Error = ZeroScalar;

    fn try_from(scalar: Scalar<E>) -> Result<Self, Self::Error> {
        Self::from_scalar(scalar).ok_or(ZeroScalar)
    }
}

impl<E: Curve> TryFrom<SecretScalar<E>> for NonZero<SecretScalar<E>> {
    type Error = ZeroScalar;

    fn try_from(secret_scalar: SecretScalar<E>) -> Result<Self, Self::Error> {
        Self::from_secret_scalar(secret_scalar).ok_or(ZeroScalar)
    }
}

impl<E: Curve> Sum<NonZero<Scalar<E>>> for Scalar<E> {
    fn sum<I: Iterator<Item = NonZero<Scalar<E>>>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + x)
    }
}

impl<'s, E: Curve> Sum<&'s NonZero<Scalar<E>>> for Scalar<E> {
    fn sum<I: Iterator<Item = &'s NonZero<Scalar<E>>>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + x)
    }
}

impl<'s, E: Curve> Sum<&'s NonZero<SecretScalar<E>>> for SecretScalar<E> {
    fn sum<I: Iterator<Item = &'s NonZero<SecretScalar<E>>>>(iter: I) -> Self {
        let mut out = Scalar::zero();
        iter.for_each(|x| out += x);
        SecretScalar::new(&mut out)
    }
}

impl<E: Curve> Sum<NonZero<SecretScalar<E>>> for SecretScalar<E> {
    fn sum<I: Iterator<Item = NonZero<SecretScalar<E>>>>(iter: I) -> Self {
        let mut out = Scalar::zero();
        iter.for_each(|x| out += x);
        SecretScalar::new(&mut out)
    }
}

impl<E: Curve> Product<NonZero<Scalar<E>>> for NonZero<Scalar<E>> {
    fn product<I: Iterator<Item = NonZero<Scalar<E>>>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}

impl<'s, E: Curve> Product<&'s NonZero<Scalar<E>>> for NonZero<Scalar<E>> {
    fn product<I: Iterator<Item = &'s NonZero<Scalar<E>>>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}

impl<'s, E: Curve> Product<&'s NonZero<SecretScalar<E>>> for NonZero<SecretScalar<E>> {
    fn product<I: Iterator<Item = &'s NonZero<SecretScalar<E>>>>(iter: I) -> Self {
        let mut out = NonZero::<Scalar<E>>::one();
        iter.for_each(|x| out *= x);
        out.into_secret()
    }
}

impl<E: Curve> Product<NonZero<SecretScalar<E>>> for NonZero<SecretScalar<E>> {
    fn product<I: Iterator<Item = NonZero<SecretScalar<E>>>>(iter: I) -> Self {
        let mut out = NonZero::<Scalar<E>>::one();
        iter.for_each(|x| out *= x);
        out.into_secret()
    }
}

impl<E: Curve> Sum<NonZero<Point<E>>> for Point<E> {
    fn sum<I: Iterator<Item = NonZero<Point<E>>>>(iter: I) -> Self {
        iter.fold(Point::zero(), |acc, x| acc + x)
    }
}
impl<'s, E: Curve> Sum<&'s NonZero<Point<E>>> for Point<E> {
    fn sum<I: Iterator<Item = &'s NonZero<Point<E>>>>(iter: I) -> Self {
        iter.fold(Point::zero(), |acc, x| acc + x)
    }
}

impl<E: Curve> Sum<NonZero<SecretPoint<E>>> for SecretPoint<E> {
    fn sum<I: Iterator<Item = NonZero<SecretPoint<E>>>>(iter: I) -> Self {
        let mut out = Point::zero();
        iter.for_each(|x| out += x);
        SecretPoint::new(&mut out)
    }
}
impl<'s, E: Curve> Sum<&'s NonZero<SecretPoint<E>>> for SecretPoint<E> {
    fn sum<I: Iterator<Item = &'s NonZero<SecretPoint<E>>>>(iter: I) -> Self {
        let mut out = Point::zero();
        iter.for_each(|x| out += x);
        SecretPoint::new(&mut out)
    }
}

impl<E: Curve> crate::traits::Samplable for NonZero<Scalar<E>> {
    fn random<R: RngCore>(rng: &mut R) -> Self {
        Self::random(rng)
    }

    fn random_vartime<R: rand_core::RngCore>(rng: &mut R) -> Self {
        Self::random_vartime(rng)
    }
}

impl<E: Curve> crate::traits::Samplable for NonZero<SecretScalar<E>> {
    fn random<R: RngCore>(rng: &mut R) -> Self {
        NonZero::<Scalar<E>>::random(rng).into_secret()
    }

    fn random_vartime<R: rand_core::RngCore>(rng: &mut R) -> Self {
        NonZero::<Scalar<E>>::random_vartime(rng).into_secret()
    }
}

impl<T> crate::traits::IsZero for NonZero<T> {
    /// Returns `false` as `NonZero<T>` cannot be zero
    #[inline(always)]
    fn is_zero(&self) -> bool {
        false
    }
}

impl<E: Curve> crate::traits::One for NonZero<Scalar<E>> {
    fn one() -> Self {
        Self::one()
    }

    fn is_one(x: &Self) -> subtle::Choice {
        x.ct_eq(&Self::one())
    }
}

impl<E: Curve> AsRef<Point<E>> for NonZero<SecretPoint<E>> {
    fn as_ref(&self) -> &Point<E> {
        let secret_point: &SecretPoint<E> = self.as_ref();
        secret_point.as_ref()
    }
}

impl<E: Curve> AsRef<Scalar<E>> for NonZero<SecretScalar<E>> {
    fn as_ref(&self) -> &Scalar<E> {
        let secret_scalar: &SecretScalar<E> = self.as_ref();
        secret_scalar.as_ref()
    }
}

impl<T> cmp::PartialEq<T> for NonZero<T>
where
    T: cmp::PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.as_ref() == other
    }
}

impl<T> cmp::PartialOrd<T> for NonZero<T>
where
    T: cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<cmp::Ordering> {
        self.as_ref().partial_cmp(other)
    }
}

/// We can't write blanket implementation `impl<T> cmp::PartialEq<NonZero<T>> for T` due to
/// the restrictions of the compiler, which implies unfortunate limitations that we can
/// do `a == b` but we can't write `b == a` and that's not user-friendly.
///
/// However, we can write implementation of PartialEq/PartialOrd for specific `T` such as
/// `Scalar<E>`, `Point<E>` and others. Moreover, we know for sure all possible `T` for which
/// `NonZero<T>` is defined, so we use this macro to implement these traits for all possible `T`.
macro_rules! impl_reverse_partial_eq_cmp {
    ($($t:ty),+) => {$(
        impl<E: Curve> cmp::PartialEq<NonZero<$t>> for $t {
            fn eq(&self, other: &NonZero<$t>) -> bool {
                let other: &$t = other.as_ref();
                self == other
            }
        }
        impl<E: Curve> cmp::PartialOrd<NonZero<$t>> for $t {
            fn partial_cmp(&self, other: &NonZero<$t>) -> Option<cmp::Ordering> {
                let other: &$t = other.as_ref();
                self.partial_cmp(other)
            }
        }
    )*};
}

// Note: not implemented for SecretScalar and SecretPoint as they don't
// implement `PartialEq` for security reasons.
impl_reverse_partial_eq_cmp!(Point<E>, Scalar<E>);

impl<T: ConstantTimeEq> ConstantTimeEq for NonZero<T> {
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        self.as_ref().ct_eq(other.as_ref())
    }
}

#[cfg(all(test, feature = "serde"))]
mod non_zero_is_serializable {
    use crate::{Curve, NonZero, Point, Scalar, SecretPoint, SecretScalar};

    fn impls_serde<T>()
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
    }

    #[allow(dead_code)]
    fn ensure_non_zero_is_serde<E: Curve>() {
        impls_serde::<NonZero<Point<E>>>();
        impls_serde::<NonZero<SecretPoint<E>>>();
        impls_serde::<NonZero<Scalar<E>>>();
        impls_serde::<NonZero<SecretScalar<E>>>();
    }
}
