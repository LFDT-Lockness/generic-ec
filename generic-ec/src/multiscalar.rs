//! Multiscalar multiplication
//!
//! Let $s_{1, \dots, n}$ and $P_{1, \dots, n}$ be lists of scalars and points
//! respectively. Multiscalar multiplication is computing point $Q$ such that:
//!
//! $$Q = s_1 P_1 + \dots + s_n P_n$$
//!
//! This module provides various algorithms for computing multiscalar multiplication
//! efficiently.
//!
//! ## Performance
//! Computing the sum naively, i.e. calculating each $s_i P_i$ separately and
//! $\sum$-ing them, is inefficient even for small $n$. You can see that in the
//! comparison below.
//!
#![doc = include_str!("../perf/multiscalar/secp256k1.svg")]
//!
//! ## How to use it
//! In most cases, all you need is [`Scalar::multiscalar_mul`] which defaults
//! to the most efficient available algorithm, similarly to [`struct@Default`].
//!
//! Alternatively, if you need to use a specific algorithm, this module provides
//! [`Straus`] and [`Dalek`].
//!
//! On [`Ed25519`](crate::curves::Ed25519) curve, consider using [`Dalek`] multiscalar
//! implementation.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::{Curve, Point, Scalar};

#[cfg(feature = "alloc")]
mod straus;

#[cfg(feature = "alloc")]
pub use self::straus::Straus;

/// Multiscalar multiplication algorithm
///
/// See [module-level docs](self) for motivation and list of provided algorithms.
pub trait MultiscalarMul<E: Curve> {
    /// Performs multiscalar multiplication
    ///
    /// Takes iterator of pairs `(scalar, point)`. Returns sum of `scalar * point`. Iterator must have
    /// exact size (i.e. it's [`ExactSizeIterator`]). Iterator size is used to determine the best
    /// algorithm for multiscalar multiplication, preallocate memory, etc. If iterator size is not
    /// correct, it may worsen performance or lead to runtime panic.
    ///
    /// Note that the multiscalar algorithm is not necessarily constant-time, thus is should not be
    /// used with [`SecretScalar<E>`](crate::SecretScalar).
    fn multiscalar_mul<S, P>(scalar_points: impl ExactSizeIterator<Item = (S, P)>) -> Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>;
}

/// Defaults to the most efficient multiscalar multiplication algorithm
///
/// When `alloc` feature is off, it always falls back to [`Naive`] implementation.
///
/// When `alloc` feature is on, it uses [`Straus`] algorithm.
///
/// It may be more convenient to use [`Scalar::multiscalar_mul`] which is an alias
/// to `Default`.
pub struct Default;

#[cfg(not(feature = "alloc"))]
impl<E: Curve> MultiscalarMul<E> for Default {
    fn multiscalar_mul<S, P>(scalar_points: impl ExactSizeIterator<Item = (S, P)>) -> Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>,
    {
        Naive::multiscalar_mul(scalar_points)
    }
}

#[cfg(feature = "alloc")]
impl<E: Curve> MultiscalarMul<E> for Default {
    fn multiscalar_mul<S, P>(scalar_points: impl ExactSizeIterator<Item = (S, P)>) -> Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>,
    {
        Straus::multiscalar_mul(scalar_points)
    }
}

/// Naive algorithm
///
/// Computes multiscalar multiplication naively, by calculating each $s_i P_i$ separately,
/// and $\sum$-ing them.
///
/// Complexity:
///
/// $$\text{cost} = \log_2 s \cdot D + \frac{1}{2} \log_2 s \cdot A$$
pub struct Naive;

impl<E: Curve> MultiscalarMul<E> for Naive {
    fn multiscalar_mul<S, P>(scalar_points: impl IntoIterator<Item = (S, P)>) -> Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>,
    {
        scalar_points
            .into_iter()
            .map(|(scalar, point)| scalar.as_ref() * point.as_ref())
            .sum()
    }
}

/// Multiscalar implementation for [`Ed25519`] curve
///
/// [`curve25519_dalek`] library provides multiscalar multiplication algorithm which only
/// works with [`Ed25519`] curve. Due to the fact that it's specifically instantiated for
/// the only one curve, this implementation is more efficient than generic [`struct@Default`]
/// or [`Straus`].
///
#[doc = include_str!("../perf/multiscalar/ed25519.svg")]
///
/// [`Ed25519`]: crate::curves::Ed25519
#[cfg(all(feature = "curve-ed25519", feature = "alloc"))]
pub struct Dalek;

#[cfg(all(feature = "curve-ed25519", feature = "alloc"))]
impl MultiscalarMul<crate::curves::Ed25519> for Dalek {
    fn multiscalar_mul<S, P>(
        scalar_points: impl IntoIterator<Item = (S, P)>,
    ) -> Point<crate::curves::Ed25519>
    where
        S: AsRef<Scalar<crate::curves::Ed25519>>,
        P: AsRef<Point<crate::curves::Ed25519>>,
    {
        use curve25519_dalek::traits::VartimeMultiscalarMul;
        use generic_ec_core::{OnCurve, SmallFactor};

        use crate::as_raw::AsRaw;

        let (scalars, points): (Vec<_>, Vec<_>) = scalar_points
            .into_iter()
            .map(|(s, p)| (s.as_ref().as_raw().0, p.as_ref().as_raw().0))
            .unzip();

        let result = curve25519_dalek::EdwardsPoint::vartime_multiscalar_mul(scalars, points);
        let result = generic_ec_curves::ed25519::Point(result);

        // Resulting point must be valid
        debug_assert!(result.is_on_curve().into() && result.is_torsion_free().into());
        Point::from_raw_unchecked(result)
    }
}
