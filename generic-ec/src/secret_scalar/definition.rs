use crate::{Curve, Scalar};

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(all())))]
mod with_alloc {
    use alloc::sync::Arc;
    use zeroize::{Zeroize as _, Zeroizing};

    use crate::{Curve, Scalar};

    pub(super) type Inner<E> = Arc<Zeroizing<Scalar<E>>>;

    #[inline(always)]
    pub(super) fn new<E: Curve>(scalar: &mut Scalar<E>) -> Inner<E> {
        let mut scalar_on_heap = Arc::<Zeroizing<Scalar<E>>>::default();
        let scalar_mut = Arc::make_mut(&mut scalar_on_heap);
        core::mem::swap(&mut **scalar_mut, scalar);
        scalar.zeroize();
        scalar_on_heap
    }
}

#[cfg(not(feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(all())))]
mod without_alloc {
    use zeroize::{Zeroize as _, Zeroizing};

    use crate::{Curve, Scalar};

    pub(super) type Inner<E> = Zeroizing<Scalar<E>>;

    #[inline(always)]
    pub(super) fn new(scalar: &mut Scalar<E>) -> Self {
        let scalar_new = Zeroizing::new(*scalar);
        scalar.zeroize();
        scalar_new
    }

    #[doc = include_str!("docs.md")]
    pub struct SecretScalar<E: Curve>(Zeroizing<Scalar<E>>);
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(all())))]
use with_alloc as imp;
#[cfg(not(feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(all())))]
use without_alloc as imp;

/// Scalar representing sensitive information (like secret key)
///
/// Secret scalar should be treated with an extra care. You shouldn't do any
/// branching (e.g. `Eq`, `Ord`) on the secret to avoid timing side-channel
/// attacks, so it implements only constant time traits (like [`ConstantTimeEq`]).
///
/// Also, when `alloc` feature is enabled, we enforce extra measures:
///
/// * Secret scalar leaves no trace in RAM after it's dropped \
///   Memory is zeroized after use
/// * All clones of secret scalar refer to the same region in the memory \
///   I.e. there will always be only one instance of the scalar in the memory
///   no matter how many clones you make
///
/// All these guarantees can be bypassed by calling `.as_ref()` and obtaining
/// `&Scalar<E>` that is not protected from timing attacks, leaving traces in
/// the memory, etc.
///
/// [`ConstantTimeEq`]: subtle::ConstantTimeEq
pub struct SecretScalar<E: Curve>(imp::Inner<E>);

impl<E: Curve> SecretScalar<E> {
    /// Constructs a new secret scalar
    ///
    /// Takes the original scalar by mutable reference instead of taking by value to
    /// avoid leaving copies of the scalar on stack. Scalar behind the reference will
    /// be zeroized after the function has returned.
    pub fn new(scalar: &mut Scalar<E>) -> Self {
        Self(imp::new(scalar))
    }
}

impl<E: Curve> AsRef<Scalar<E>> for SecretScalar<E> {
    fn as_ref(&self) -> &Scalar<E> {
        &self.0
    }
}

impl<E: Curve> Clone for SecretScalar<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
