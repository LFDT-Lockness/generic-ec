use core::fmt;

use crate::Curve;

pub struct EncodedPoint<E: Curve>(EncodedPointInner<E>);

impl<E: Curve> EncodedPoint<E> {
    pub(crate) fn new_compressed(bytes: E::CompressedPointArray) -> Self {
        Self(EncodedPointInner::Compressed(bytes))
    }

    pub(crate) fn new_uncompressed(bytes: E::UncompressedPointArray) -> Self {
        Self(EncodedPointInner::Uncompressed(bytes))
    }

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

pub struct EncodedScalar<E: Curve>(E::ScalarArray);

impl<E: Curve> EncodedScalar<E> {
    pub(crate) fn new(bytes: E::ScalarArray) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<E: Curve> AsRef<[u8]> for EncodedScalar<E> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
