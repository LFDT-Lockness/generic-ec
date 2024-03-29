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
//! for the same parameters, Straus algorithm has $\text{cost} = 300 D + 192 A$ which
//! is already significantly faster than naive algorithm.

use core::iter;

use crate::{Curve, Point, Radix16Iter, Scalar};

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

/// Straus algorithm
pub struct Straus;

impl<E: Curve> MultiscalarMul<E> for Straus {
    fn multiscalar_mul<S, P>(scalar_points: impl IntoIterator<Item = (S, P)>) -> Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>,
    {
        let (mut scalars, points): (Vec<Radix16Iter<E>>, Vec<Point<E>>) = scalar_points
            .into_iter()
            .map(|(scalar, point)| (scalar.as_ref().as_radix16_be(), *point.as_ref()))
            .unzip();
        if scalars.is_empty() {
            return Point::zero();
        }

        // table[i] = [point_i, 2 * point_i, ..., 15 * point_i]
        let table = points
            .iter()
            .map(|point_i| {
                iter::successors(Some(*point_i), |point| Some(point + point_i))
                    .take(15)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // `serialized_len` is amount of radix256-digits. We multiply it by 2 and get
        // amount of radix16-digits
        let num_digits = 2 * Scalar::<E>::serialized_len();

        let mut sum = Point::<E>::zero();
        for j in 0..num_digits {
            // partial_sum = \sum_i s_{i,k} P_i where `k` is index of the most significant
            // unprocessed coefficient
            let partial_sum = scalars
                .iter_mut()
                .map(|radix16| {
                    radix16
                        .next()
                        .expect("there must be next radix16 available")
                })
                .enumerate()
                .map(|(i, v)| {
                    // We need to calculate `v * P_i`
                    if v == 0 {
                        // P_i * 0 = 0
                        Point::zero()
                    } else {
                        debug_assert!(v < 16);
                        table[i][usize::from(v) - 1]
                    }
                })
                .sum::<Point<E>>();

            if j == 0 {
                sum = partial_sum
            } else {
                // sum = 16 * sum + partial_sum
                let sum_at_16 = sum.double().double().double().double();
                sum = sum_at_16 + partial_sum;
            }
        }

        sum
    }
}

/// Pippenger algorithm
pub struct Pippenger;

impl<E: Curve> MultiscalarMul<E> for Pippenger {
    fn multiscalar_mul<S, P>(scalar_points: impl IntoIterator<Item = (S, P)>) -> Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>,
    {
        let (mut scalars, points): (Vec<Radix16Iter<E>>, Vec<Point<E>>) = scalar_points
            .into_iter()
            .map(|(scalar, point)| (scalar.as_ref().as_radix16_be(), *point.as_ref()))
            .unzip();
        if scalars.is_empty() {
            return Point::zero();
        }

        // `serialized_len` is amount of radix256-digits. We multiply it by 2 and get
        // amount of radix16-digits
        let num_digits = 2 * Scalar::<E>::serialized_len();

        let mut result = Point::zero();
        for i in 0..num_digits {
            let mut buckets = [None; 15];

            // Put each point into its bucket
            scalars
                .iter_mut()
                .map(|radix16| {
                    radix16
                        .next()
                        .expect("there must be next radix16 available")
                })
                .zip(&points)
                .for_each(|(s_ij, &point_i)| {
                    debug_assert!(s_ij < 16);
                    if s_ij == 0 {
                        // P_i * 0 = 0, we ignore it
                        return;
                    } else {
                        match &mut buckets[usize::from(s_ij) - 1] {
                            Some(bucket) => *bucket += point_i,
                            bucket @ None => *bucket = Some(point_i),
                        }
                    }
                });

            // Compute full_sum = 1 * buckets[0] + 2 * buckets[1] + ... + 15 * buckets[14]
            let mut sum = buckets[14].unwrap_or_else(|| Point::zero());
            let mut full_sum = sum;

            for bucket_j in buckets.iter().rev().skip(1) {
                if let Some(bucket_j) = bucket_j {
                    sum += bucket_j;
                }
                full_sum += sum;
            }
            debug_assert_eq!(
                full_sum,
                (1..)
                    .zip(&buckets)
                    .map(|(i, bucket_i)| bucket_i.unwrap_or_default() * Scalar::from(i))
                    .sum::<Point<E>>()
            );

            if i == 0 {
                result = full_sum
            } else {
                let result_at_16 = result.double().double().double().double();
                result = result_at_16 + full_sum
            }
        }

        result
    }
}
