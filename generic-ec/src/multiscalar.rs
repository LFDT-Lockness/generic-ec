//! Multiscalar multiplication
//!
//! Let $s_{1, \dots, n}$ and $P_{1, \dots, n}$ be lists of scalars and points
//! respectively. Multiscalar multiplication is computing point $Q$ such that:
//!
//! $$Q = s_1 P_1 + \dots + s_n P_n$$
//!
//! This module provides various algorithms for computing multiscalar multiplication
//! efficiently
//!
//! ## Motivation
//! Computing the sum naively, i.e. calculating each $s_i P_i$ separately and
//! $\sum$-ing them, is inefficient even for small $n$.
//!
//! Recall an algorithm for computing $Q = s P$:
//! 1. Let $Q \gets \O, A \gets P$
//! 2. While $s \ne 0$ do the following:
//!    1. If $s$ is odd, set $Q \gets Q + A$
//!    2. Set $s \gets s \gg 1$ (bitwise shift to left by 1 bit)
//!    3. If $s \ne 0$, set $A \gets A + A$
//! 3. Return $Q$
//!
//! This algorithm does 1 point doubling per each bit in $s$ and one addition
//! per each bit in $s$ that's set to `1`. For a random scalar, we can expect
//! that, on average, half of its bits are set to `1`. Thus, cost of the
//! multiplication is:
//!
//! $$\text{cost} = \log_2 s \cdot D + \frac{1}{2} \log_2 s \cdot A$$
//!
//! i.e. it does $\log_2 s$ doublings and $\frac{1}{2} \log_2 s$ additions. Naive
//! multiscalar multiplication algorithm just computes $s_i P_i$ individually
//! and then sums them. Total cost is:
//!
//! $$\text{cost} = n A + n (\log_2 s \cdot D + \frac{1}{2} \log_2 s \cdot A)$$
//!
//! Typically, we're working with scalars of 256 bits size. You can easily calculate
//! that, for instance, even for $n = 3$, the total cost is $768 D + 387 A$. However,
//! for the same $n$, Straus algorithm has $\text{cost} = 300 D + 192 A$ which
//! is already significantly faster than naive algorithm.
//!
//! You can see the perfromance comparison for different algorithms on secp256k1 curve
//! on the plot below:
//!
#![doc = include_str!("../perf/multiscalar/secp256k1.svg")]
//!
//! ## How to use it
//! In most cases, all you need is [`Scalar::multiscalar_mul`] which defaults
//! to the most efficient available algorithm, similarly to [`struct@Default`].
//!
//! Alternatively, if you need to use a specific algorithm, this module provides
//! [`Straus`] and [`Pippenger`].

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
    /// Takes iterator of pairs `(scalar, point)`. Returns sum of `scalar * point`.
    fn multiscalar_mul<S, P>(scalar_points: impl IntoIterator<Item = (S, P)>) -> Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>;
}

/// Defaults to the most efficient multiscalar multiplication algorithm
///
/// When `alloc` feature is off, it always falls back to [`Naive`] implementation.
///
/// When `alloc` feature is on, it chooses the algorithm based on size of input `n`:
/// * [`Straus`] when `n < 50`
/// * [`Pippenger`] otherwise
///
/// It may be more convenient to use [`Scalar::multiscalar_mul`] which is an alias
/// to `Default`.
pub struct Default;

#[cfg(not(feature = "alloc"))]
impl<E: Curve> MultiscalarMul<E> for Default {
    fn multiscalar_mul<S, P>(scalar_points: impl IntoIterator<Item = (S, P)>) -> Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>,
    {
        Naive::multiscalar_mul(scalar_points)
    }
}

#[cfg(feature = "alloc")]
impl<E: Curve> MultiscalarMul<E> for Default {
    fn multiscalar_mul<S, P>(scalar_points: impl IntoIterator<Item = (S, P)>) -> Point<E>
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

/// Multiscalar implementation taken from [`curve25519_dalek`] library
///
/// Only works with [`Ed25519`](crate::curves::Ed25519) curve.
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
