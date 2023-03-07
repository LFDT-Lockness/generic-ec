use core::{fmt, ops};

use crate::{as_raw::AsRaw, core::ByteArray, Curve};

/// Bytes representation of an elliptic point
pub struct EncodedPoint<E: Curve>(EncodedPointInner<E>);

impl<E: Curve> EncodedPoint<E> {
    pub(crate) fn new_compressed(bytes: E::CompressedPointArray) -> Self {
        Self(EncodedPointInner::Compressed(bytes))
    }

    pub(crate) fn new_uncompressed(bytes: E::UncompressedPointArray) -> Self {
        Self(EncodedPointInner::Uncompressed(bytes))
    }

    /// Returns bytes representation of the point
    pub fn as_bytes(&self) -> &[u8] {
        match &self.0 {
            EncodedPointInner::Compressed(bytes) => bytes.as_ref(),
            EncodedPointInner::Uncompressed(bytes) => bytes.as_ref(),
        }
    }
}

impl<E: Curve> Clone for EncodedPoint<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<E: Curve> PartialEq for EncodedPoint<E> {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<E: Curve> Eq for EncodedPoint<E> {}

impl<E: Curve> fmt::Debug for EncodedPoint<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple("EncodedPoint");
        #[cfg(feature = "alloc")]
        {
            tuple.field(&hex::encode(self.as_bytes()));
        }
        tuple.finish()
    }
}

impl<E: Curve> ops::Deref for EncodedPoint<E> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[derive(Clone)]
enum EncodedPointInner<E: Curve> {
    Compressed(E::CompressedPointArray),
    Uncompressed(E::UncompressedPointArray),
}

impl<E: Curve> AsRef<[u8]> for EncodedPoint<E> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Bytes representation of a scalar (either in big-endian or in little-endian)
#[derive(Clone)]
pub struct EncodedScalar<E: Curve>(E::ScalarArray);

impl<E: Curve> EncodedScalar<E> {
    pub(crate) fn new(bytes: E::ScalarArray) -> Self {
        Self(bytes)
    }

    /// Returns bytes representation of a scalar
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<E: Curve> AsRef<[u8]> for EncodedScalar<E> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<E: Curve> AsMut<[u8]> for EncodedScalar<E> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl<E: Curve> ops::Deref for EncodedScalar<E> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<E: Curve> fmt::Debug for EncodedScalar<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_tuple("EncodedScalar");
        #[cfg(feature = "std")]
        {
            s.field(&hex::encode(self.as_bytes()));
        }
        s.finish()
    }
}

impl<E: Curve> PartialEq for EncodedScalar<E> {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<E: Curve> Eq for EncodedScalar<E> {}

impl<E: Curve> Default for EncodedScalar<E> {
    fn default() -> Self {
        let bytes = E::ScalarArray::zeroes();
        Self(bytes)
    }
}

impl<E: Curve> AsRaw for EncodedScalar<E> {
    type Raw = E::ScalarArray;
    fn as_raw(&self) -> &Self::Raw {
        &self.0
    }
}
