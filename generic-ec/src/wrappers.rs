use subtle::{ConditionallySelectable, CtOption};
use zeroize::Zeroizing;

use crate::ec_core::coords as coords_core;
use crate::ec_core::*;
use crate::{coords::*, ec_core::Curve};

#[derive(Copy, Clone, Default)]
pub struct Point<E: Curve>(E::Point);

impl<E: Curve> Point<E> {
    pub fn from_raw(point: E::Point) -> CtOption<Self> {
        let is_torison_free = point.is_torsion_free();
        CtOption::new(Point(point), is_torison_free)
    }
}

impl<E: Curve> HasAffineX<E> for Point<E>
where
    E: coords_core::HasAffineX,
{
    fn x(&self) -> CtOption<Coordinate<E>> {
        let (is_infinity, coord) = E::x(&self.0);
        CtOption::new(Coordinate::new(coord), !is_infinity)
    }
}

impl<E: Curve> HasAffineXAndParity<E> for Point<E>
where
    E: coords_core::HasAffineXAndParity,
{
    fn x_and_parity(&self) -> CtOption<(Parity, Coordinate<E>)> {
        let (is_infinity, parity, coord) = E::x_and_parity(&self.0);
        CtOption::new((parity, Coordinate::new(coord)), !is_infinity)
    }

    fn from_x_and_parity(x: Coordinate<E>, y_parity: Parity) -> CtOption<Self> {
        E::from_x_and_parity(x.as_array(), y_parity).and_then(Point::from_raw)
    }
}

impl<E: Curve> HasAffineY<E> for Point<E>
where
    E: coords_core::HasAffineY,
{
    fn y(&self) -> CtOption<Coordinate<E>> {
        let (is_infinity, coord) = E::y(&self.0);
        CtOption::new(Coordinate::new(coord), !is_infinity)
    }
}

impl<E: Curve> HasAffineXY<E> for Point<E>
where
    E: coords_core::HasAffineXY,
{
    fn coords(&self) -> CtOption<Coordinates<E>> {
        let (is_infinity, x, y) = E::x_and_y(&self.0);
        CtOption::new(
            Coordinates {
                x: Coordinate::new(x),
                y: Coordinate::new(y),
            },
            !is_infinity,
        )
    }

    fn from_coords(coords: &Coordinates<E>) -> CtOption<Self> {
        E::from_x_and_y(coords.x.as_array(), coords.y.as_array()).and_then(Point::from_raw)
    }
}

impl<E: Curve> AlwaysHasAffineY<E> for Point<E>
where
    E: coords_core::AlwaysHasAffineY,
{
    fn y(&self) -> Coordinate<E> {
        Coordinate::new(E::y(&self.0))
    }
}

impl<E: Curve> AlwaysHasAffineYAndSign<E> for Point<E>
where
    E: coords_core::AlwaysHasAffineYAndSign,
{
    fn y_and_sign(&self) -> (Sign, Coordinate<E>) {
        let (sign, coord) = E::y_and_sign(&self.0);
        (sign, Coordinate::new(coord))
    }

    fn from_y_and_sign(x_sign: Sign, y: &Coordinate<E>) -> Option<Self> {
        E::from_y_and_sign(x_sign, &y.as_array()).and_then(|point| Point::from_raw(point).into())
    }
}

impl<E: Curve> ConditionallySelectable for Point<E> {
    fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
        Point(<E::Point as ConditionallySelectable>::conditional_select(
            &a.0, &b.0, choice,
        ))
    }
}

pub struct Scalar<E: Curve>(E::Scalar);

#[cfg(feature = "std")]
pub struct SecretScalar<E: Curve>(Box<Zeroizing<E::Scalar>>);

pub struct NonZero<T>(T);
