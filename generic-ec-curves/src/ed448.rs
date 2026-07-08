//! Ed448 (Goldilocks) curve

use generic_array::GenericArray;

/// Ed448 (Goldilocks) curve
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default, zeroize::Zeroize)]
pub struct Ed448 {
    _private: (),
}

impl generic_ec_core::Curve for Ed448 {
    const CURVE_NAME: &'static str = "ed448";

    type Point = Point;
    type Scalar = Scalar;

    type CompressedPointArray = <Point as generic_ec_core::CompressedEncoding>::Bytes;
    type UncompressedPointArray = <Point as generic_ec_core::UncompressedEncoding>::Bytes;

    type ScalarArray = <Scalar as generic_ec_core::IntegerEncoding>::Bytes;

    // We don't expose affine coordinates for ed448
    type CoordinateArray = [u8; 0];
}

// --- Point ---

/// Ed448 point
#[derive(Clone, Copy, PartialEq, Eq, zeroize::Zeroize)]
#[repr(transparent)]
pub struct Point(pub ed448_goldilocks_plus::EdwardsPoint);

impl generic_ec_core::Additive for Point {
    #[inline]
    fn add(a: &Self, b: &Self) -> Self {
        Self(a.0 + b.0)
    }

    #[inline]
    fn sub(a: &Self, b: &Self) -> Self {
        Self(a.0 - b.0)
    }

    #[inline]
    fn negate(x: &Self) -> Self {
        Self(-x.0)
    }
}

impl From<generic_ec_core::CurveGenerator> for Point {
    #[inline]
    fn from(_: generic_ec_core::CurveGenerator) -> Self {
        Self(group::Group::generator())
    }
}

impl generic_ec_core::Zero for Point {
    fn zero() -> Self {
        Self(group::Group::identity())
    }

    fn is_zero(x: &Self) -> subtle::Choice {
        subtle::ConstantTimeEq::ct_eq(x, &Self::zero())
    }
}

impl generic_ec_core::OnCurve for Point {
    #[inline]
    fn is_on_curve(&self) -> subtle::Choice {
        // All EdwardsPoint values produced by this crate are on the curve
        subtle::Choice::from(1)
    }
}

impl generic_ec_core::SmallFactor for Point {
    #[inline]
    fn is_torsion_free(&self) -> subtle::Choice {
        self.0.is_torsion_free()
    }
}

impl subtle::ConstantTimeEq for Point {
    #[inline]
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        self.0.ct_eq(&other.0)
    }
}

impl subtle::ConditionallySelectable for Point {
    #[inline]
    fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
        Self(subtle::ConditionallySelectable::conditional_select(
            &a.0, &b.0, choice,
        ))
    }
}

impl generic_ec_core::CompressedEncoding for Point {
    type Bytes = [u8; 57];

    fn to_bytes_compressed(&self) -> Self::Bytes {
        use group::GroupEncoding;
        self.0.to_bytes().into()
    }
}

impl generic_ec_core::UncompressedEncoding for Point {
    type Bytes = <Self as generic_ec_core::CompressedEncoding>::Bytes;

    fn to_bytes_uncompressed(&self) -> Self::Bytes {
        <Self as generic_ec_core::CompressedEncoding>::to_bytes_compressed(self)
    }
}

impl generic_ec_core::Decode for Point {
    fn decode(bytes: &[u8]) -> Option<Self> {
        use group::GroupEncoding;
        if bytes.len() != 57 {
            return None;
        }
        let ga = GenericArray::from_slice(bytes);
        Option::from(ed448_goldilocks_plus::EdwardsPoint::from_bytes(ga)).map(Self)
    }
}

impl core::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        use generic_ec_core::CompressedEncoding;
        self.to_bytes_compressed().cmp(&other.to_bytes_compressed())
    }
}

impl core::hash::Hash for Point {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        use generic_ec_core::CompressedEncoding;
        self.to_bytes_compressed().hash(state)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self(group::Group::identity())
    }
}

// --- Scalar ---

/// Ed448 scalar
#[derive(Default, Clone, Copy, PartialEq, Eq, zeroize::Zeroize)]
pub struct Scalar(pub ed448_goldilocks_plus::Scalar);

impl Scalar {
    /// Scalar equal to 1
    pub const ONE: Self = Self(ed448_goldilocks_plus::Scalar::ONE);
    /// Scalar equal to 0
    pub const ZERO: Self = Self(ed448_goldilocks_plus::Scalar::ZERO);
}

impl generic_ec_core::Additive for Scalar {
    #[inline]
    fn add(a: &Self, b: &Self) -> Self {
        Self(a.0 + b.0)
    }

    #[inline]
    fn sub(a: &Self, b: &Self) -> Self {
        Self(a.0 - b.0)
    }

    #[inline]
    fn negate(x: &Self) -> Self {
        Self(-x.0)
    }
}

impl generic_ec_core::Multiplicative<Scalar> for Scalar {
    type Output = Scalar;

    #[inline]
    fn mul(a: &Self, b: &Scalar) -> Self::Output {
        Self(a.0 * b.0)
    }
}

impl generic_ec_core::Multiplicative<Point> for Scalar {
    type Output = Point;
    #[inline]
    fn mul(a: &Self, b: &Point) -> Self::Output {
        Point(a.0 * b.0)
    }
}

impl generic_ec_core::Multiplicative<generic_ec_core::CurveGenerator> for Scalar {
    type Output = Point;

    #[inline]
    fn mul(a: &Self, _: &generic_ec_core::CurveGenerator) -> Self::Output {
        Point(a.0 * <ed448_goldilocks_plus::EdwardsPoint as group::Group>::generator())
    }
}

impl generic_ec_core::Invertible for Scalar {
    fn invert(x: &Self) -> subtle::CtOption<Self> {
        subtle::CtOption::new(Self(x.0.invert()), !generic_ec_core::Zero::is_zero(x))
    }
}

impl generic_ec_core::Zero for Scalar {
    fn zero() -> Self {
        Self(ed448_goldilocks_plus::Scalar::ZERO)
    }

    fn is_zero(x: &Self) -> subtle::Choice {
        subtle::ConstantTimeEq::ct_eq(&x.0, &ed448_goldilocks_plus::Scalar::ZERO)
    }
}

impl generic_ec_core::One for Scalar {
    fn one() -> Self {
        Self(ed448_goldilocks_plus::Scalar::ONE)
    }

    fn is_one(x: &Self) -> subtle::Choice {
        subtle::ConstantTimeEq::ct_eq(&x.0, &ed448_goldilocks_plus::Scalar::ONE)
    }
}

impl generic_ec_core::FromUniformBytes for Scalar {
    /// 84 bytes
    ///
    /// `L = ceil((ceil(log2(q)) + k) / 8) = ceil((446 + 224) / 8) = 84` bytes are enough to
    /// guarantee the uniform distribution (RFC 9380)
    type Bytes = [u8; 84];

    fn from_uniform_bytes(bytes: &Self::Bytes) -> Self {
        let mut wide = GenericArray::default();
        wide[..84].copy_from_slice(bytes);
        Self(ed448_goldilocks_plus::Scalar::from_bytes_mod_order_wide(
            &wide,
        ))
    }
}

impl generic_ec_core::SamplableVartime for Scalar {
    fn random_vartime(rng: &mut impl rand_core::RngCore) -> Self {
        // `ed448-goldilocks-plus` requires CryptoRng for scalar generation,
        // but it's not a hard requirement for our use case
        struct FakeCryptoRng<R>(R);
        impl<R: rand_core::RngCore> rand_core::RngCore for FakeCryptoRng<R> {
            fn next_u32(&mut self) -> u32 {
                self.0.next_u32()
            }
            fn next_u64(&mut self) -> u64 {
                self.0.next_u64()
            }
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                self.0.fill_bytes(dest)
            }
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
                self.0.try_fill_bytes(dest)
            }
        }
        impl<R> rand_core::CryptoRng for FakeCryptoRng<R> {}

        Self(ed448_goldilocks_plus::Scalar::random(&mut FakeCryptoRng(
            rng,
        )))
    }
}

impl subtle::ConstantTimeEq for Scalar {
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        self.0.ct_eq(&other.0)
    }
}

impl subtle::ConditionallySelectable for Scalar {
    fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
        Self(subtle::ConditionallySelectable::conditional_select(
            &a.0, &b.0, choice,
        ))
    }
}

impl generic_ec_core::IntegerEncoding for Scalar {
    type Bytes = [u8; 57];

    fn to_be_bytes(&self) -> Self::Bytes {
        let mut bytes = self.to_le_bytes();
        bytes.reverse();
        bytes
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        self.0.to_bytes_rfc_8032().into()
    }

    fn from_be_bytes_exact(bytes: &Self::Bytes) -> Option<Self> {
        let mut bytes = *bytes;
        bytes.reverse();
        Self::from_le_bytes_exact(&bytes)
    }

    fn from_le_bytes_exact(bytes: &Self::Bytes) -> Option<Self> {
        Option::from(ed448_goldilocks_plus::Scalar::from_canonical_bytes(
            bytes.into(),
        ))
        .map(Self)
    }

    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_be_bytes_mod_order_reducing::<_, 57>(bytes, &Self::ONE)
    }

    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_le_bytes_mod_order_reducing::<_, 57>(bytes, &Self::ONE)
    }
}

impl core::cmp::PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::Ord for Scalar {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        use generic_ec_core::IntegerEncoding;
        self.to_le_bytes().cmp(&other.to_le_bytes())
    }
}

impl generic_ec_core::Reduce<57> for Scalar {
    fn from_be_array_mod_order(bytes: &[u8; 57]) -> Self {
        let mut bytes = *bytes;
        bytes.reverse();
        Self::from_le_array_mod_order(&bytes)
    }
    fn from_le_array_mod_order(bytes: &[u8; 57]) -> Self {
        // `from_bytes_mod_order` only uses bytes[0..56] and ignores byte[56].
        // Use `from_bytes_mod_order_wide` with zero-padding so all 57 bytes
        // (= 456-bit LE integer) are correctly reduced mod the group order.
        let mut wide = GenericArray::default();
        wide[..57].copy_from_slice(bytes);
        Self(ed448_goldilocks_plus::Scalar::from_bytes_mod_order_wide(
            &wide,
        ))
    }
}

impl generic_ec_core::Reduce<114> for Scalar {
    fn from_be_array_mod_order(bytes: &[u8; 114]) -> Self {
        let mut bytes = *bytes;
        bytes.reverse();
        let mut wide = GenericArray::default();
        wide.copy_from_slice(&bytes);
        Self(ed448_goldilocks_plus::Scalar::from_bytes_mod_order_wide(
            &wide,
        ))
    }
    fn from_le_array_mod_order(bytes: &[u8; 114]) -> Self {
        let mut wide = GenericArray::default();
        wide.copy_from_slice(bytes);
        Self(ed448_goldilocks_plus::Scalar::from_bytes_mod_order_wide(
            &wide,
        ))
    }
}
