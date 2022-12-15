use crate::coords::{
    AlwaysHasAffineX, AlwaysHasAffineXY, AlwaysHasAffineY, HasAffineX, HasAffineXY, HasAffineY,
};
use crate::{Curve, Point};

use super::definition::NonZero;

impl<E: Curve> AlwaysHasAffineX<E> for NonZero<Point<E>>
where
    Point<E>: HasAffineX<E>,
{
    fn x(&self) -> crate::coords::Coordinate<E> {
        HasAffineX::x(&**self).expect("non-zero point always has coordinates")
    }
}

impl<E: Curve> AlwaysHasAffineY<E> for NonZero<Point<E>>
where
    Point<E>: HasAffineY<E>,
{
    fn y(&self) -> crate::coords::Coordinate<E> {
        HasAffineY::y(&**self).expect("non-zero point always has coordinates")
    }
}

impl<E: Curve> AlwaysHasAffineXY<E> for NonZero<Point<E>>
where
    Point<E>: HasAffineXY<E>,
{
    fn from_coords(coords: &crate::coords::Coordinates<E>) -> Option<Self> {
        <Point<E> as HasAffineXY<E>>::from_coords(coords).and_then(NonZero::from_point)
    }
}
