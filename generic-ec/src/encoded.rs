use core::{fmt, ops};

use subtle::{Choice, ConstantTimeEq};
use zeroize::Zeroize;

use crate::{as_raw::AsRaw, core::ByteArray, Curve};

/// Bytes representation of an elliptic point
#[derive(Zeroize)]
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

impl<E: Curve> Default for EncodedPoint<E> {
    fn default() -> Self {
        Self(EncodedPointInner::Uncompressed(
            E::UncompressedPointArray::zeroes(),
        ))
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

impl<E: Curve> AsMut<[u8]> for EncodedPoint<E> {
    fn as_mut(&mut self) -> &mut [u8] {
        match &mut self.0 {
            EncodedPointInner::Compressed(a) => a.as_mut(),
            EncodedPointInner::Uncompressed(a) => a.as_mut(),
        }
    }
}

impl<E: Curve> Zeroize for EncodedPointInner<E> {
    fn zeroize(&mut self) {
        match self {
            EncodedPointInner::Compressed(a) => a.as_mut().zeroize(),
            EncodedPointInner::Uncompressed(a) => a.as_mut().zeroize(),
        }
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

impl<E: Curve> Zeroize for EncodedScalar<E> {
    fn zeroize(&mut self) {
        self.as_mut().zeroize()
    }
}

// Internals for both SecretEncodedPoint and SecretEncodingScalar depending on
// presence of alloc
#[cfg(feature = "alloc")]
mod imp {
    use alloc::sync::Arc;

    pub(super) type Secret<T> = Arc<zeroize::Zeroizing<T>>;

    #[inline(always)]
    pub(super) fn new<T>(x: &mut T) -> Secret<T>
    where
        T: zeroize::Zeroize + Default + Clone,
    {
        let mut value_on_heap = Arc::<zeroize::Zeroizing<T>>::default();
        let value_mut = Arc::make_mut(&mut value_on_heap);
        core::mem::swap(&mut **value_mut, x);
        x.zeroize();
        value_on_heap
    }

    pub(super) fn inner<T>(x: Secret<T>) -> T
    where
        T: zeroize::Zeroize + Clone,
    {
        (**x).clone()
    }
}
#[cfg(not(feature = "alloc"))]
mod imp {
    pub(super) type Secret<T> = zeroize::Zeroizing<T>;

    #[inline(always)]
    pub(super) fn new<T>(x: &mut T) -> Secret<T>
    where
        T: zeroize::Zeroize + Clone,
    {
        let value_new = zeroize::Zeroizing::new(x.clone());
        x.zeroize();
        value_new
    }

    pub(super) fn inner<T>(x: Secret<T>) -> T
    where
        T: zeroize::Zeroize + Clone,
    {
        (*x).clone()
    }
}

/// Bytes representation of a secret elliptic point. See [`SecretPoint`] for
/// more information
///
/// This representation is automatically zeroed on drop, and doesn't implement
/// some vartime methods. You can still access the underlying bytes by calling
/// `.as_ref()`. This bypasses all the secrecy guarantees given by this struct.
///
/// [`SecretPoint`]: crate::SecretPoint
#[derive(Clone, Default)]
pub struct EncodedSecretPoint<E: Curve>(imp::Secret<EncodedPoint<E>>);

impl<E: Curve> EncodedSecretPoint<E> {
    /// Wrap a non-secret representation
    #[inline(always)]
    pub fn new(mut point: EncodedPoint<E>) -> Self {
        Self(imp::new(&mut point))
    }

    /// Obtain the reference to the encoded bytes. This bypasses all the secrecy
    /// guarantees, so be careful when handling them
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Convert the representation into not-secret, dropping all guarantees
    pub fn into_not_secret(self) -> EncodedPoint<E> {
        imp::inner(self.0)
    }
}

impl<E: Curve> fmt::Debug for EncodedSecretPoint<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretEncodedPoint")
    }
}

impl<E: Curve> ConstantTimeEq for EncodedSecretPoint<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl<E: Curve> ops::Deref for EncodedSecretPoint<E> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<E: Curve> AsRef<[u8]> for EncodedSecretPoint<E> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Bytes representation of a secret elliptic scalar. See [`SecretScalar`] for
/// more information
///
/// This representation is automatically zeroed on drop, and doesn't implement
/// some vartime methods. You can still access the underlying bytes by calling
/// `.as_ref()`. This bypasses all the secrecy guarantees given by this struct.
///
/// [`SecretScalar`]: crate::SecretScalar
#[derive(Clone, Default)]
pub struct EncodedSecretScalar<E: Curve>(imp::Secret<EncodedScalar<E>>);

impl<E: Curve> EncodedSecretScalar<E> {
    /// Wrap a non-secret representation
    pub fn new(mut scalar: EncodedScalar<E>) -> Self {
        Self(imp::new(&mut scalar))
    }

    /// Obtain the reference to the encoded bytes. This bypasses all the secrecy
    /// guarantees, so be careful when handling them
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Convert the representation into not-secret, dropping all guarantees
    pub fn into_not_secret(self) -> EncodedScalar<E> {
        imp::inner(self.0)
    }
}

impl<E: Curve> fmt::Debug for EncodedSecretScalar<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretEncodedScalar")
    }
}

impl<E: Curve> ConstantTimeEq for EncodedSecretScalar<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl<E: Curve> ops::Deref for EncodedSecretScalar<E> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<E: Curve> AsRef<[u8]> for EncodedSecretScalar<E> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
