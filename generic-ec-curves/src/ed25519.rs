#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default, zeroize::Zeroize)]
pub struct Ed25519 {
    _private: (),
}

impl generic_ec_core::Curve for Ed25519 {
    const CURVE_NAME: &'static str = "ed25519";

    type Point = Point;
    type Scalar = Scalar;

    type CompressedPointArray = <Point as generic_ec_core::CompressedEncoding>::Bytes;
    type UncompressedPointArray = <Point as generic_ec_core::UncompressedEncoding>::Bytes;

    type ScalarArray = <Scalar as generic_ec_core::IntegerEncoding>::Bytes;

    // We don't expose affine coordinates for ed25519 curve
    type CoordinateArray = [u8; 0];
}

#[derive(Clone, Copy, PartialEq, Eq, zeroize::Zeroize)]
#[repr(transparent)]
pub struct Point(pub curve25519::EdwardsPoint);

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
        subtle::Choice::from(1)
    }
}

impl generic_ec_core::SmallFactor for Point {
    #[inline]
    fn is_torsion_free(&self) -> subtle::Choice {
        subtle::Choice::from(u8::from(self.0.is_torsion_free()))
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
    type Bytes = [u8; 32];

    fn to_bytes_compressed(&self) -> Self::Bytes {
        self.0.compress().to_bytes()
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
        let compressed = curve25519::edwards::CompressedEdwardsY::from_slice(bytes).ok()?;
        compressed.decompress().map(Self)
    }
}

impl core::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0
            .compress()
            .as_bytes()
            .cmp(other.0.compress().as_bytes())
    }
}

impl core::hash::Hash for Point {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.compress().as_bytes().hash(state)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self(group::Group::identity())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, zeroize::Zeroize)]
pub struct Scalar(pub curve25519::Scalar);

impl Scalar {
    /// Scalar equal to 1
    pub const ONE: Self = Self(curve25519::Scalar::ONE);
    /// Scalar equal to 0
    pub const ZERO: Self = Self(curve25519::Scalar::ZERO);
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
        use curve25519::traits::BasepointTable;
        Point(curve25519::constants::ED25519_BASEPOINT_TABLE.mul_base(&a.0))
    }
}

impl generic_ec_core::Invertible for Scalar {
    fn invert(x: &Self) -> subtle::CtOption<Self> {
        subtle::CtOption::new(Self(x.0.invert()), !generic_ec_core::Zero::is_zero(x))
    }
}

impl generic_ec_core::Zero for Scalar {
    fn zero() -> Self {
        Self(curve25519::Scalar::ZERO)
    }

    fn is_zero(x: &Self) -> subtle::Choice {
        subtle::ConstantTimeEq::ct_eq(&x.0, &curve25519::Scalar::ZERO)
    }
}

impl generic_ec_core::One for Scalar {
    fn one() -> Self {
        Self(curve25519::Scalar::ONE)
    }

    fn is_one(x: &Self) -> subtle::Choice {
        subtle::ConstantTimeEq::ct_eq(&x.0, &curve25519::Scalar::ONE)
    }
}

impl generic_ec_core::Samplable for Scalar {
    fn random<R: rand_core::RngCore>(rng: &mut R) -> Self {
        // Having crypto rng for scalar generation is not a hard requirement,
        // as in some cases it isn't needed. However, `curve25519` lib asks for
        // it, so we'll trick it
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

        Self(curve25519::Scalar::random(&mut FakeCryptoRng(rng)))
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
    type Bytes = [u8; 32];

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
        Option::from(curve25519::Scalar::from_canonical_bytes(*bytes)).map(Self)
    }

    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_be_bytes_mod_order_reducing_32_64(bytes, &Self::ONE)
    }

    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        crate::utils::scalar_from_le_bytes_mod_order_reducing_32_64(bytes, &Self::ONE)
    }
}

impl core::cmp::PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::Ord for Scalar {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.as_bytes().cmp(other.0.as_bytes())
    }
}

impl generic_ec_core::Reduce<32> for Scalar {
    fn from_be_array_mod_order(bytes: &[u8; 32]) -> Self {
        let mut bytes = *bytes;
        bytes.reverse();
        Self(curve25519::Scalar::from_bytes_mod_order(bytes))
    }
    fn from_le_array_mod_order(bytes: &[u8; 32]) -> Self {
        Self(curve25519::Scalar::from_bytes_mod_order(*bytes))
    }
}
impl generic_ec_core::Reduce<64> for Scalar {
    fn from_be_array_mod_order(bytes: &[u8; 64]) -> Self {
        let mut bytes = *bytes;
        bytes.reverse();
        Self(curve25519::Scalar::from_bytes_mod_order_wide(&bytes))
    }
    fn from_le_array_mod_order(bytes: &[u8; 64]) -> Self {
        Self(curve25519::Scalar::from_bytes_mod_order_wide(bytes))
    }
}
