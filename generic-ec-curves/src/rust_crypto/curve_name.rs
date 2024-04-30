/// Name of the curve
pub trait CurveName {
    /// Name of the curve
    const CURVE_NAME: &'static str;
}

#[cfg(feature = "secp256r1")]
impl CurveName for p256::NistP256 {
    const CURVE_NAME: &'static str = "secp256r1";
}

#[cfg(feature = "secp256k1")]
impl CurveName for k256::Secp256k1 {
    const CURVE_NAME: &'static str = "secp256k1";
}

#[cfg(feature = "stark")]
impl CurveName for stark_curve::StarkCurve {
    const CURVE_NAME: &'static str = "stark";
}
