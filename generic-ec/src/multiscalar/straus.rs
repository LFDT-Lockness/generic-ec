use alloc::{vec, vec::Vec};
use core::iter;

use crate::{Curve, Point, Scalar};

/// Straus algorithm
///
/// # How it works
/// Below we'll briefly explain how the algorithm works for better auditability. You can
/// also refer to [original](#credits) implementation.
///
/// Note that algorithm is defined for a parameter $w$, however, in our implementation we hardcoded
/// $w = 5$. It was observed in the benchmarks that $w=5$ gives the best performance for all
/// $n$ (amount of input scalar/point pairs).
///
/// Recall that the multiscalar algorithm takes list of $n$ points $P_1, \dots, P_n$, and a list
/// of $n$ scalars $s_1, \dots, s_n$, and it outputs $Q$ such that:
///
/// $$Q = s_1 P_1 + \dots + s_n P_n$$
///
/// ## Non-Adjacent Form (NAF)
/// Straus algorithm works with scalars in Non-Adjacent Form. Each scalar $s$ is represented
/// as:
///
/// $$s = s_0 2^0 + s_1 2^1 + \dots + s_k 2^k$$
///
/// where $-2^{w-1} \le s_i < 2^{w-1}$, each $s_i$ is either odd or zero, and $k = \log_2 s$ (most
/// commonly, we work with scalars for which $k=256$).
///
/// ## Lookup tables
/// For each point $P_i$, we precompute a lookup table $T_j = j P_i$. We only need to do that
/// for odd $j$ up to $2^{w-1}$. In this way, NAF allows us to cut size of lookup tables by
/// the factor of 4: we reduce size of tables by 2 because NAF has signed terms, and then we
/// also reduce table size twice by working only with odd coefficients.
///
/// ## Computing the sum
///
/// Let's write the full sum that we need to compute:
/// $$
/// \begin{aligned}
/// s_1 P_1 &=&& s_{1,0} P_1 &&+&& 2^1 s_{1,1} P_1 &&+ \dots +&& 2^{k} s_{1,k-1} P_1 \\\\
///    \+   & &&        +    && &&            +    &&         &&                +    \\\\
/// s_2 P_2 &=&& s_{2,0} P_2 &&+&& 2^1 s_{2,1} P_2 &&+ \dots +&& 2^{k} s_{2,k-1} P_2 \\\\
///    \+   & &&        +    && &&            +    &&         &&                +    \\\\
/// \vdots  & && \vdots      && && \vdots          &&         && \vdots              \\\\
///    \+   & &&        +    && &&            +    &&         &&                +    \\\\
/// s_n P_n &=&& s_{n,0} P_n &&+&& 2^1 s_{n,1} P_n &&+ \dots +&& 2^{k-1} s_{n,k-1} P_n
/// \end{aligned}
/// $$
///
/// Note that each $s_{i,j} P_i$ is already computed in a lookup table, and can be replaced with
/// $T_{i, s_{i,j}}$. To compute a sum, we go column-by-column from right to left.
///
/// $$
/// \begin{aligned}
/// Q_k     &= &              &\sum_{i = 0}^n T_{i, s_{i,k}} \\\\
/// Q_{k-1} &= &2 Q_k +       &\sum_{i = 0}^n T_{i, s_{i,k-1}} \\\\
/// \vdots  &  &              &\\\\
/// Q_j     &= &2 Q_{j + 1} + &\sum_{i = 0}^n T_{i, s_{i,j}} \\\\
/// \vdots  &  &              &\\\\
/// Q = Q_0 &= &2 Q_1 +       &\sum_{i = 0}^n T_{i, s_{i,0}} \\\\
/// \end{aligned}
/// $$
///
/// ## Credits
/// Algorithm was adopted from [`curve25519_dalek`] crate, with the modification that
/// it would work with any curve, not only with ed25519. You can find original implementation
/// [here](https://github.com/dalek-cryptography/curve25519-dalek/blob/1efe6a93b176c4389b78e81e52b2cf85d728aac6/curve25519-dalek/src/backend/serial/scalar_mul/straus.rs#L147-L201).
pub struct Straus;

impl<E: Curve> super::MultiscalarMul<E> for Straus {
    fn multiscalar_mul<S, P>(
        scalar_points: impl ExactSizeIterator<Item = (S, P)>,
    ) -> crate::Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>,
    {
        let mut nafs = NafMatrix::new(5, scalar_points.len());
        let lookup_tables: Vec<_> = scalar_points
            .into_iter()
            .map(|(scalar, point)| {
                nafs.add_scalar(scalar.as_ref());
                point
            })
            .map(|point| LookupTable::new(*point.as_ref()))
            .collect();
        if lookup_tables.is_empty() {
            return Point::zero();
        }

        let naf_size = nafs.naf_size;

        let mut r = Point::zero();
        for (i, is_first_iter) in (0..naf_size)
            .rev()
            .zip(iter::once(true).chain(iter::repeat(false)))
        {
            if !is_first_iter {
                r = r.double();
            }
            for (naf, lookup_table) in nafs.iter().zip(&lookup_tables) {
                let naf_i = naf[i];
                match naf_i.cmp(&0) {
                    core::cmp::Ordering::Greater => {
                        r += lookup_table.get(naf_i as usize);
                    }
                    core::cmp::Ordering::Less => {
                        r -= lookup_table.get(-naf_i as usize);
                    }
                    core::cmp::Ordering::Equal => {}
                }
            }
        }
        r
    }
}

struct LookupTable<E: Curve>([Point<E>; 8]);

impl<E: Curve> LookupTable<E> {
    /// Builds a lookup table for point $P$
    fn new(point: Point<E>) -> Self {
        let mut table = [point; 8];
        let point2 = point.double();
        for i in 0..7 {
            table[i + 1] = point2 + table[i];
        }
        Self(table)
    }
    /// Takes odd integer $x$ such as $0 < x < 2^4$, returns $x P$
    fn get(&self, x: usize) -> Point<E> {
        debug_assert_eq!(x & 1, 1);
        debug_assert!(x < 16);

        self.0[x / 2]
    }
}

/// Stores a width-$w$ "Non-Adjacent Form" (NAF) of multiple scalars
///
/// Width-$w$ NAF represents an integer $k$ via coefficients $k_0, \dots, k_n$ such as:
///
/// $$k = \sum_{i=0}^{n} k_i \cdot 2^i$$
///
/// where each $k_i$ is odd and lies within range $-2^{w-1} \le k_i < 2^{w-1}$.
///
/// Non Adjacent Form allows us to reduce size of tables we need to precompute in Straus
/// multiscalar multiplication by factor of 4.
struct NafMatrix<E: Curve> {
    /// Size of one scalar in non adjacent form
    naf_size: usize,
    /// Input parameter `w`
    w: usize,
    /// width = 2^w
    width: u64,
    /// width_half = width / 2
    width_half: u64,
    /// window_mask = width - 1
    window_mask: u64,
    matrix: Vec<i8>,

    _curve: core::marker::PhantomData<E>,
}

impl<E: Curve> NafMatrix<E> {
    /// Construct a new matrix with parameter `w`
    ///
    /// Preallocates memory to fit `capacity` amount of scalars
    fn new(w: usize, capacity: usize) -> Self {
        assert!((2..=8).contains(&w));
        let naf_size = Scalar::<E>::serialized_len() * 8 + 1;
        let width = 1 << w;

        Self {
            naf_size,
            w,
            width,
            width_half: 1 << (w - 1),
            matrix: Vec::with_capacity(naf_size * capacity),
            window_mask: width - 1,
            _curve: Default::default(),
        }
    }
    /// Adds a scalar into matrix
    fn add_scalar(&mut self, scalar: &Scalar<E>) {
        let scalar_bytes = scalar.to_le_bytes();
        let mut x_u64 = vec![0u64; scalar_bytes.len() / 8 + 1];
        read_le_u64_into(&scalar_bytes, &mut x_u64[0..4]);

        let offset = self.matrix.len();
        debug_assert!(
            offset + self.naf_size <= self.matrix.capacity(),
            "unnecessary allocations detected"
        );
        self.matrix.resize(offset + self.naf_size, 0i8);
        let naf = &mut self.matrix[offset..];

        let mut pos = 0;
        let mut carry = false;
        while pos < self.naf_size {
            let u64_idx = pos / 64;
            let bit_idx = pos % 64;
            let bit_buf: u64 = if bit_idx < 64 - self.w {
                // This window bits are contained in a single u64
                (x_u64[u64_idx] >> bit_idx) & self.window_mask
            } else {
                // Combine the current u64's bits with the bits from the next u64
                ((x_u64[u64_idx] >> bit_idx) | (x_u64[u64_idx + 1] << (64 - bit_idx)))
                    & self.window_mask
            };

            // Add the carry into the current window
            let window = if carry { bit_buf + 1 } else { bit_buf };

            if window & 1 == 0 {
                // If the window value is even, preserve the carry and continue.
                // Why is the carry preserved?
                // If carry == 0 and window & 1 == 0, then the next carry should be 0
                // If carry == 1 and window & 1 == 0, then bit_buf & 1 == 1 so the next carry should be 1
                pos += 1;
                continue;
            }

            if window < self.width_half {
                carry = false;
                naf[pos] = window as i8;
            } else {
                carry = true;
                naf[pos] = (window as i8).wrapping_sub(self.width as i8);
            }

            pos += self.w;
        }

        debug_assert!(!carry);
    }

    /// Iterates over scalars NAF representations in the same order as
    /// scalars were added into the matrix
    fn iter(&self) -> impl Iterator<Item = &[i8]> {
        self.matrix.chunks_exact(self.naf_size)
    }
}

/// Read one or more u64s stored as little endian bytes.
///
/// ## Panics
/// Panics if `src.len() != 8 * dst.len()`.
fn read_le_u64_into(src: &[u8], dst: &mut [u64]) {
    assert!(
        src.len() == 8 * dst.len(),
        "src.len() = {}, dst.len() = {}",
        src.len(),
        dst.len()
    );
    for (bytes, val) in src.chunks(8).zip(dst.iter_mut()) {
        *val = u64::from_le_bytes(
            #[allow(clippy::expect_used)]
            bytes
                .try_into()
                .expect("Incorrect src length, should be 8 * dst.len()"),
        );
    }
}

#[cfg(test)]
#[generic_tests::define]
mod tests {
    use alloc::vec::Vec;
    use core::iter;

    use crate::{Curve, Point, Scalar};

    #[test]
    fn non_adjacent_form_is_correct<E: Curve>() {
        let mut rng = rand_dev::DevRng::new();

        let scalars = iter::once(Scalar::<E>::zero())
            .chain(iter::once(Scalar::one()))
            .chain(iter::once(-Scalar::one()))
            .chain(iter::repeat_with(|| Scalar::random(&mut rng)).take(15))
            .collect::<Vec<_>>();

        for w in 2..=8 {
            let mut nafs = super::NafMatrix::new(w, scalars.len());
            scalars.iter().for_each(|scalar| nafs.add_scalar(scalar));

            for (scalar, naf) in scalars.iter().zip(nafs.iter()) {
                std::eprintln!("scalar {scalar:?}");
                std::eprintln!("naf: {naf:?}");

                assert!(naf.iter().all(|&k_i| -(1i16 << (w - 1)) <= i16::from(k_i)
                    && i16::from(k_i) < (1i16 << (w - 1))));

                let expected = naf.iter().rev().fold(Scalar::<E>::zero(), |acc, naf_i| {
                    acc + acc + Scalar::from(*naf_i)
                });
                assert_eq!(*scalar, expected)
            }
        }
    }

    #[test]
    fn lookup_table<E: Curve>() {
        let mut rng = rand_dev::DevRng::new();

        let points = iter::once(Point::<E>::generator().to_point())
            .chain(iter::repeat_with(|| Scalar::random(&mut rng) * Point::generator()).take(50));
        for point in points {
            let table = super::LookupTable::new(point);

            for x in (1..16).step_by(2) {
                assert_eq!(table.get(x), point * Scalar::from(x));
            }
        }
    }

    #[instantiate_tests(<crate::curves::Secp256k1>)]
    mod secp256k1 {}
    #[instantiate_tests(<crate::curves::Secp256r1>)]
    mod secp256r1 {}
    #[instantiate_tests(<crate::curves::Stark>)]
    mod stark {}
    #[instantiate_tests(<crate::curves::Ed25519>)]
    mod ed25519 {}
}
