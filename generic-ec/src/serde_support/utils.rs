use core::{fmt, marker::PhantomData};

use serde::{
    de::{self, Visitor},
    Deserializer, Serializer,
};
use serde_with::{DeserializeAs, SerializeAs};

pub use self::expectation::Expectation;

/// (De)serializes a sequence of bytes
///
/// Serializes bytes in hex encoding for human-readable formats (like json)
/// when `std` feature is enabled. Otherwise serializes bytes as is.
pub struct Bytes<Ex>(PhantomData<Ex>);

impl<T, Ex> SerializeAs<T> for Bytes<Ex>
where
    T: AsRef<[u8]>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bytes(serializer, source.as_ref())
    }
}

impl<'de, Ex, T> DeserializeAs<'de, T> for Bytes<Ex>
where
    Ex: expectation::Expectation,
    T: AsMut<[u8]> + Default,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_bytes::<'de, D, Ex, T>(deserializer)
    }
}

pub mod expectation {
    use core::fmt;

    pub trait Expectation {
        fn fmt(f: &mut fmt::Formatter) -> fmt::Result;
    }

    macro_rules! declare_expectations {
        ($(
            $struct_name:ident => fmt!($($fmt:tt)*)
        ),+,) => {$(
            pub struct $struct_name;
            impl Expectation for $struct_name {
                fn fmt(f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, $($fmt)*)
                }
            }
        )+};
    }

    declare_expectations! {
        Coordinate => fmt!("coordinate of point on elliptic curve"),
    }
}

fn serialize_bytes<S: Serializer>(serializer: S, bytes: &[u8]) -> Result<S::Ok, S::Error> {
    #[cfg(feature = "alloc")]
    if serializer.is_human_readable() {
        let bytes_hex = hex::encode(bytes);
        return serializer.serialize_str(&bytes_hex);
    }

    serializer.serialize_bytes(bytes)
}

fn deserialize_bytes<'de, D: Deserializer<'de>, E: Expectation, T: AsMut<[u8]> + Default>(
    deserializer: D,
) -> Result<T, D::Error> {
    struct BytesVisitor<E: Expectation, T: AsMut<[u8]>> {
        output: T,
        _ph: PhantomData<E>,
    }
    impl<'de, E: Expectation, T: AsMut<[u8]>> Visitor<'de> for BytesVisitor<E, T> {
        type Value = T;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            E::fmt(f)
        }
        fn visit_str<Err>(mut self, v: &str) -> Result<Self::Value, Err>
        where
            Err: de::Error,
        {
            hex::decode_to_slice(v, self.output.as_mut())
                .map_err(|err| Err::custom(MalformedHexEncoding(err)))?;
            Ok(self.output)
        }
        fn visit_bytes<Err>(mut self, v: &[u8]) -> Result<Self::Value, Err>
        where
            Err: de::Error,
        {
            let expected_len = self.output.as_mut().len();
            if v.len() != expected_len {
                return Err(Err::invalid_length(v.len(), &ExpectedLength(expected_len)));
            }
            self.output.as_mut().copy_from_slice(v);
            Ok(self.output)
        }
        fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let expected_len = self.output.as_mut().len();
            if let Some(actual_len) = seq.size_hint() {
                if actual_len != expected_len {
                    return Err(<A::Error as de::Error>::invalid_length(
                        actual_len,
                        &ExpectedLength(expected_len),
                    ));
                }
            }

            for (i, digit) in self.output.as_mut().iter_mut().enumerate() {
                *digit = seq.next_element()?.ok_or_else(|| {
                    <A::Error as de::Error>::invalid_length(i, &ExpectedLength(expected_len))
                })?;
            }

            let mut actual_len = self.output.as_mut().len();
            while let Some(de::IgnoredAny) = seq.next_element()? {
                actual_len += 1
            }
            if actual_len != expected_len {
                return Err(<A::Error as de::Error>::invalid_length(
                    actual_len,
                    &ExpectedLength(expected_len),
                ));
            }

            Ok(self.output)
        }
    }

    let visitor = BytesVisitor::<E, _> {
        output: T::default(),
        _ph: PhantomData,
    };
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(visitor)
    } else {
        deserializer.deserialize_bytes(visitor)
    }
}

struct MalformedHexEncoding(pub hex::FromHexError);

impl fmt::Display for MalformedHexEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "malformed hex encoding: {}", self.0)
    }
}

struct ExpectedLength(usize);

impl de::Expected for ExpectedLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bytes", self.0)
    }
}
