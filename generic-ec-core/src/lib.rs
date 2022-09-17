#![no_std]

use core::fmt::Debug;
use core::hash::Hash;

use generic_array::{ArrayLength, GenericArray};
use rand_core::{CryptoRng, RngCore};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::Zeroize;

pub mod coords;

pub trait Curve: Debug + Copy + Eq + Ord + Hash + Default + Sync + Send {
    type Point: Additive
        + From<CurveGenerator>
        + Zero
        + Zeroize
        + OnCurve
        + SmallFactor
        + Copy
        + Eq
        + ConstantTimeEq
        + Hash
        + Ord
        + ConditionallySelectable
        + Default
        + Encoding<Self::CompressedPointArray>
        + Encoding<Self::UncompressedPointArray>
        + Decode
        + Sync
        + Send;
    type Scalar: Additive
        + Multiplicative<Self::Scalar, Output = Self::Scalar>
        + Multiplicative<CurveGenerator, Output = Self::Point>
        + Multiplicative<Self::Point, Output = Self::Point>
        + Invertible
        + Zero
        + One
        + Samplable
        + Zeroize
        + Canonical
        + Copy
        + Eq
        + ConstantTimeEq
        + Hash
        + Ord
        + ConditionallySelectable
        + Default
        + Encoding<Self::ScalarArray>
        + Decode
        + Sync
        + Send;

    type CompressedPointArray: ByteArray;
    type UncompressedPointArray: ByteArray;
    type ScalarArray: ByteArray;
    type CoordinateArray: ByteArray;
}

pub trait HashToCurve: Curve {
    fn hash_to_curve(ctx: &[u8], msgs: &[&[u8]]) -> Result<Self::Point, Error>;
    fn hash_to_scalar(ctx: &[u8], msgs: &[&[u8]]) -> Result<Self::Scalar, Error>;
}

pub trait Additive {
    fn add(a: &Self, b: &Self) -> Self;
    fn sub(a: &Self, b: &Self) -> Self;
    fn negate(x: &Self) -> Self;
}

pub trait Multiplicative<Rhs> {
    type Output;
    fn mul(a: &Self, b: &Rhs) -> Self::Output;
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

pub trait OnCurve {
    fn is_on_curve(&self) -> Choice;
}

pub trait SmallFactor {
    fn is_torsion_free(&self) -> Choice;
}

pub struct CurveGenerator;

pub trait Encoding<A>: Sized {
    fn encode(&self) -> A;
}

pub trait Decode {
    fn decode(bytes: &[u8]) -> Option<Self>;
}

pub trait Canonical {
    fn is_canonical(&self) -> Choice;
    fn reduce(&self) -> Self;
}

pub struct Error;

pub trait ByteArray:
    AsRef<[u8]> + AsMut<[u8]> + Clone + Send + Sync + Sized + Eq + Ord + Hash + Debug
{
    const SIZE: usize;

    /// New byte array of zeroes
    ///
    /// Alternative to [`Default`] that is not implemented for generic `[T; N]`
    /// (see [tracking issue](https://github.com/rust-lang/rust/issues/61415))
    fn zeroes() -> Self;
}

impl<const N: usize> ByteArray for [u8; N] {
    const SIZE: usize = N;

    fn zeroes() -> Self {
        [0; N]
    }
}

impl<N: ArrayLength<u8>> ByteArray for GenericArray<u8, N> {
    const SIZE: usize = N::USIZE;

    fn zeroes() -> Self {
        GenericArray::default()
    }
}
