//! Serde support
//!
//! ## Default serialization format
//!
//! By default, serialization format is excessive. Points are serialized without compression.
//! Points and scalars have an extra field "curve" that specifies which curve this point/scalar
//! belongs to.
//!
//! ```rust
//! # fn main() -> Result<(), serde_json::Error> {
//! use generic_ec::{Curve, Point, Scalar, curves::Secp256k1};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! #[serde(bound = "")]
//! pub struct ZkProof<E: Curve> {
//!     some_point: Point<E>,
//!     some_scalar: Scalar<E>,
//! }
//!
//! let proof = ZkProof::<Secp256k1> {
//!     some_point: Point::generator().to_point(),
//!     some_scalar: Scalar::one(),
//! };
//! assert_eq!(serde_json::to_string_pretty(&proof)?, r#"{
//!   "some_point": {
//!     "curve": "secp256k1",
//!     "point": "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"
//!   },
//!   "some_scalar": {
//!     "curve": "secp256k1",
//!     "scalar": "0000000000000000000000000000000000000000000000000000000000000001"
//!   }
//! }"#);
//! # Ok(()) }
//! ```
//!
//! Excessive serialization format enables better security as it makes it harder to misuse the library.
//! E.g. if by some mistake you parse a point that was initially generated on another curve, you'll get
//! instant error. Without this field, behavior is uncertain and difficult to debug: point from one curve
//! can happen to be a valid point on another curve.
//!
//! ## Compact serialization format
//!
//! You may opt for compact serialization format. If you do that, points are seialized in compressed form, and
//! extra "curve" field is dropped.
//!
//! Compact serialization format can be enabled using [serde_with] crate and [`Compact`] helper struct:
//!
//! ```rust
//! # fn main() -> Result<(), serde_json::Error> {
//! use generic_ec::{Curve, Point, Scalar, curves::Secp256k1};
//! use serde::{Serialize, Deserialize};
//! use serde_with::serde_as;
//!
//! #[serde_as]
//! #[derive(Serialize, Deserialize)]
//! #[serde(bound = "")]
//! pub struct ZkProof<E: Curve> {
//!     #[serde_as(as = "generic_ec::serde::Compact")]
//!     some_point: Point<E>,
//!     #[serde_as(as = "generic_ec::serde::Compact")]
//!     some_scalar: Scalar<E>,
//! }
//!
//! let proof = ZkProof::<Secp256k1> {
//!     some_point: Point::generator().to_point(),
//!     some_scalar: Scalar::one(),
//! };
//! assert_eq!(serde_json::to_string_pretty(&proof)?, r#"{
//!   "some_point": "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
//!   "some_scalar": "0000000000000000000000000000000000000000000000000000000000000001"
//! }"#);
//! # Ok(()) }
//! ```

use core::{convert::TryInto, fmt};

use phantom_type::PhantomType;
use serde::{de::Visitor, Deserialize, Serialize};
use serde_with::{DeserializeAs, SerializeAs};

use crate::core::Curve;
use crate::{Point, Scalar, SecretScalar};

impl<E: Curve> Serialize for Point<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        models::PointUncompressed::from(self).serialize(serializer)
    }
}

impl<'de, E: Curve> Deserialize<'de> for Point<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        models::PointUncompressed::deserialize(deserializer)?
            .try_into()
            .map_err(<D::Error as serde::de::Error>::custom)
    }
}

impl<E: Curve> Serialize for Scalar<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        models::ScalarUncompressed::from(self).serialize(serializer)
    }
}

impl<'de, E: Curve> Deserialize<'de> for Scalar<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        models::ScalarUncompressed::deserialize(deserializer)?
            .try_into()
            .map_err(<D::Error as serde::de::Error>::custom)
    }
}

impl<E: Curve> Serialize for SecretScalar<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl<'de, E: Curve> Deserialize<'de> for SecretScalar<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(SecretScalar::new(&mut Scalar::deserialize(deserializer)?))
    }
}

/// Compact serialization format
pub struct Compact;

impl<E: Curve> SerializeAs<Point<E>> for Compact {
    fn serialize_as<S>(source: &Point<E>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        models::PointCompact::from(source).serialize(serializer)
    }
}

impl<'de, E: Curve> DeserializeAs<'de, Point<E>> for Compact {
    fn deserialize_as<D>(deserializer: D) -> Result<Point<E>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        models::PointCompact::deserialize(deserializer)?
            .try_into()
            .map_err(<D::Error as serde::de::Error>::custom)
    }
}

impl<E: Curve> SerializeAs<Scalar<E>> for Compact {
    fn serialize_as<S>(source: &Scalar<E>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        models::ScalarCompact::from(source).serialize(serializer)
    }
}

impl<'de, E: Curve> DeserializeAs<'de, Scalar<E>> for Compact {
    fn deserialize_as<D>(deserializer: D) -> Result<Scalar<E>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        models::ScalarCompact::deserialize(deserializer)?
            .try_into()
            .map_err(<D::Error as serde::de::Error>::custom)
    }
}

impl<E: Curve> SerializeAs<SecretScalar<E>> for Compact {
    fn serialize_as<S>(source: &SecretScalar<E>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        models::ScalarCompact::from(source.as_ref()).serialize(serializer)
    }
}

impl<'de, E: Curve> DeserializeAs<'de, SecretScalar<E>> for Compact {
    fn deserialize_as<D>(deserializer: D) -> Result<SecretScalar<E>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut scalar = <Compact as DeserializeAs<'de, Scalar<E>>>::deserialize_as(deserializer)?;
        Ok(SecretScalar::new(&mut scalar))
    }
}

/// A guard type asserting that deserialized value belongs to curve `E`
///
/// It implements [Serialize] and [Deserialize] traits. When serialized, `CurveName`
/// is converted into string containing curve name. When deserialized, it parses a string
/// and requires it to match curve name, otherwise deserialization error is returned.
///
/// ## Example
///
/// ```rust
/// # fn main() -> Result<(), serde_json::Error> {
/// use generic_ec::{serde::CurveName, curves::{Secp256k1, Secp256r1}};
/// use serde_json::Value;
///
/// let curve_name = CurveName::<Secp256k1>::new();
/// let value = serde_json::to_value(&curve_name)?;
/// assert_eq!(value, Value::String("secp256k1".into()));
///
/// // `value` can be deserialized back to `CurveName<Secp256k1>`
/// let _curve_name: CurveName<Secp256k1> = serde_json::from_value(value.clone())?;
///
/// // but it can't be deserialized to `CurveName<Secp256r1>`
/// let serialization_fails = serde_json::from_value::<CurveName<Secp256r1>>(value);
/// assert!(serialization_fails.is_err());
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct CurveName<E: Curve>(PhantomType<E>);

impl<E: Curve> CurveName<E> {
    /// Construct a `CurveName` guard
    pub fn new() -> Self {
        Self(PhantomType::new())
    }
}

impl<E: Curve> Default for CurveName<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: Curve> Serialize for CurveName<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(E::CURVE_NAME)
    }
}

impl<'de, E: Curve> Deserialize<'de> for CurveName<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        pub struct CurveNameVisitor<E: Curve>(PhantomType<E>);
        impl<'de, E: Curve> Visitor<'de> for CurveNameVisitor<E> {
            type Value = CurveName<E>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "curve {name}", name = E::CURVE_NAME)
            }
            fn visit_str<Error>(self, v: &str) -> Result<Self::Value, Error>
            where
                Error: serde::de::Error,
            {
                if v == E::CURVE_NAME {
                    Ok(CurveName::default())
                } else {
                    Err(Error::custom(error_msg::ExpectedCurve {
                        expected: E::CURVE_NAME,
                        got: v,
                    }))
                }
            }
        }
        deserializer.deserialize_str(CurveNameVisitor(PhantomType::new()))
    }
}

mod models {
    use core::convert::TryFrom;

    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;

    use crate::core::{CompressedEncoding, IntegerEncoding, UncompressedEncoding};
    use crate::{as_raw::AsRaw, Curve, Point, Scalar};

    use super::{
        error_msg::{InvalidPoint, InvalidScalar},
        CurveName,
    };

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    #[serde(bound = "")]
    pub struct PointUncompressed<E: Curve> {
        curve: CurveName<E>,
        #[serde_as(as = "super::utils::Bytes")]
        point: E::UncompressedPointArray,
    }
    impl<E: Curve> From<&Point<E>> for PointUncompressed<E> {
        fn from(p: &Point<E>) -> Self {
            let bytes = p.as_raw().to_bytes_uncompressed();
            Self {
                curve: CurveName::new(),
                point: bytes,
            }
        }
    }
    impl<E: Curve> TryFrom<PointUncompressed<E>> for Point<E> {
        type Error = InvalidPoint;
        fn try_from(value: PointUncompressed<E>) -> Result<Self, Self::Error> {
            Point::from_bytes(value.point).or(Err(InvalidPoint))
        }
    }

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    #[serde(bound = "")]
    pub struct PointCompact<E: Curve>(
        #[serde_as(as = "super::utils::Bytes")] E::CompressedPointArray,
    );
    impl<E: Curve> From<&Point<E>> for PointCompact<E> {
        fn from(p: &Point<E>) -> Self {
            let bytes = p.as_raw().to_bytes_compressed();
            Self(bytes)
        }
    }
    impl<E: Curve> TryFrom<PointCompact<E>> for Point<E> {
        type Error = InvalidPoint;
        fn try_from(value: PointCompact<E>) -> Result<Self, Self::Error> {
            Point::from_bytes(value.0).or(Err(InvalidPoint))
        }
    }

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    #[serde(bound = "")]
    pub struct ScalarUncompressed<E: Curve> {
        curve: CurveName<E>,
        #[serde_as(as = "super::utils::Bytes")]
        scalar: E::ScalarArray,
    }
    impl<E: Curve> From<&Scalar<E>> for ScalarUncompressed<E> {
        fn from(s: &Scalar<E>) -> Self {
            let bytes = s.as_raw().to_be_bytes();
            Self {
                curve: CurveName::new(),
                scalar: bytes,
            }
        }
    }
    impl<E: Curve> TryFrom<ScalarUncompressed<E>> for Scalar<E> {
        type Error = InvalidScalar;
        fn try_from(value: ScalarUncompressed<E>) -> Result<Self, Self::Error> {
            Scalar::from_be_bytes(value.scalar).or(Err(InvalidScalar))
        }
    }

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    #[serde(bound = "")]
    pub struct ScalarCompact<E: Curve>(#[serde_as(as = "super::utils::Bytes")] E::ScalarArray);
    impl<E: Curve> From<&Scalar<E>> for ScalarCompact<E> {
        fn from(s: &Scalar<E>) -> Self {
            let bytes = s.as_raw().to_be_bytes();
            Self(bytes)
        }
    }
    impl<E: Curve> TryFrom<ScalarCompact<E>> for Scalar<E> {
        type Error = InvalidScalar;
        fn try_from(value: ScalarCompact<E>) -> Result<Self, Self::Error> {
            Scalar::from_be_bytes(&value.0).or(Err(InvalidScalar))
        }
    }
}

mod utils {
    use core::fmt;

    use serde::de::{self, Visitor};
    use serde_with::{DeserializeAs, SerializeAs};

    use crate::core::ByteArray;

    pub struct Bytes;

    impl<T> SerializeAs<T> for Bytes
    where
        T: AsRef<[u8]>,
    {
        fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if serializer.is_human_readable() {
                // We only support serialization of byte arrays up to 128 bytes. It can be generalized when
                // Rust has better support of const generics
                let mut buf = [0u8; 256];

                if source.as_ref().len() * 2 > buf.len() {
                    return Err(<S::Error as serde::ser::Error>::custom(
                        super::error_msg::ByteArrayTooLarge {
                            len: source.as_ref().len(),
                            supported_len: buf.len() / 2,
                        },
                    ));
                }
                let buf = &mut buf[..2 * source.as_ref().len()];
                hex::encode_to_slice(source, buf)
                    .map_err(<S::Error as serde::ser::Error>::custom)?;
                let buf_str = core::str::from_utf8(buf).map_err(|e| {
                    <S::Error as serde::ser::Error>::custom(super::error_msg::MalformedHex(e))
                })?;
                serializer.serialize_str(buf_str)
            } else {
                serializer.serialize_bytes(source.as_ref())
            }
        }
    }

    impl<'de, T> DeserializeAs<'de, T> for Bytes
    where
        T: ByteArray,
    {
        fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            pub struct BytesVisitor<T>(T);
            impl<'de, T: AsMut<[u8]>> Visitor<'de> for BytesVisitor<T> {
                type Value = T;
                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "bytes")
                }
                fn visit_str<E>(mut self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    hex::decode_to_slice(v, self.0.as_mut()).map_err(E::custom)?;
                    Ok(self.0)
                }
                fn visit_bytes<E>(mut self, v: &[u8]) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    let expected_len = self.0.as_mut().len();
                    if v.len() != expected_len {
                        return Err(E::invalid_length(
                            v.len(),
                            &super::error_msg::ExpectedLen(expected_len),
                        ));
                    }
                    self.0.as_mut().copy_from_slice(v);
                    Ok(self.0)
                }
                fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
                {
                    let expected_len = self.0.as_mut().len();
                    let bytes = self.0.as_mut().iter_mut().enumerate();

                    for (i, byte_i) in bytes {
                        let byte_parsed = seq.next_element()?.ok_or_else(|| {
                            <A::Error as de::Error>::invalid_length(
                                i,
                                &super::error_msg::ExpectedLen(expected_len),
                            )
                        })?;
                        *byte_i = byte_parsed;
                    }

                    let mut unparsed_bytes = 0;
                    while seq.next_element::<serde::de::IgnoredAny>()?.is_some() {
                        unparsed_bytes += 1
                    }

                    if unparsed_bytes > 0 {
                        Err(<A::Error as de::Error>::invalid_length(
                            expected_len + unparsed_bytes,
                            &super::error_msg::ExpectedLen(expected_len),
                        ))
                    } else {
                        Ok(self.0)
                    }
                }
            }
            let visitor = BytesVisitor(T::zeroes());
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(visitor)
            } else {
                deserializer.deserialize_bytes(visitor)
            }
        }
    }
}

mod error_msg {
    use core::fmt;

    use serde::de::Expected;

    pub struct ExpectedCurve<'g> {
        pub expected: &'static str,
        pub got: &'g str,
    }

    impl<'g> fmt::Display for ExpectedCurve<'g> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "expected {e} curve, got {g}",
                e = self.expected,
                g = self.got
            )
        }
    }

    pub struct ExpectedLen(pub usize);

    impl Expected for ExpectedLen {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} bytes", self.0)
        }
    }

    pub struct InvalidPoint;
    impl fmt::Display for InvalidPoint {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "invalid point")
        }
    }

    pub struct InvalidScalar;
    impl fmt::Display for InvalidScalar {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "invalid scalar")
        }
    }

    pub struct MalformedHex(pub core::str::Utf8Error);
    impl fmt::Display for MalformedHex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "malformed hex: {}", self.0)
        }
    }

    pub struct ByteArrayTooLarge {
        pub len: usize,
        pub supported_len: usize,
    }
    impl fmt::Display for ByteArrayTooLarge {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "byte array is too large: its length is {} bytes, but only up to {} bytes can be serialized", self.len, self.supported_len)
        }
    }
}
