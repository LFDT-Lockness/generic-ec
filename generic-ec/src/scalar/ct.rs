use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

use crate::Curve;

use super::definition::Scalar;

impl<E: Curve> ConditionallySelectable for Scalar<E> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        // Correctness: both `a` and `b` have to be valid points by construction
        Scalar::from_raw_unchecked(<E::Scalar as ConditionallySelectable>::conditional_select(
            &a.as_raw(),
            &b.as_raw(),
            choice,
        ))
    }
}

impl<E: Curve> ConstantTimeEq for Scalar<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_raw().ct_eq(other.as_raw())
    }
}
