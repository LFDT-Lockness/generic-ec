use elliptic_curve::{
    group::cofactor::CofactorGroup,
    hash2curve::{ExpandMsg, FromOkm, GroupDigest},
    ProjectiveArithmetic,
};
use generic_ec_core::hash_to_curve::{HashToCurve, Tag};

use super::{RustCryptoCurve, RustCryptoPoint, RustCryptoScalar};

impl<C, X> HashToCurve for RustCryptoCurve<C, X>
where
    C: ProjectiveArithmetic + GroupDigest,
    C::ProjectivePoint: CofactorGroup,
    C::Scalar: FromOkm,
    for<'a> X: ExpandMsg<'a>,
    RustCryptoCurve<C, X>:
        generic_ec_core::Curve<Point = RustCryptoPoint<C>, Scalar = RustCryptoScalar<C>>,
{
    fn hash_to_curve(ctx: Tag, msgs: &[&[u8]]) -> Result<Self::Point, generic_ec_core::Error> {
        let point = <C as GroupDigest>::hash_from_bytes::<X>(msgs, ctx.as_bytes())
            .or(Err(generic_ec_core::Error))?;
        Ok(RustCryptoPoint(point))
    }

    fn hash_to_scalar(ctx: Tag, msgs: &[&[u8]]) -> Result<Self::Scalar, generic_ec_core::Error> {
        let scalar = <C as GroupDigest>::hash_to_scalar::<X>(msgs, ctx.as_bytes())
            .or(Err(generic_ec_core::Error))?;
        Ok(RustCryptoScalar(scalar))
    }
}
