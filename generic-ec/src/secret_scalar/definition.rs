#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(all())))]
mod with_alloc {
    use alloc::sync::Arc;
    use zeroize::{Zeroize, Zeroizing};

    use crate::{Curve, Scalar};

    #[doc = include_str!("docs.md")]
    pub struct SecretScalar<E: Curve>(Arc<Zeroizing<Scalar<E>>>);

    impl<E: Curve> SecretScalar<E> {
        #[doc = include_str!("docs-constructor.md")]
        pub fn new(scalar: &mut Scalar<E>) -> Self {
            let mut scalar_on_heap = Arc::<Zeroizing<Scalar<E>>>::default();
            let scalar_mut = Arc::make_mut(&mut scalar_on_heap);
            core::mem::swap(&mut **scalar_mut, scalar);
            scalar.zeroize();
            Self(scalar_on_heap)
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
}

#[cfg(not(feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(all())))]
mod without_alloc {
    use zeroize::{Zeroize, Zeroizing};

    use crate::{Curve, Scalar};

    #[doc = include_str!("docs.md")]
    pub struct SecretScalar<E: Curve>(Zeroizing<Scalar<E>>);

    impl<E: Curve> SecretScalar<E> {
        #[doc = include_str!("docs-constructor.md")]
        pub fn new(scalar: &mut Scalar<E>) -> Self {
            let scalar_new = Self(Zeroizing::new(*scalar));
            scalar.zeroize();
            scalar_new
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
}

mod secret_scalar {
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(all())))]
    pub use super::with_alloc::SecretScalar;
    #[cfg(not(feature = "alloc"))]
    #[cfg_attr(docsrs, doc(cfg(all())))]
    pub use super::without_alloc::SecretScalar;
}

pub use self::secret_scalar::SecretScalar;
