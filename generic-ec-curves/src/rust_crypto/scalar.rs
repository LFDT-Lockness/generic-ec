use core::ops::Mul;

use elliptic_curve::bigint::{ArrayEncoding, ByteArray, U256, U512};
use elliptic_curve::{Curve, CurveArithmetic, Field, Group, PrimeField, ScalarPrimitive};
use generic_ec_core::{
    Additive, CurveGenerator, IntegerEncoding, Invertible, Multiplicative, One, Reduce, Samplable,
    Zero,
};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::DefaultIsZeroes;

/// Scalar wrapper
pub struct RustCryptoScalar<E: CurveArithmetic>(pub E::Scalar);

impl<E: CurveArithmetic> Additive for RustCryptoScalar<E> {
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

impl<E: CurveArithmetic> Multiplicative<RustCryptoScalar<E>> for RustCryptoScalar<E> {
    type Output = RustCryptoScalar<E>;

    fn mul(a: &Self, b: &RustCryptoScalar<E>) -> Self::Output {
        Self(a.0 * b.0)
    }
}

impl<E> Multiplicative<super::RustCryptoPoint<E>> for RustCryptoScalar<E>
where
    E: CurveArithmetic,
    for<'a> &'a E::ProjectivePoint: Mul<&'a E::Scalar, Output = E::ProjectivePoint>,
{
    type Output = super::RustCryptoPoint<E>;

    fn mul(a: &Self, b: &super::RustCryptoPoint<E>) -> Self::Output {
        super::RustCryptoPoint(b.0 * a.0)
    }
}

impl<E> Multiplicative<CurveGenerator> for RustCryptoScalar<E>
where
    E: CurveArithmetic,
    for<'a> &'a E::ProjectivePoint: Mul<&'a E::Scalar, Output = E::ProjectivePoint>,
{
    type Output = super::RustCryptoPoint<E>;

    fn mul(a: &Self, _b: &CurveGenerator) -> Self::Output {
        super::RustCryptoPoint(E::ProjectivePoint::generator() * a.0)
    }
}

impl<E: CurveArithmetic> Invertible for RustCryptoScalar<E> {
    fn invert(x: &Self) -> CtOption<Self> {
        x.0.invert().map(Self)
    }
}

impl<E: CurveArithmetic> Zero for RustCryptoScalar<E> {
    fn zero() -> Self {
        Self(E::Scalar::ZERO)
    }

    fn is_zero(x: &Self) -> subtle::Choice {
        x.0.is_zero()
    }
}

impl<E: CurveArithmetic> One for RustCryptoScalar<E> {
    fn one() -> Self {
        Self(E::Scalar::ONE)
    }

    fn is_one(x: &Self) -> Choice {
        x.0.ct_eq(&E::Scalar::ONE)
    }
}

impl<E: CurveArithmetic> Samplable for RustCryptoScalar<E> {
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

impl<E: CurveArithmetic> Default for RustCryptoScalar<E> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<E: CurveArithmetic> Clone for RustCryptoScalar<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E: CurveArithmetic> Copy for RustCryptoScalar<E> {}

impl<E> DefaultIsZeroes for RustCryptoScalar<E>
where
    E: CurveArithmetic,
    E::Scalar: DefaultIsZeroes,
{
}

impl<E> PartialEq for RustCryptoScalar<E>
where
    E: CurveArithmetic,
    E::Scalar: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<E> Eq for RustCryptoScalar<E>
where
    E: CurveArithmetic,
    E::Scalar: Eq,
{
}

impl<E> ConstantTimeEq for RustCryptoScalar<E>
where
    E: CurveArithmetic,
    E::Scalar: ConstantTimeEq,
{
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl<E> ConditionallySelectable for RustCryptoScalar<E>
where
    E: CurveArithmetic,
    E::Scalar: ConditionallySelectable,
{
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(E::Scalar::conditional_select(&a.0, &b.0, choice))
    }
}

impl<E: CurveArithmetic + Curve> IntegerEncoding for RustCryptoScalar<E>
where
    for<'s> ScalarPrimitive<E>: From<&'s E::Scalar>,
    E::Scalar: elliptic_curve::ops::Reduce<E::Uint>,
    Self: BytesModOrder,
{
    type Bytes = ByteArray<E::Uint>;

    fn to_be_bytes(&self) -> Self::Bytes {
        let scalar_core = ScalarPrimitive::<E>::from(&self.0);
        let uint = scalar_core.as_uint();
        uint.to_be_byte_array()
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        let scalar_core = ScalarPrimitive::<E>::from(&self.0);
        let uint = scalar_core.as_uint();
        uint.to_le_byte_array()
    }

    fn from_be_bytes_exact(bytes: &Self::Bytes) -> Option<Self> {
        let uint = E::Uint::from_be_byte_array(bytes.clone());
        let scalar_core: Option<ScalarPrimitive<E>> = ScalarPrimitive::<E>::new(uint).into();
        Some(Self(E::Scalar::from(scalar_core?)))
    }

    fn from_le_bytes_exact(bytes: &Self::Bytes) -> Option<Self> {
        let uint = E::Uint::from_le_byte_array(bytes.clone());
        let scalar_core: Option<ScalarPrimitive<E>> = ScalarPrimitive::<E>::new(uint).into();
        Some(Self(E::Scalar::from(scalar_core?)))
    }

    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        <Self as BytesModOrder>::from_be_bytes_mod_order(bytes)
    }
    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        <Self as BytesModOrder>::from_le_bytes_mod_order(bytes)
    }
}

impl<E: CurveArithmetic + Curve> Reduce<32> for RustCryptoScalar<E>
where
    E::Scalar: elliptic_curve::ops::Reduce<U256>,
{
    fn from_be_array_mod_order(bytes: &[u8; 32]) -> Self {
        Self(elliptic_curve::ops::Reduce::<U256>::reduce(
            U256::from_be_byte_array((*bytes).into()),
        ))
    }
    fn from_le_array_mod_order(bytes: &[u8; 32]) -> Self {
        Self(elliptic_curve::ops::Reduce::<U256>::reduce(
            U256::from_le_byte_array((*bytes).into()),
        ))
    }
}

impl<E: CurveArithmetic + Curve> Reduce<64> for RustCryptoScalar<E>
where
    E::Scalar: elliptic_curve::ops::Reduce<U512>,
{
    fn from_be_array_mod_order(bytes: &[u8; 64]) -> Self {
        Self(elliptic_curve::ops::Reduce::<U512>::reduce(
            U512::from_be_byte_array((*bytes).into()),
        ))
    }
    fn from_le_array_mod_order(bytes: &[u8; 64]) -> Self {
        Self(elliptic_curve::ops::Reduce::<U512>::reduce(
            U512::from_le_byte_array((*bytes).into()),
        ))
    }
}

/// Choice of algorithm for computing bytes mod curve order. Efficient algorithm
/// is different for different curves.
pub(super) trait BytesModOrder {
    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self;
    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self;
}

#[cfg(feature = "secp256k1")]
impl BytesModOrder for RustCryptoScalar<k256::Secp256k1> {
    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_be_bytes_mod_order_reducing_32_64(bytes, &Self(k256::Scalar::ONE))
    }
    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_le_bytes_mod_order_reducing_32_64(bytes, &Self(k256::Scalar::ONE))
    }
}
#[cfg(feature = "secp256r1")]
impl BytesModOrder for RustCryptoScalar<p256::NistP256> {
    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_be_bytes_mod_order_reducing_32(bytes, &Self(p256::Scalar::ONE))
    }
    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_le_bytes_mod_order_reducing_32(bytes, &Self(p256::Scalar::ONE))
    }
}
#[cfg(feature = "stark")]
impl BytesModOrder for RustCryptoScalar<stark_curve::StarkCurve> {
    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_be_bytes_mod_order_reducing_32(
            bytes,
            &Self(stark_curve::Scalar::ONE),
        )
    }
    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_le_bytes_mod_order_reducing_32(
            bytes,
            &Self(stark_curve::Scalar::ONE),
        )
    }
}
