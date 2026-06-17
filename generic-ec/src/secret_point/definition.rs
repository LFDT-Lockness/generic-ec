use crate::{Curve, Point};

#[cfg(feature = "alloc")]
mod with_alloc {
    use alloc::sync::Arc;
    use zeroize::Zeroize as _;

    use crate::{Curve, Point};

    pub(super) type Internal<E> = Arc<zeroize::Zeroizing<Point<E>>>;

    #[inline(always)]
    pub(super) fn new<E: Curve>(point: &mut Point<E>) -> Internal<E> {
        let mut point_on_heap = Arc::<zeroize::Zeroizing<Point<E>>>::default();
        let point_mut = Arc::make_mut(&mut point_on_heap);
        core::mem::swap(&mut **point_mut, point);
        point.zeroize();
        point_on_heap
    }
}

#[cfg(not(feature = "alloc"))]
mod without_alloc {
    pub(super) type Internal<E> = zeroize::Zeroizing<crate::Point<E>>;

    #[inline(always)]
    pub fn new(point: &mut crate::Point<E>) -> Internal {
        let point_new = zeroize::Zeroizing::new(*point);
        point.zeroize();
        point_new
    }
}

#[cfg(feature = "alloc")]
use with_alloc as imp;
#[cfg(not(feature = "alloc"))]
use without_alloc as imp;

/// Point representing sensitive information (like a derived secret)
///
/// Secret point should be treated with an extra care. You shouldn't do any
/// branching (e.g. `Eq`, `Ord`) on the secret to avoid timing side-channel
/// attacks, so it implements only constant time traits (like [`ConstantTimeEq`]).
///
/// Also, when `alloc` feature is enabled, we enforce extra measures:
///
/// * Secret point leaves no trace in RAM after it's dropped \
///   Memory is zeroized after use
/// * All clones of a secret point refer to the same region in the memory \
///   I.e. there will always be only one instance of the point in the memory
///   no matter how many clones you make
///
/// All these guarantees can be bypassed by calling `.as_ref()` and obtaining
/// `&Point<E>` that is not protected from timing attacks, leaving traces in
/// the memory, etc.
///
/// [`ConstantTimeEq`]: subtle::ConstantTimeEq
pub struct SecretPoint<E: Curve>(imp::Internal<E>);

impl<E: Curve> SecretPoint<E> {
    /// Constructs a new secret point
    ///
    /// Takes the original point by mutable reference instead of taking by value
    /// to avoid leaving copies of the point on stack. Point behind the
    /// reference will be zeroized after the function has returned.
    pub fn new(point: &mut Point<E>) -> Self {
        Self(imp::new(point))
    }
}

impl<E: Curve> AsRef<Point<E>> for SecretPoint<E> {
    fn as_ref(&self) -> &Point<E> {
        &self.0
    }
}

impl<E: Curve> Clone for SecretPoint<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
