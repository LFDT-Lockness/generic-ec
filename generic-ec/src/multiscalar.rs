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
//! <img src="https://raw.githubusercontent.com/dfns/generic-ec/m/perf/multiscalar/secp256k1.svg"/>
//!
//! ## How to use it
//! In most cases, all you need is [`Scalar::multiscalar_mul`] which defaults
//! to the most efficient available algorithm, similarly to [`struct@Default`].
//!
//! Alternatively, if you need to use a specific algorithm, this module provides
//! [`Straus`] and [`Pippenger`].

use core::iter;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

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
        let (mut scalars, points): (Vec<Radix16Iter<E>>, Vec<Point<E>>) = scalar_points
            .into_iter()
            .map(|(scalar, point)| (scalar.as_ref().as_radix16_be(), *point.as_ref()))
            .unzip();

        if scalars.len() < 50 {
            Straus::multiscalar_mul_inner(&mut scalars, &points)
        } else {
            Pippenger::mutliscalar_mul_inner(&mut scalars, &points)
        }
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

/// Straus algorithm
///
/// Efficient for smaller $n$ up to 50. It has estimated complexity:
///
/// $$\text{cost} = (4D + A)(log_{16} s - 1) + (n - 1) log_{16} s \cdot A + 16 n \cdot D$$
///
/// ## Algorithm
/// **Inputs:** list of $n$ points $P_1, \dots, P_n$ and scalars $s_1, \dots, s_n$. Each
/// scalar is in radix 16 representation: $s_i = s_{i,0} + s_{i,1} 16^1 + \dots + s_{i,k-1} 16^{k-1}$
/// where $k = log_{16} s$, and $0 \le s_{i,j} < 16$
///
/// **Outputs:** $Q = s_1 P_1 + \dots + s_n P_n$
///
/// **Steps:**
///
/// 1. Compute a table $T_i = [\O, P_i, 2 P_i, \dots, 15 P_i]$ for each $1 \le i \le n$
/// 2. Compute $Q_{k-1} = \sum_i P_i s_{i,k-1} = \sum_i T_{i,s_{i,k-1}}$
/// 3. Compute $Q_j = 16 Q_{j+1} + \sum_i T_{i,s_{i,j}}$
/// 4. Output $Q = Q_0$
///
/// ## How it works
/// Recall that each scalar is given in radix 16 representation. The whole sum $s_1 P_1 + \dots + s_n P_n$
/// can be rewritten as:
///
/// $$
/// \begin{aligned}
/// s_1 P_1 &=&& s_{1,0} P_1 &&+&& 16^1 s_{1,1} P_1 &&+ \dots +&& 16^{k-1} s_{1,k-1} P_1 \\\\
///    \+   & &&        +    && &&             +    &&         &&                   +    \\\\
/// s_2 P_2 &=&& s_{2,0} P_2 &&+&& 16^1 s_{2,1} P_2 &&+ \dots +&& 16^{k-1} s_{2,k-1} P_2 \\\\
///    \+   & &&        +    && &&             +    &&         &&                   +    \\\\
/// \vdots  & && \vdots      && && \vdots           &&         && \vdots                 \\\\
///    \+   & &&        +    && &&             +    &&         &&                   +    \\\\
/// s_n P_n &=&& s_{n,0} P_n &&+&& 16^1 s_{n,1} P_n &&+ \dots +&& 16^{k-1} s_{n,k-1} P_n
/// \end{aligned}
/// $$
///
/// Straus algorithm computes the sum column by column from right to left, multiplying result
/// by 16 after each column sum is computed. Also, it uses the precomputed table
/// $T_i = [\O, P_i, 2 P_i, \dots, 15 P_i]$ to optimize multiplication $s_{i,j} P_i$.
///
/// Transformed sum that's computed by Straus algorithm looks like this:
///
/// $$
/// \begin{aligned}
/// Q_{k-1} &=             & T_{1,s_{1,k-1}} &+ \dots &+ T_{n,s_{n,k-1}}& \\\\
/// Q_{k-2} &= 16 Q_{k-1} +& T_{1,s_{1,k-2}} &+ \dots &+ T_{n,s_{n,k-2}}& \\\\
/// \vdots  &              &                 &        &                 & \\\\
/// Q_1     &= 16 Q_2     +& T_{1,s_{1,1}}   &+ \dots &+ T_{n,s_{n,1}}  & \\\\
/// Q = Q_0 &= 16 Q_1     +& T_{1,s_{1,0}}   &+ \dots &+ T_{n,s_{n,0}}  &
/// \end{aligned}
/// $$
#[cfg(feature = "alloc")]
pub struct Straus;

#[cfg(feature = "alloc")]
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
        Self::multiscalar_mul_inner(&mut scalars, &points)
    }
}

#[cfg(feature = "alloc")]
impl Straus {
    fn multiscalar_mul_inner<E: Curve>(
        scalars: &mut [Radix16Iter<E>],
        points: &[Point<E>],
    ) -> Point<E> {
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
                    #[allow(clippy::expect_used)]
                    radix16
                        .next()
                        .expect("there must be next radix16 available")
                })
                .enumerate()
                .flat_map(|(i, v)| {
                    // We need to calculate `v * P_i`
                    if v == 0 {
                        // P_i * 0 = 0, we don't include it into the sum
                        None
                    } else {
                        debug_assert!(v < 16);
                        Some(table[i][usize::from(v) - 1])
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
///
/// Outperforms [`Straus`] algorithm on $n \ge 50$. It has estimated performance:
///
/// $$\text{cost} = (4D + A)(log_{16} s - 1) + (n A + 30 A) log_{16} s$$
///
/// ## Algorithm
/// **Inputs:** list of $n$ points $P_1, \dots, P_n$ and scalars $s_1, \dots, s_n$. Each
/// scalar is in radix 16 representation: $s_i = s_{i,0} + s_{i,1} 16^1 + \dots + s_{i,k-1} 16^{k-1}$
/// where $k = log_{16} s$, and $0 \le s_{i,j} < 16$
///
/// **Outputs:** $Q = s_1 P_1 + \dots + s_n P_n$
///
/// **Steps:**
///
/// Set $Q = \O$. For each $j \in [k-1, k-2, \dots, 1, 0]$:
/// 1. Let $B$ be list of points indexed from $1$ to $15$, each element of which is
///    initially set to $\O$, i.e. $B_1 = B_2 = \dots = B_{15} = \O$
/// 2. Sort points $P_{1..n}$ into buckets $B$: for each $1 \le i \le n$, set
///    $B_{s_{i,j}} := B_{s_{i,j}} + P_i$
/// 3. Compute $S = 1 B_1 + 2 B_2 + \dots + 15 B_{15}$. To do this efficiently:
///    1. Set $S' := B_{15}$, $S := S'$
///    2. Then, for each $m \in [14, 13, \dots, 1]$, do: \
///       Set $S' := S' + B_m$, and $S := S + S'$
/// 4. Set $Q := 16 Q + S$
///
/// Output $Q$
///
/// ## How it works
/// Similarly to [`Straus`] algorithm, here we work with 2-dimention sum:
///
/// $$
/// \begin{aligned}
/// s_1 P_1 &=&& s_{1,0} P_1 &&+&& 16^1 s_{1,1} P_1 &&+ \dots +&& 16^{k-1} s_{1,k-1} P_1 \\\\
///    \+   & &&        +    && &&             +    &&         &&                   +    \\\\
/// s_2 P_2 &=&& s_{2,0} P_2 &&+&& 16^1 s_{2,1} P_2 &&+ \dots +&& 16^{k-1} s_{2,k-1} P_2 \\\\
///    \+   & &&        +    && &&             +    &&         &&                   +    \\\\
/// \vdots  & && \vdots      && && \vdots           &&         && \vdots                 \\\\
///    \+   & &&        +    && &&             +    &&         &&                   +    \\\\
/// s_n P_n &=&& s_{n,0} P_n &&+&& 16^1 s_{n,1} P_n &&+ \dots +&& 16^{k-1} s_{n,k-1} P_n
/// \end{aligned}
/// $$
///
/// We compute the sum column by column from right to left. The difference is how
/// $s_{1,j} P_1 + s_{2,j} P_2 + \dots + s_{n,j} P_n$ is computed: Pippenger algorithm uses
/// the buckets trick as described above.
#[cfg(feature = "alloc")]
pub struct Pippenger;

#[cfg(feature = "alloc")]
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
        Self::mutliscalar_mul_inner(&mut scalars, &points)
    }
}

#[cfg(feature = "alloc")]
impl Pippenger {
    /// Takes `scalars` represented in big-endian radix 16, `points`, and performs mulitscalar
    /// multiplication, returns `sum = scalars[0] * points[0] + ... + scalars[n-1] * points[n-1]`
    ///
    /// Requires that `scalars.len() == points.len()`
    fn mutliscalar_mul_inner<E: Curve>(
        scalars: &mut [Radix16Iter<E>],
        points: &[Point<E>],
    ) -> Point<E> {
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
                    #[allow(clippy::expect_used)]
                    radix16
                        .next()
                        .expect("there must be next radix16 available")
                })
                .zip(points)
                .for_each(|(s_ij, &point_i)| {
                    debug_assert!(s_ij < 16);
                    if s_ij == 0 {
                        // P_i * 0 = 0, we ignore it
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
