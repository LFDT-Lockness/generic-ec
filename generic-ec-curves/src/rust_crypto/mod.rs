use core::fmt;
use core::hash::{self, Hash};
use core::marker::PhantomData;
use core::ops::Mul;

use elliptic_curve::group::cofactor::CofactorGroup;
use elliptic_curve::ops::Reduce;
use elliptic_curve::sec1::{FromEncodedPoint, ModulusSize, ToEncodedPoint};
use elliptic_curve::{FieldSize, ProjectiveArithmetic, ScalarArithmetic, ScalarCore};
use generic_ec_core::{CompressedEncoding, Curve, IntegerEncoding, UncompressedEncoding};
use subtle::{ConditionallySelectable, ConstantTimeEq};
use zeroize::{DefaultIsZeroes, Zeroize};

pub use self::{curve_name::CurveName, point::RustCryptoPoint, scalar::RustCryptoScalar};

mod curve_name;
mod point;
mod scalar;

pub struct RustCryptoCurve<C> {
    _ph: PhantomData<fn() -> C>,
}

#[cfg(feature = "secp256k1")]
pub type Secp256k1 = RustCryptoCurve<k256::Secp256k1>;
#[cfg(feature = "secp256r1")]
pub type Secp2556r1 = RustCryptoCurve<p256::NistP256>;

impl<C> Curve for RustCryptoCurve<C>
where
    C: CurveName + ScalarArithmetic + ProjectiveArithmetic,
    C::ProjectivePoint: CofactorGroup
        + ToEncodedPoint<C>
        + FromEncodedPoint<C>
        + fmt::Debug
        + Copy
        + Eq
        + Ord
        + Hash
        + Default
        + ConstantTimeEq
        + ConditionallySelectable
        + Zeroize,
    for<'a> &'a C::ProjectivePoint: Mul<&'a C::Scalar, Output = C::ProjectivePoint>,
    C::Scalar: Reduce<C::UInt>
        + Eq
        + Ord
        + Hash
        + ConstantTimeEq
        + ConditionallySelectable
        + DefaultIsZeroes,
    for<'a> ScalarCore<C>: From<&'a C::Scalar>,
    FieldSize<C>: ModulusSize,
{
    const CURVE_NAME: &'static str = C::CURVE_NAME;

    type Point = RustCryptoPoint<C>;
    type Scalar = RustCryptoScalar<C>;

    type CompressedPointArray = <Self::Point as CompressedEncoding>::Bytes;
    type UncompressedPointArray = <Self::Point as UncompressedEncoding>::Bytes;

    type ScalarArray = <Self::Scalar as IntegerEncoding>::Bytes;

    type CoordinateArray = [u8; 0];
}

impl<C: CurveName> fmt::Debug for RustCryptoCurve<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RustCryptoCurve")
            .field("curve", &C::CURVE_NAME)
            .finish()
    }
}

impl<C> Clone for RustCryptoCurve<C> {
    fn clone(&self) -> Self {
        Self { _ph: PhantomData }
    }
}

impl<C> Copy for RustCryptoCurve<C> {}

impl<C> PartialEq for RustCryptoCurve<C> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<C> Eq for RustCryptoCurve<C> {}

impl<C> PartialOrd for RustCryptoCurve<C> {
    fn partial_cmp(&self, _other: &Self) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Equal)
    }
}

impl<C> Ord for RustCryptoCurve<C> {
    fn cmp(&self, _other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}

impl<C> Hash for RustCryptoCurve<C>
where
    C: CurveName,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(C::CURVE_NAME.as_bytes())
    }
}

impl<C> Default for RustCryptoCurve<C> {
    fn default() -> Self {
        Self { _ph: PhantomData }
    }
}
