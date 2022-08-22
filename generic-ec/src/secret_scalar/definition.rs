#[cfg(feature = "alloc")]
mod with_alloc {
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use zeroize::{Zeroize, Zeroizing};

    use crate::{Curve, Scalar};

    #[doc = include_str!("docs.md")]
    pub struct SecretScalar<E: Curve>(Arc<Box<Zeroizing<Scalar<E>>>>);

    impl<E: Curve> SecretScalar<E> {
        #[doc = include_str!("docs-constructor.md")]
        pub fn new(scalar: &mut Scalar<E>) -> Self {
            let mut scalar_boxed = Box::<Zeroizing<Scalar<E>>>::default();
            core::mem::swap(&mut **scalar_boxed, scalar);
            scalar.zeroize();
            Self(Arc::new(scalar_boxed))
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
mod without_alloc {
    use zeroize::{Zeroize, Zeroizing};

    use crate::Scalar;

    #[doc = include_str!("docs.md")]
    pub struct SecretScalar<E: Curve>(Zeroizing<Scalar<E>>);

    impl<E: Curve> SecretScalar<E> {
        #[doc = include_str!("docs-constructor.md")]
        pub fn new(scalar: &mut Scalar) -> Self {
            let scalar = Self(Zeroizing::new(*scalar));
            scalar.zeroize();
            scalar
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
    pub use super::with_alloc::SecretScalar;
    #[cfg(not(feature = "alloc"))]
    pub use super::without_alloc::SecretScalar;
}

pub use self::secret_scalar::SecretScalar;
