use core::ops::Mul;

use elliptic_curve::ops::Reduce;
use elliptic_curve::{
    Field, FieldBytes, Group, PrimeField, ProjectiveArithmetic, ScalarArithmetic, ScalarCore,
};
use generic_ec_core::*;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::DefaultIsZeroes;

pub struct RustCryptoScalar<E: ScalarArithmetic>(pub E::Scalar);

impl<E: ScalarArithmetic> Additive for RustCryptoScalar<E> {
    fn add(a: &Self, b: &Self) -> Self {
        Self(a.0 + b.0)
    }

    fn sub(a: &Self, b: &Self) -> Self {
        Self(a.0 - b.0)
    }

    fn negate(x: &Self) -> Self {
        Self(-x.0)
    }
}

impl<E: ScalarArithmetic> Multiplicative<RustCryptoScalar<E>> for RustCryptoScalar<E> {
    type Output = RustCryptoScalar<E>;

    fn mul(a: &Self, b: &RustCryptoScalar<E>) -> Self::Output {
        Self(a.0 * b.0)
    }
}

impl<E> Multiplicative<super::RustCryptoPoint<E>> for RustCryptoScalar<E>
where
    E: ProjectiveArithmetic + ScalarArithmetic,
    for<'a> &'a E::ProjectivePoint: Mul<&'a E::Scalar, Output = E::ProjectivePoint>,
{
    type Output = super::RustCryptoPoint<E>;

    fn mul(a: &Self, b: &super::RustCryptoPoint<E>) -> Self::Output {
        super::RustCryptoPoint(&b.0 * &a.0)
    }
}

impl<E> Multiplicative<CurveGenerator> for RustCryptoScalar<E>
where
    E: ProjectiveArithmetic + ScalarArithmetic,
    for<'a> &'a E::ProjectivePoint: Mul<&'a E::Scalar, Output = E::ProjectivePoint>,
{
    type Output = super::RustCryptoPoint<E>;

    fn mul(a: &Self, _b: &CurveGenerator) -> Self::Output {
        super::RustCryptoPoint(&E::ProjectivePoint::generator() * &a.0)
    }
}

impl<E: ScalarArithmetic> Invertible for RustCryptoScalar<E> {
    fn invert(x: &Self) -> CtOption<Self> {
        x.0.invert().map(Self)
    }
}

impl<E: ScalarArithmetic> Zero for RustCryptoScalar<E> {
    fn zero() -> Self {
        Self(E::Scalar::zero())
    }

    fn is_zero(x: &Self) -> subtle::Choice {
        x.0.is_zero()
    }
}

impl<E: ScalarArithmetic> One for RustCryptoScalar<E> {
    fn one() -> Self {
        Self(E::Scalar::one())
    }

    fn is_one(x: &Self) -> Choice {
        x.0.ct_eq(&E::Scalar::one())
    }
}

impl<E: ScalarArithmetic> Samplable for RustCryptoScalar<E> {
    fn random<R: rand_core::RngCore>(rng: &mut R) -> Self {
        let mut bytes: <E::Scalar as PrimeField>::Repr = Default::default();

        loop {
            rng.fill_bytes(bytes.as_mut());

            if let Some(scalar) = <E::Scalar as PrimeField>::from_repr_vartime(bytes.clone()) {
                break Self(scalar);
            }
        }
    }
}

impl<E: ScalarArithmetic> Default for RustCryptoScalar<E> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<E: ScalarArithmetic> Clone for RustCryptoScalar<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<E: ScalarArithmetic> Copy for RustCryptoScalar<E> {}

impl<E> DefaultIsZeroes for RustCryptoScalar<E>
where
    E: ScalarArithmetic,
    E::Scalar: DefaultIsZeroes,
{
}

impl<E> PartialEq for RustCryptoScalar<E>
where
    E: ScalarArithmetic,
    E::Scalar: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<E> Eq for RustCryptoScalar<E>
where
    E: ScalarArithmetic,
    E::Scalar: Eq,
{
}

impl<E> ConstantTimeEq for RustCryptoScalar<E>
where
    E: ScalarArithmetic,
    E::Scalar: ConstantTimeEq,
{
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl<E> ConditionallySelectable for RustCryptoScalar<E>
where
    E: ScalarArithmetic,
    E::Scalar: ConditionallySelectable,
{
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(E::Scalar::conditional_select(&a.0, &b.0, choice))
    }
}

impl<E: ScalarArithmetic> IntegerEncoding for RustCryptoScalar<E>
where
    for<'s> ScalarCore<E>: From<&'s E::Scalar>,
    E::Scalar: Reduce<E::UInt>,
{
    type Bytes = FieldBytes<E>;

    fn to_be_bytes(&self) -> Self::Bytes {
        let scalar_core = ScalarCore::<E>::from(&self.0);
        scalar_core.to_be_bytes()
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        let scalar_core = ScalarCore::<E>::from(&self.0);
        scalar_core.to_le_bytes()
    }

    fn from_be_bytes(bytes: &Self::Bytes) -> Self {
        Self(E::Scalar::from_be_bytes_reduced(bytes.clone()))
    }

    fn from_le_bytes(bytes: &Self::Bytes) -> Self {
        Self(E::Scalar::from_le_bytes_reduced(bytes.clone()))
    }

    fn from_be_bytes_exact(bytes: &Self::Bytes) -> Option<Self> {
        let scalar_core: Option<ScalarCore<E>> =
            ScalarCore::<E>::from_be_bytes(bytes.clone()).into();
        Some(Self(E::Scalar::from(scalar_core?)))
    }

    fn from_le_bytes_exact(bytes: &Self::Bytes) -> Option<Self> {
        let scalar_core: Option<ScalarCore<E>> =
            ScalarCore::<E>::from_le_bytes(bytes.clone()).into();
        Some(Self(E::Scalar::from(scalar_core?)))
    }
}
