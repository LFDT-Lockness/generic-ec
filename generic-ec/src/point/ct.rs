use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

use crate::Curve;

use super::definition::Point;

impl<E: Curve> ConditionallySelectable for Point<E> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        // Correctness: both `a` and `b` have to be valid points by construction
        Point::from_raw_unchecked(<E::Point as ConditionallySelectable>::conditional_select(
            &a.as_raw(),
            &b.as_raw(),
            choice,
        ))
    }
}

impl<E: Curve> ConstantTimeEq for Point<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_raw().ct_eq(other.as_raw())
    }
}
