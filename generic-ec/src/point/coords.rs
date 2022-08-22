use subtle::CtOption;

use crate::as_raw::{AsRaw, FromRaw};
use crate::coords::*;
use crate::ec_core::coords as coords_core;
use crate::ec_core::*;

use super::definition::Point;

impl<E: Curve> HasAffineX<E> for Point<E>
where
    E: coords_core::HasAffineX,
{
    fn x(&self) -> Option<Coordinate<E>> {
        self.ct_x().into()
    }
    fn ct_x(&self) -> CtOption<Coordinate<E>> {
        let (is_infinity, coord) = E::x(&self.as_raw());
        CtOption::new(Coordinate::new(coord), !is_infinity)
    }
}

impl<E: Curve> HasAffineXAndParity<E> for Point<E>
where
    E: coords_core::HasAffineXAndParity,
{
    fn x_and_parity(&self) -> Option<(Parity, Coordinate<E>)> {
        self.ct_x_and_parity().into()
    }

    fn from_x_and_parity(x: Coordinate<E>, y_parity: Parity) -> Option<Self> {
        Self::ct_from_x_and_parity(x, y_parity).into()
    }

    fn ct_x_and_parity(&self) -> CtOption<(Parity, Coordinate<E>)> {
        let (is_infinity, parity, coord) = E::x_and_parity(&self.as_raw());
        CtOption::new((parity, Coordinate::new(coord)), !is_infinity)
    }

    fn ct_from_x_and_parity(x: Coordinate<E>, y_parity: Parity) -> CtOption<Self> {
        E::from_x_and_parity(x.as_array(), y_parity).and_then(Point::ct_from_raw)
    }
}

impl<E: Curve> HasAffineY<E> for Point<E>
where
    E: coords_core::HasAffineY,
{
    fn y(&self) -> Option<Coordinate<E>> {
        self.ct_y().into()
    }

    fn ct_y(&self) -> CtOption<Coordinate<E>> {
        let (is_infinity, coord) = E::y(&self.as_raw());
        CtOption::new(Coordinate::new(coord), !is_infinity)
    }
}

impl<E: Curve> HasAffineXY<E> for Point<E>
where
    E: coords_core::HasAffineXY,
{
    fn coords(&self) -> Option<Coordinates<E>> {
        self.ct_coords().into()
    }

    fn from_coords(coords: &Coordinates<E>) -> Option<Self> {
        Self::ct_from_coords(coords).into()
    }

    fn ct_coords(&self) -> CtOption<Coordinates<E>> {
        let (is_infinity, x, y) = E::x_and_y(&self.as_raw());
        CtOption::new(
            Coordinates {
                x: Coordinate::new(x),
                y: Coordinate::new(y),
            },
            !is_infinity,
        )
    }

    fn ct_from_coords(coords: &Coordinates<E>) -> CtOption<Self> {
        E::from_x_and_y(coords.x.as_array(), coords.y.as_array()).and_then(Point::ct_from_raw)
    }
}

impl<E: Curve> AlwaysHasAffineY<E> for Point<E>
where
    E: coords_core::AlwaysHasAffineY,
{
    fn y(&self) -> Coordinate<E> {
        Coordinate::new(E::y(&self.as_raw()))
    }
}

impl<E: Curve> AlwaysHasAffineYAndSign<E> for Point<E>
where
    E: coords_core::AlwaysHasAffineYAndSign,
{
    fn y_and_sign(&self) -> (Sign, Coordinate<E>) {
        let (sign, coord) = E::y_and_sign(&self.as_raw());
        (sign, Coordinate::new(coord))
    }

    fn from_y_and_sign(x_sign: Sign, y: &Coordinate<E>) -> Option<Self> {
        E::from_y_and_sign(x_sign, &y.as_array()).and_then(|point| Point::from_raw(point).into())
    }
}
