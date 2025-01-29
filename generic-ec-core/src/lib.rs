//! This crate contains core traits for [`generic-ec`](https://docs.rs/generic-ec) crate.
//! You should only need these traits if you implement your own [`Curve`] instance.
//! Otherwise, `generic-ec` API should suffice.

#![no_std]
#![cfg_attr(not(test), forbid(unused_crate_dependencies))]
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![forbid(missing_docs)]

use core::fmt::Debug;
use core::hash::Hash;

use generic_array::{ArrayLength, GenericArray};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::Zeroize;

pub mod coords;

/// Elliptic curve
///
/// This trait contains all the low-level curve implementation logic: scalar, point arithmetics,
/// encoding and etc.
pub trait Curve: Debug + Copy + Eq + Ord + Hash + Default + Sync + Send + 'static {
    /// Curve name
    const CURVE_NAME: &'static str;

    /// Type that represents a curve point
    type Point: Additive
        + From<CurveGenerator>
        + Zero
        + Zeroize
        + OnCurve
        + SmallFactor
        + Copy
        + Eq
        + ConstantTimeEq
        + ConditionallySelectable
        + Default
        + CompressedEncoding<Bytes = Self::CompressedPointArray>
        + UncompressedEncoding<Bytes = Self::UncompressedPointArray>
        + Decode
        + Unpin
        + Sync
        + Send;
    /// Type that represents a curve scalar
    type Scalar: Additive
        + Multiplicative<Self::Scalar, Output = Self::Scalar>
        + Multiplicative<CurveGenerator, Output = Self::Point>
        + Multiplicative<Self::Point, Output = Self::Point>
        + Invertible
        + Zero
        + One
        + Samplable
        + Zeroize
        + Copy
        + Eq
        + ConstantTimeEq
        + ConditionallySelectable
        + Default
        + IntegerEncoding<Bytes = Self::ScalarArray>
        + Unpin
        + Sync
        + Send;

    /// Byte array that fits the whole bytes representation of compressed point
    type CompressedPointArray: ByteArray;
    /// Byte array that fits the whole bytes representation of uncompressed point
    type UncompressedPointArray: ByteArray;
    /// Byte array that fits the whole bytes representation of a scalar
    type ScalarArray: ByteArray;
    /// Byte array that fits the whole bytes representation of a coordinate
    ///
    /// If a curve doesn't expose point coordinates, it may be `[u8; 0]`
    type CoordinateArray: ByteArray;
}

/// Type for which addition is defined
pub trait Additive {
    /// Computes `a + b`
    fn add(a: &Self, b: &Self) -> Self;
    /// Computes `a - b`
    fn sub(a: &Self, b: &Self) -> Self;
    /// Computes `-a`
    fn negate(x: &Self) -> Self;

    /// Takes `x`, returns `x + x`
    ///
    /// This can be more efficient than calling [`Self::add(x, x)`](Self::add)
    fn double(x: &Self) -> Self
    where
        Self: Sized,
    {
        Self::add(x, x)
    }
}

/// Type for which multiplication is defined
pub trait Multiplicative<Rhs> {
    /// Type of multiplication output
    type Output;
    /// Computes `a * b`
    fn mul(a: &Self, b: &Rhs) -> Self::Output;
}

/// Type for which invert function is defined
pub trait Invertible
where
    Self: Sized,
{
    /// Inverts $x$, returns $x^{-1}$ such that $x \cdot x^{-1} = 1$
    fn invert(x: &Self) -> CtOption<Self>;
}

/// Type that has zero value (additive identity)
pub trait Zero {
    /// Constructs zero value of `Self`
    fn zero() -> Self;
    /// Checks (in constant-time) if `x` is zero
    fn is_zero(x: &Self) -> Choice;
}

/// Type that has "one" value (multiplicative identity)
pub trait One {
    /// Constructs one value of `Self`
    fn one() -> Self;
    /// Checks (in constant-time) if `x` is one
    fn is_one(x: &Self) -> Choice;
}

/// Type can be uniformly sampled
pub trait Samplable {
    /// Byte array that can be converted into instance of `Self` via [`Samplable::from_uniform_bytes`]
    type Bytes: ByteArray;

    /// Maps uniformly distributed bytes array to uniformly distributed instance of `Self`.
    ///
    /// Instead of taking a source of randomness directly, this implementation takes a byte array, that was
    /// populated from the source of randomness, and outputs a random element.
    ///
    /// ## Guarantees
    /// When `bytes` are uniformly distributed, output distribution must be indistinguishable from uniform,
    /// respective to the security level of the curve.
    ///
    /// ## Implementation details
    /// This trait is required to be implemented for scalars. It's recommended to take approach described in
    /// "5. Hashing to Finite Field" of [RFC9380]:
    ///
    /// 1. Take a uniform bytestring of `L` bytes, where
    ///    ```text
    ///    L = ceil((ceil(log2(q)) + k) / 8)
    ///    ```
    ///
    ///    `q` is prime (sub)group order, and `k` is target security level in bits
    /// 2. Interpret the bytestring as big-endian/little-endian encoding of the integer, and reduce it modulo `q`
    ///
    /// The output is then guaranteed to have a bias at most 2^-k.
    ///
    /// [RFC9380]: https://www.rfc-editor.org/rfc/rfc9380.html#name-hashing-to-a-finite-field
    fn from_uniform_bytes(bytes: Self::Bytes) -> Self;
}

/// Checks whether the point is on curve
pub trait OnCurve {
    /// Checks whether the point is on curve
    fn is_on_curve(&self) -> Choice;
}

/// Checks whether a point has small factor
pub trait SmallFactor {
    /// Checks whether a point has no small factor
    fn is_torsion_free(&self) -> Choice;
}

/// Curve generator
///
/// Represents a curve generator. The curve point must implement `From<CurveGenerator>`.
/// The curve scalar can be multiplied at `CurveGenerator`, implementation may be
/// more efficient than a generic multiplication.
pub struct CurveGenerator;

/// Compressed encoding of the point
pub trait CompressedEncoding
where
    Self: Sized,
{
    /// Byte array that fits the whole compressed point representation
    type Bytes: ByteArray;

    /// Encodes the point as bytes in compressed form
    fn to_bytes_compressed(&self) -> Self::Bytes;
}

/// Uncompressed encoding of the point
pub trait UncompressedEncoding
where
    Self: Sized,
{
    /// Byte array that fits the whole uncompressed point representation
    type Bytes: ByteArray;

    /// Encodes the point as bytes in uncompressed form
    ///
    /// Some curves may not have such thing as compressed and uncompressed forms.
    /// For these curves, we `CompressedEncoding` and `UncompressedEncoding` should
    /// return the same encoding.
    fn to_bytes_uncompressed(&self) -> Self::Bytes;
}

/// Encodes an integer as bytes
pub trait IntegerEncoding
where
    Self: Sized,
{
    /// Byte array that fits the whole encoded integer
    type Bytes: ByteArray;

    /// Encodes integer as bytes in big-endian byte order
    fn to_be_bytes(&self) -> Self::Bytes;
    /// Encodes integer as bytes in little-endian byte order
    fn to_le_bytes(&self) -> Self::Bytes;

    /// Decodes integer encoded as bytes in big-endian bytes order
    ///
    /// Returns `None` if the bytes don't correspond to a valid integer.
    fn from_be_bytes_exact(bytes: &Self::Bytes) -> Option<Self>;
    /// Decodes integer encoded as bytes in little-endian bytes order
    ///
    /// Returns `None` if the bytes don't correspond to a valid integer.
    fn from_le_bytes_exact(bytes: &Self::Bytes) -> Option<Self>;

    /// Interprets `bytes` as big-endian encoding of an integer. Returns integer mod curve (prime) order.
    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self;
    /// Interprets `bytes` as little-endian encoding of an integer. Returns integer mod curve (prime) order.
    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self;
}

/// Decodes a point from its compressed or uncompressed representation
pub trait Decode: Sized {
    /// Decodes a point from its compressed or uncompressed representation
    fn decode(bytes: &[u8]) -> Option<Self>;
}

/// Error type
pub struct Error;

/// Byte array
pub trait ByteArray: AsRef<[u8]> + AsMut<[u8]> + Clone + Send + Sync + 'static {
    /// New byte array of zeroes
    ///
    /// Alternative to [`Default`] that is not implemented for generic `[T; N]`
    /// (see [tracking issue](https://github.com/rust-lang/rust/issues/61415))
    fn zeroes() -> Self;
}

impl<const N: usize> ByteArray for [u8; N] {
    fn zeroes() -> Self {
        [0; N]
    }
}

impl<N: ArrayLength<u8>> ByteArray for GenericArray<u8, N> {
    fn zeroes() -> Self {
        GenericArray::default()
    }
}

/// Reduces an integer represented as array of `N` bytes modulo curve (prime) order
pub trait Reduce<const N: usize> {
    /// Interprets `bytes` as big-endian encoding of an integer, returns this
    /// integer modulo curve (prime) order
    fn from_be_array_mod_order(bytes: &[u8; N]) -> Self;
    /// Interprets `bytes` as little-endian encoding of an integer, returns this
    /// integer modulo curve (prime) order
    fn from_le_array_mod_order(bytes: &[u8; N]) -> Self;
}

/// Marker trait for curves whose underlying implementation doesn't allow
/// representing invalid points.
/// # Safety
/// Safe to implement when the checks for valid points always return `true`.
/// Those checks are:
/// - [`OnCurve::is_on_curve`]
/// - [`SmallFactor::is_torsion_free`]
pub unsafe trait NoInvalidPoints {}
