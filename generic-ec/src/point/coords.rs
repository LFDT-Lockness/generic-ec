use crate::as_raw::{AsRaw, TryFromRaw};
use crate::coords::*;
use crate::core::coords as coords_core;
use crate::core::*;

use super::definition::Point;

impl<E: Curve> HasAffineX<E> for Point<E>
where
    E: coords_core::HasAffineX,
{
    fn x(&self) -> Option<Coordinate<E>> {
        E::x(self.as_raw()).map(Coordinate::new)
    }
}

impl<E: Curve> HasAffineXAndParity<E> for Point<E>
where
    E: coords_core::HasAffineXAndParity,
{
    fn x_and_parity(&self) -> Option<(Coordinate<E>, Parity)> {
        E::x_and_parity(self.as_raw()).map(|(x, p)| (Coordinate::new(x), p))
    }

    fn from_x_and_parity(x: &Coordinate<E>, y_parity: Parity) -> Option<Self> {
        E::from_x_and_parity(x.as_array(), y_parity).and_then(Self::try_from_raw)
    }
}

impl<E: Curve> HasAffineY<E> for Point<E>
where
    E: coords_core::HasAffineY,
{
    fn y(&self) -> Option<Coordinate<E>> {
        E::y(self.as_raw()).map(Coordinate::new)
    }
}

impl<E: Curve> HasAffineXY<E> for Point<E>
where
    E: coords_core::HasAffineXY,
{
    fn coords(&self) -> Option<Coordinates<E>> {
        let (x, y) = E::x_and_y(self.as_raw())?;
        Some(Coordinates {
            x: Coordinate::new(x),
            y: Coordinate::new(y),
        })
    }

    fn from_coords(coords: &Coordinates<E>) -> Option<Self> {
        E::from_x_and_y(coords.x.as_array(), coords.y.as_array()).and_then(Self::try_from_raw)
    }
}

impl<E: Curve> AlwaysHasAffineY<E> for Point<E>
where
    E: coords_core::AlwaysHasAffineY,
{
    fn y(&self) -> Coordinate<E> {
        Coordinate::new(E::y(self.as_raw()))
    }
}

impl<E: Curve> AlwaysHasAffineYAndSign<E> for Point<E>
where
    E: coords_core::AlwaysHasAffineYAndSign,
{
    fn y_and_sign(&self) -> (Sign, Coordinate<E>) {
        let (sign, coord) = E::y_and_sign(self.as_raw());
        (sign, Coordinate::new(coord))
    }

    fn from_y_and_sign(x_sign: Sign, y: &Coordinate<E>) -> Option<Self> {
        E::from_y_and_sign(x_sign, y.as_array()).and_then(Point::try_from_raw)
    }
}
