//! Internals for both Secret structs, like `SecretScalar` and `SecretEncodedPoint`. They have a different representation based on the presence of `alloc`: either a reference-counted nonmoving one, or a moving value

#[cfg(feature = "alloc")]
mod imp {
    use alloc::sync::Arc;

    pub type Secret<T> = Arc<zeroize::Zeroizing<T>>;

    #[inline(always)]
    pub fn new<T>(x: &mut T) -> Secret<T>
    where
        T: zeroize::Zeroize + Default + Clone,
    {
        let mut value_on_heap = Arc::<zeroize::Zeroizing<T>>::default();
        let value_mut = Arc::make_mut(&mut value_on_heap);
        core::mem::swap(&mut **value_mut, x);
        x.zeroize();
        value_on_heap
    }

    pub fn inner<T>(x: Secret<T>) -> T
    where
        T: zeroize::Zeroize + Clone,
    {
        (**x).clone()
    }
}
#[cfg(not(feature = "alloc"))]
mod imp {
    pub type Secret<T> = zeroize::Zeroizing<T>;

    #[inline(always)]
    pub fn new<T>(x: &mut T) -> Secret<T>
    where
        T: zeroize::Zeroize + Clone,
    {
        let value_new = zeroize::Zeroizing::new(x.clone());
        x.zeroize();
        value_new
    }

    pub fn inner<T>(x: Secret<T>) -> T
    where
        T: zeroize::Zeroize + Clone,
    {
        (*x).clone()
    }
}

/// Unwrap the Secret into the underlying type
pub(crate) use imp::inner;
/// Wrap the underlying type into the Secret repr
pub(crate) use imp::new;
/// Type used as the representation
pub(crate) use imp::Secret;
