use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};
use zeroize::Zeroize;

use crate::ec_core::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct DummyCurve;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DummyPoint([u8; 32]);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DummyScalar([u8; 32]);

impl Curve for DummyCurve {
    type Point = DummyPoint;

    type Scalar = DummyScalar;

    type CompressedPointSize = generic_array::typenum::U32;

    type UncompressedPointSize = generic_array::typenum::U32;

    type CoordinateSize = generic_array::typenum::U32;
}

impl Additive for DummyPoint {
    fn add(_a: &Self, _b: &Self) -> Self {
        todo!()
    }
    fn add_assign(_a: &mut Self, _b: &Self) {
        todo!()
    }
    fn sub(_a: &Self, _b: &Self) -> Self {
        todo!()
    }
    fn sub_assign(_a: &mut Self, _b: &Self) {
        todo!()
    }
    fn negate(_x: &Self) -> Self {
        todo!()
    }
}

impl<T> Multiplicative<T> for DummyPoint {
    fn mul(_a: &Self, _b: &T) -> Self {
        todo!()
    }
    fn mul_assign(_a: &mut Self, _b: &T) {
        todo!()
    }
}

impl Additive for DummyScalar {
    fn add(_a: &Self, _b: &Self) -> Self {
        todo!()
    }
    fn add_assign(_a: &mut Self, _b: &Self) {
        todo!()
    }
    fn sub(_a: &Self, _b: &Self) -> Self {
        todo!()
    }
    fn sub_assign(_a: &mut Self, _b: &Self) {
        todo!()
    }
    fn negate(_x: &Self) -> Self {
        todo!()
    }
}

impl Multiplicative for DummyScalar {
    fn mul(_a: &Self, _b: &Self) -> Self {
        todo!()
    }
    fn mul_assign(_a: &mut Self, _b: &Self) {
        todo!()
    }
}

impl Zeroize for DummyPoint {
    fn zeroize(&mut self) {
        todo!()
    }
}

impl Zeroize for DummyScalar {
    fn zeroize(&mut self) {
        todo!()
    }
}

impl Zero for DummyPoint {
    fn zero() -> Self {
        todo!()
    }
    fn is_zero(_x: &Self) -> subtle::Choice {
        todo!()
    }
}

impl Zero for DummyScalar {
    fn zero() -> Self {
        todo!()
    }
    fn is_zero(_x: &Self) -> subtle::Choice {
        todo!()
    }
}

impl From<CurveGenerator> for DummyPoint {
    fn from(_: CurveGenerator) -> Self {
        todo!()
    }
}

impl SmallFactor for DummyPoint {
    fn is_torsion_free(&self) -> Choice {
        todo!()
    }
}

impl ConstantTimeEq for DummyPoint {
    fn ct_eq(&self, _other: &Self) -> subtle::Choice {
        todo!()
    }
}

impl ConstantTimeEq for DummyScalar {
    fn ct_eq(&self, _other: &Self) -> subtle::Choice {
        todo!()
    }
}

impl Invertible for DummyScalar {
    fn invert(_x: &Self) -> subtle::CtOption<Self> {
        todo!()
    }
}

impl Samplable for DummyScalar {
    fn random<R: rand_core::RngCore + rand_core::CryptoRng>(_rng: &mut R) -> Self {
        todo!()
    }
}

impl One for DummyScalar {
    fn one() -> Self {
        todo!()
    }
    fn is_one(_x: &Self) -> subtle::Choice {
        todo!()
    }
}

impl ConditionallySelectable for DummyPoint {
    fn conditional_select(_a: &Self, _b: &Self, _choice: subtle::Choice) -> Self {
        todo!()
    }
}

impl ConditionallySelectable for DummyScalar {
    fn conditional_select(_a: &Self, _b: &Self, _choice: subtle::Choice) -> Self {
        todo!()
    }
}

impl Default for DummyPoint {
    fn default() -> Self {
        todo!()
    }
}

impl Default for DummyScalar {
    fn default() -> Self {
        todo!()
    }
}
