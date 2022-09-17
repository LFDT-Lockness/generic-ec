use crate::Curve;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EncodedPoint<E: Curve>(EncodedPointInner<E>);

impl<E: Curve> EncodedPoint<E> {
    pub(crate) fn new_compressed(bytes: E::CompressedPointArray) -> Self {
        Self(EncodedPointInner::Compressed(bytes))
    }

    pub(crate) fn new_uncompressed(bytes: E::UncompressedPointArray) -> Self {
        Self(EncodedPointInner::Uncompressed(bytes))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum EncodedPointInner<E: Curve> {
    Compressed(E::CompressedPointArray),
    Uncompressed(E::UncompressedPointArray),
}

impl<E: Curve> EncodedPoint<E> {
    pub fn as_bytes(&self) -> &[u8] {
        match &self.0 {
            EncodedPointInner::Compressed(bytes) => bytes.as_ref(),
            EncodedPointInner::Uncompressed(bytes) => bytes.as_ref(),
        }
    }
}

impl<E: Curve> AsRef<[u8]> for EncodedPoint<E> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
