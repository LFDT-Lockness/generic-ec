use core::ops::Deref;

use zeroize::Zeroize;

/// Non zero [Point](crate::Point) or [Scalar](crate::Scalar)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroize)]
pub struct NonZero<T>(T);

impl<T> NonZero<T> {
    /// Constructs `NonZero` without checking whether value is actually non zero
    ///
    /// Caller **must** guarantee that value is non zero. Caller **must** provide a comment
    /// justifying a call and proving that value is non zero.
    pub(crate) fn new_unchecked(v: T) -> Self {
        Self(v)
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
