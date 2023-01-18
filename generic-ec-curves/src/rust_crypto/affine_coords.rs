use elliptic_curve::generic_array::GenericArray;
use elliptic_curve::sec1::{
    CompressedPointSize, Coordinates, EncodedPoint, FromEncodedPoint, Tag, ToEncodedPoint,
};
use elliptic_curve::{AffineArithmetic, FieldSize, ProjectiveArithmetic};
use generic_ec_core::coords::{HasAffineX, HasAffineXAndParity, HasAffineXY, HasAffineY, Parity};

use super::{RustCryptoCurve, RustCryptoPoint};

impl<C, X> HasAffineX for RustCryptoCurve<C, X>
where
    C: ProjectiveArithmetic + AffineArithmetic,
    FieldSize<C>: elliptic_curve::sec1::ModulusSize,
    C::AffinePoint: ToEncodedPoint<C> + From<C::ProjectivePoint>,
    RustCryptoCurve<C, X>: generic_ec_core::Curve<
        Point = RustCryptoPoint<C>,
        CoordinateArray = elliptic_curve::FieldBytes<C>,
    >,
{
    fn x(point: &Self::Point) -> Option<Self::CoordinateArray> {
        match C::AffinePoint::from(point.0)
            .to_encoded_point(false)
            .coordinates()
        {
            Coordinates::Identity => None,
            Coordinates::Uncompressed { x, .. } => Some(x.clone()),
            Coordinates::Compact { .. } | Coordinates::Compressed { .. } => {
                unreachable!("point was encoded in uncompressed form")
            }
        }
    }
}

impl<C, X> HasAffineXAndParity for RustCryptoCurve<C, X>
where
    C: ProjectiveArithmetic + AffineArithmetic,
    FieldSize<C>: elliptic_curve::sec1::ModulusSize,
    C::AffinePoint: ToEncodedPoint<C>
        + FromEncodedPoint<C>
        + From<C::ProjectivePoint>
        + Into<C::ProjectivePoint>,
    RustCryptoCurve<C, X>: generic_ec_core::Curve<
        Point = RustCryptoPoint<C>,
        CoordinateArray = elliptic_curve::FieldBytes<C>,
    >,
{
    fn x_and_parity(point: &Self::Point) -> Option<(Self::CoordinateArray, Parity)> {
        match C::AffinePoint::from(point.0)
            .to_encoded_point(true)
            .coordinates()
        {
            Coordinates::Identity => None,
            Coordinates::Compressed { x, y_is_odd } => {
                Some((x.clone(), if y_is_odd { Parity::Odd } else { Parity::Even }))
            }
            Coordinates::Compact { .. } | Coordinates::Uncompressed { .. } => {
                unreachable!("point was encoded in uncompressed form")
            }
        }
    }

    fn from_x_and_parity(x: &Self::CoordinateArray, y_parity: Parity) -> Option<Self::Point> {
        let mut encoding = GenericArray::<u8, CompressedPointSize<C>>::default();
        let tag = match y_parity {
            Parity::Even => Tag::CompressedEvenY,
            Parity::Odd => Tag::CompressedOddY,
        };
        encoding[0] = tag as u8;
        encoding[1..].copy_from_slice(x);

        let encoded_point = EncodedPoint::<C>::from_bytes(&encoding).ok()?;
        Option::from(C::AffinePoint::from_encoded_point(&encoded_point))
            .map(|point: C::AffinePoint| RustCryptoPoint(point.into()))
    }
}

impl<C, X> HasAffineY for RustCryptoCurve<C, X>
where
    C: ProjectiveArithmetic + AffineArithmetic,
    FieldSize<C>: elliptic_curve::sec1::ModulusSize,
    C::AffinePoint: ToEncodedPoint<C> + From<C::ProjectivePoint>,
    RustCryptoCurve<C, X>: generic_ec_core::Curve<
        Point = RustCryptoPoint<C>,
        CoordinateArray = elliptic_curve::FieldBytes<C>,
    >,
{
    fn y(point: &Self::Point) -> Option<Self::CoordinateArray> {
        match C::AffinePoint::from(point.0)
            .to_encoded_point(false)
            .coordinates()
        {
            Coordinates::Identity => None,
            Coordinates::Uncompressed { y, .. } => Some(y.clone()),
            Coordinates::Compact { .. } | Coordinates::Compressed { .. } => {
                unreachable!("point was encoded in uncompressed form")
            }
        }
    }
}

impl<C, X> HasAffineXY for RustCryptoCurve<C, X>
where
    C: ProjectiveArithmetic + AffineArithmetic,
    FieldSize<C>: elliptic_curve::sec1::ModulusSize,
    C::AffinePoint: ToEncodedPoint<C>
        + FromEncodedPoint<C>
        + From<C::ProjectivePoint>
        + Into<C::ProjectivePoint>,
    RustCryptoCurve<C, X>: generic_ec_core::Curve<
        Point = RustCryptoPoint<C>,
        CoordinateArray = elliptic_curve::FieldBytes<C>,
    >,
{
    fn x_and_y(point: &Self::Point) -> Option<(Self::CoordinateArray, Self::CoordinateArray)> {
        match C::AffinePoint::from(point.0)
            .to_encoded_point(false)
            .coordinates()
        {
            Coordinates::Identity => None,
            Coordinates::Uncompressed { x, y } => Some((x.clone(), y.clone())),
            Coordinates::Compact { .. } | Coordinates::Compressed { .. } => {
                unreachable!("point was encoded in uncompressed form")
            }
        }
    }

    fn from_x_and_y(x: &Self::CoordinateArray, y: &Self::CoordinateArray) -> Option<Self::Point> {
        let encoded_point = EncodedPoint::<C>::from_affine_coordinates(x, y, false);
        Option::from(C::AffinePoint::from_encoded_point(&encoded_point))
            .map(|point: C::AffinePoint| RustCryptoPoint(point.into()))
    }
}
