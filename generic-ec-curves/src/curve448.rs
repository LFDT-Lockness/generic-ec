//! Curve448 (Goldilocks) curve

use generic_array::GenericArray;

/// Pads a 56-byte LE scalar to 57 bytes (ScalarBytes format)
fn pad_to_57(bytes: &[u8; 56]) -> [u8; 57] {
    let mut padded = [0u8; 57];
    padded[..56].copy_from_slice(bytes);
    padded
}

/// Pads a 112-byte LE scalar to 114 bytes (WideScalarBytes format)
fn pad_to_114(bytes: &[u8; 112]) -> [u8; 114] {
    let mut padded = [0u8; 114];
    padded[..112].copy_from_slice(bytes);
    padded
}

/// Curve448 (Goldilocks) curve
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct Curve448 {
    _private: (),
}

impl generic_ec_core::Curve for Curve448 {
    const CURVE_NAME: &'static str = "curve448";

    type Point = Point;
    type Scalar = Scalar;

    type CompressedPointArray = <Point as generic_ec_core::CompressedEncoding>::Bytes;
    type UncompressedPointArray = <Point as generic_ec_core::UncompressedEncoding>::Bytes;

    type ScalarArray = <Scalar as generic_ec_core::IntegerEncoding>::Bytes;

    // We don't expose affine coordinates for curve448
    type CoordinateArray = [u8; 0];
}

// --- Point ---

/// Curve448 point
#[derive(Clone, Copy, PartialEq, Eq)]
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
        <ed448_goldilocks_plus::EdwardsPoint as group::cofactor::CofactorGroup>::is_torsion_free(
            &self.0,
        )
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
        let ga = self.0.to_bytes();
        let mut bytes = [0u8; 57];
        bytes.copy_from_slice(ga.as_ref());
        bytes
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

impl core::fmt::Debug for Point {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use generic_ec_core::CompressedEncoding;
        f.debug_tuple("Point")
            .field(&self.to_bytes_compressed())
            .finish()
    }
}

impl zeroize::Zeroize for Point {
    fn zeroize(&mut self) {
        // Replace with identity point
        self.0 = group::Group::identity();
    }
}

// --- Scalar ---

/// Curve448 scalar
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
    /// 72 bytes
    ///
    /// `L = ceil((ceil(log2(q)) + k) / 8) = ceil((446 + 128) / 8) = 72` bytes are enough to
    /// guarantee the uniform distribution (RFC 9380)
    type Bytes = [u8; 72];

    fn from_uniform_bytes(bytes: &Self::Bytes) -> Self {
        let mut bytes_le = [0u8; 114];
        bytes_le[..72].copy_from_slice(bytes);
        Self(ed448_goldilocks_plus::Scalar::from_bytes_mod_order_wide(
            GenericArray::from_slice(&bytes_le),
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

        Self(ed448_goldilocks_plus::Scalar::random(&mut FakeCryptoRng(rng)))
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
    type Bytes = [u8; 56];

    fn to_be_bytes(&self) -> Self::Bytes {
        let mut bytes = self.to_le_bytes();
        bytes.reverse();
        bytes
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        self.0.to_bytes()
    }

    fn from_be_bytes_exact(bytes: &Self::Bytes) -> Option<Self> {
        let mut bytes = *bytes;
        bytes.reverse();
        Self::from_le_bytes_exact(&bytes)
    }

    fn from_le_bytes_exact(bytes: &Self::Bytes) -> Option<Self> {
        let padded = pad_to_57(bytes);
        Option::from(ed448_goldilocks_plus::Scalar::from_canonical_bytes(
            GenericArray::from_slice(&padded),
        ))
        .map(Self)
    }

    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_be_bytes_mod_order_reducing::<_, 56>(bytes, &Self::ONE)
    }

    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_le_bytes_mod_order_reducing::<_, 56>(bytes, &Self::ONE)
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

impl core::fmt::Debug for Scalar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use generic_ec_core::IntegerEncoding;
        f.debug_tuple("Scalar").field(&self.to_le_bytes()).finish()
    }
}

impl zeroize::Zeroize for Scalar {
    fn zeroize(&mut self) {
        self.0 = ed448_goldilocks_plus::Scalar::ZERO;
    }
}

impl generic_ec_core::Reduce<56> for Scalar {
    fn from_be_array_mod_order(bytes: &[u8; 56]) -> Self {
        let mut bytes = *bytes;
        bytes.reverse();
        let padded = pad_to_57(&bytes);
        Self(ed448_goldilocks_plus::Scalar::from_bytes_mod_order(
            GenericArray::from_slice(&padded),
        ))
    }
    fn from_le_array_mod_order(bytes: &[u8; 56]) -> Self {
        let padded = pad_to_57(bytes);
        Self(ed448_goldilocks_plus::Scalar::from_bytes_mod_order(
            GenericArray::from_slice(&padded),
        ))
    }
}

impl generic_ec_core::Reduce<112> for Scalar {
    fn from_be_array_mod_order(bytes: &[u8; 112]) -> Self {
        let mut bytes = *bytes;
        bytes.reverse();
        let padded = pad_to_114(&bytes);
        Self(ed448_goldilocks_plus::Scalar::from_bytes_mod_order_wide(
            GenericArray::from_slice(&padded),
        ))
    }
    fn from_le_array_mod_order(bytes: &[u8; 112]) -> Self {
        let padded = pad_to_114(bytes);
        Self(ed448_goldilocks_plus::Scalar::from_bytes_mod_order_wide(
            GenericArray::from_slice(&padded),
        ))
    }
}
