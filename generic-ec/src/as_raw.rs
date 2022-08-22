//! Accessing backend library representation of point/scalar
//!
//! > You don't normally need to use traits from this module
//!
//! `Point<E>` and `Scalar<E>` provide convenient interface for doing EC arithmetic, while wrapping
//! `E::Point` and `E::Scalar` which actually implement EC arithmetic. You may want to access
//! wrapped `E::Point` and `E::Scalar` if you lack some functionality that cannot be implemented
//! on top of `Point<E>` and `Scalar<E>`.

use subtle::CtOption;

mod sealed {
    pub trait Sealed {}
    impl<E: crate::Curve> Sealed for crate::Point<E> {}
    impl<E: crate::Curve> Sealed for crate::Scalar<E> {}
}

/// Accesses backend library representation of the point/scalar
pub trait AsRaw
where
    Self: Sized,
{
    /// Wrapped point/scalar
    type Raw;

    /// Returns reference to wrapped value
    fn as_raw(&self) -> &Self::Raw;
}

/// Constructs a point/scalar from its backend library representation
pub trait FromRaw: AsRaw {
    /// Wraps a point/scalar
    ///
    /// Returns `None` if value doesn't meet wrapper constraints
    fn from_raw(raw: Self::Raw) -> Option<Self> {
        Self::ct_from_raw(raw).into()
    }
    /// Wraps a point/scalar (constant time)
    ///
    /// Returns `None` if value doesn't meet wrapper constraints
    fn ct_from_raw(raw: Self::Raw) -> CtOption<Self>;
}
