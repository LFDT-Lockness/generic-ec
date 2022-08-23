use subtle::CtOption;
use zeroize::Zeroizing;

use crate::{
    coords::{Coordinate, HasAffineX},
    traits::Curve,
};

pub struct Point<E: Curve>(E::Point);

// impl<E: Curve + HasAffineX> Point<E> {
//     pub fn x_coord(&self) -> CtOption<Coordinate<E>> {
//         <E as HasAffineX>::x(&self.0).map(Coordinate)
//     }
// }

pub struct Scalar<E: Curve>(E::Scalar);

#[cfg(feature = "std")]
pub struct SecretScalar<E: Curve>(Box<Zeroizing<E::Scalar>>);

pub struct NonZero<T>(T);
