use core::cmp;
use core::fmt;
use core::hash::{self, Hash};

use elliptic_curve::{
    group::cofactor::CofactorGroup,
    sec1::{EncodedPoint, FromEncodedPoint, ModulusSize, ToEncodedPoint},
    FieldSize, Group, ProjectiveArithmetic,
};
use generic_ec_core::*;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};
use zeroize::Zeroize;

pub struct RustCryptoPoint<E: ProjectiveArithmetic>(pub E::ProjectivePoint);

impl<E: ProjectiveArithmetic> Additive for RustCryptoPoint<E> {
    #[inline]
    fn add(a: &Self, b: &Self) -> Self {
        RustCryptoPoint(a.0 + b.0)
    }

    #[inline]
    fn sub(a: &Self, b: &Self) -> Self {
        RustCryptoPoint(a.0 - b.0)
    }

    #[inline]
    fn negate(x: &Self) -> Self {
        RustCryptoPoint(-x.0)
    }
}

impl<E: ProjectiveArithmetic> From<CurveGenerator> for RustCryptoPoint<E> {
    #[inline]
    fn from(_: CurveGenerator) -> Self {
        RustCryptoPoint(E::ProjectivePoint::generator())
    }
}

impl<E: ProjectiveArithmetic> Zero for RustCryptoPoint<E> {
    #[inline]
    fn zero() -> Self {
        RustCryptoPoint(E::ProjectivePoint::identity())
    }

    #[inline]
    fn is_zero(x: &Self) -> Choice {
        x.0.is_identity()
    }
}

impl<E: ProjectiveArithmetic> OnCurve for RustCryptoPoint<E> {
    #[inline]
    fn is_on_curve(&self) -> Choice {
        Choice::from(1)
    }
}

impl<E> SmallFactor for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: CofactorGroup,
{
    #[inline]
    fn is_torsion_free(&self) -> Choice {
        self.0.is_torsion_free()
    }
}

impl<E> ConstantTimeEq for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: ConstantTimeEq,
{
    #[inline]
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl<E> ConditionallySelectable for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: ConditionallySelectable,
{
    #[inline]
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(E::ProjectivePoint::conditional_select(&a.0, &b.0, choice))
    }

    #[inline]
    fn conditional_assign(&mut self, other: &Self, choice: Choice) {
        self.0.conditional_assign(&other.0, choice)
    }

    #[inline]
    fn conditional_swap(a: &mut Self, b: &mut Self, choice: Choice) {
        E::ProjectivePoint::conditional_swap(&mut a.0, &mut b.0, choice)
    }
}

impl<E> CompressedEncoding for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: ToEncodedPoint<E>,
    FieldSize<E>: ModulusSize,
{
    type Bytes = elliptic_curve::sec1::CompressedPoint<E>;
    fn to_bytes_compressed(&self) -> Self::Bytes {
        let point_encoded = self.0.to_encoded_point(true);

        let mut bytes = Self::Bytes::default();
        bytes.copy_from_slice(point_encoded.as_bytes());

        bytes
    }
}

impl<E> UncompressedEncoding for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: ToEncodedPoint<E>,
    FieldSize<E>: ModulusSize,
{
    type Bytes = elliptic_curve::sec1::UncompressedPoint<E>;
    fn to_bytes_uncompressed(&self) -> Self::Bytes {
        let point_encoded = self.0.to_encoded_point(false);

        let mut bytes = Self::Bytes::default();
        bytes.copy_from_slice(point_encoded.as_bytes());

        bytes
    }
}

impl<E> Decode for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: FromEncodedPoint<E>,
    FieldSize<E>: ModulusSize,
{
    fn decode(bytes: &[u8]) -> Option<Self> {
        let encoded_point = EncodedPoint::<E>::from_bytes(bytes).ok()?;
        Option::from(E::ProjectivePoint::from_encoded_point(&encoded_point)).map(Self)
    }
}

impl<E> fmt::Debug for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> Clone for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<E> Copy for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: Copy,
{
}

impl<E> Zeroize for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: Zeroize,
{
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}

impl<E> PartialEq for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl<E> Eq for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: Eq,
{
}

impl<E> Hash for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<E> PartialOrd for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<E> Ord for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<E> Default for RustCryptoPoint<E>
where
    E: ProjectiveArithmetic,
    E::ProjectivePoint: Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
