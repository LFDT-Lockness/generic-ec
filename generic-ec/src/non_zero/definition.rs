use core::ops::Deref;

use zeroize::Zeroize;

/// Non zero [Point](crate::Point) or [Scalar](crate::Scalar)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroize, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(bound(
        serialize = "T: From<NonZero<T>> + Clone + serde::Serialize",
        deserialize = "NonZero<T>: TryFrom<T>, <NonZero<T> as TryFrom<T>>::Error: core::fmt::Display, T: serde::Deserialize<'de>",
    )),
    serde(into = "T", try_from = "T")
)]
#[cfg_attr(feature = "udigest", derive(udigest::Digestable))]
pub struct NonZero<T>(T);

impl<T> NonZero<T> {
    /// Constructs `NonZero` without checking whether value is actually non zero
    ///
    /// Caller **must** guarantee that value is non zero. Caller **must** provide a comment
    /// justifying a call and proving that value is non zero.
    pub(crate) fn new_unchecked(v: T) -> Self {
        Self(v)
    }

    /// Returns wrapped value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for NonZero<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for NonZero<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
