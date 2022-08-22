use core::fmt::Debug;
use core::hash::Hash;

use generic_array::ArrayLength;
use rand_core::{CryptoRng, RngCore};
use subtle::{Choice, ConstantTimeEq, CtOption};
use zeroize::Zeroize;

use crate::errors::HashToCurveError;

pub trait Curve: Debug + Copy + Eq + Ord + Hash {
    type Point: Additive
        + Multiplicative<Self::Scalar>
        + Multiplicative<CurveGenerator>
        + From<CurveGenerator>
        + Zero
        + Zeroize
        + SmallFactor
        + Copy
        + Eq
        + ConstantTimeEq
        + Hash
        + Ord;
    type Scalar: Additive
        + Multiplicative
        + Invertible
        + Zero
        + One
        + Samplable
        + Zeroize
        + Copy
        + Eq
        + ConstantTimeEq
        + Hash
        + Ord;

    type CompressedPointSize: ArrayLength<u8>;
    type UncompressedPointSize: ArrayLength<u8>;
    type CoordinateSize: ArrayLength<u8>;
}

pub trait HashToCurve: Curve {
    fn hash_to_curve(ctx: &[u8], msgs: &[&[u8]]) -> Result<Self::Point, HashToCurveError>;
    fn hash_to_scalar(ctx: &[u8], msgs: &[&[u8]]) -> Result<Self::Scalar, HashToCurveError>;
}

pub trait Additive {
    fn add(a: &Self, b: &Self) -> Self;
    fn add_assign(a: &mut Self, b: &Self);
    fn sub(a: &Self, b: &Self) -> Self;
    fn sub_assign(a: &mut Self, b: &Self);
    fn negate(x: &Self) -> Self;
}

pub trait Multiplicative<Rhs = Self> {
    fn mul(a: &Self, b: &Rhs) -> Self;
    fn mul_assign(a: &mut Self, b: &Rhs);
}

pub trait Invertible
where
    Self: Sized,
{
    fn invert(x: &Self) -> CtOption<Self>;
}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(x: &Self) -> Choice;
}

pub trait One {
    fn one() -> Self;
    fn is_one(x: &Self) -> Choice;
}

pub trait Samplable {
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self;
}

pub trait SmallFactor {
    fn is_torsion_free(&self) -> bool;
}

pub struct CurveGenerator;
