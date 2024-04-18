use alloc::{vec, vec::Vec};
use core::iter;

use crate::{Curve, Point, Scalar};

/// Straus algorithm v2
pub struct StrausV2;

impl<E: Curve> super::MultiscalarMul<E> for StrausV2 {
    fn multiscalar_mul<S, P>(scalar_points: impl IntoIterator<Item = (S, P)>) -> crate::Point<E>
    where
        S: AsRef<Scalar<E>>,
        P: AsRef<Point<E>>,
    {
        let (nafs, lookup_tables): (Vec<_>, Vec<_>) = scalar_points
            .into_iter()
            .map(|(scalar, point)| {
                (
                    non_adjacent_form(5, scalar.as_ref()),
                    LookupTable::new(*point.as_ref()),
                )
            })
            .unzip();
        if nafs.is_empty() {
            return Point::zero();
        }

        let naf_size = nafs[0].len();
        debug_assert!(nafs.iter().all(|naf| naf.len() == naf_size));

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

/// Computes a width-$w$ "Non-Adjacent Form" of the scalar
///
/// Takes an integer $k$ and parameter $w$, returns coefficients $k_0, \dots, k_n$ such as:
///
/// $$k = \sum_{i=0}^{n} k_i \cdot 2^i$$
///
/// where each $k_i$ is odd and lies within range $-2^{w-1} \le k_i < 2^{w-1}$.
///
/// Non Adjacent Form allows us to reduce size of tables we need to precompute in Straus
/// multiscalar multiplication by 4.
///
/// Always returns the same amount of coefficients for scalars of the same curve `E`.
fn non_adjacent_form<E: Curve>(w: usize, scalar: &Scalar<E>) -> Vec<i8> {
    assert!((2..=8).contains(&w));

    let scalar_bytes = scalar.to_le_bytes();
    let mut x_u64 = vec![0u64; scalar_bytes.len() / 8 + 1];
    read_le_u64_into(&scalar_bytes, &mut x_u64[0..4]);

    let naf_size = scalar_bytes.len() * 8 + 1;
    let mut naf = vec![0i8; naf_size];

    let width = 1 << w;
    let width_half = 1 << (w - 1);
    let window_mask = width - 1;

    let mut pos = 0;
    let mut carry = false;
    while pos < naf_size {
        let u64_idx = pos / 64;
        let bit_idx = pos % 64;
        let bit_buf: u64 = if bit_idx < 64 - w {
            // This window bits are contained in a single u64
            (x_u64[u64_idx] >> bit_idx) & window_mask
        } else {
            // Combine the current u64's bits with the bits from the next u64
            ((x_u64[u64_idx] >> bit_idx) | (x_u64[u64_idx + 1] << (64 - bit_idx))) & window_mask
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

        if window < width_half {
            carry = false;
            naf[pos] = window as i8;
        } else {
            carry = true;
            naf[pos] = (window as i8).wrapping_sub(width as i8);
        }

        pos += w;
    }

    debug_assert!(!carry);

    naf
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
    use core::iter;

    use crate::{Curve, Point, Scalar};

    #[test]
    fn non_adjacent_form_is_correct<E: Curve>() {
        let mut rng = rand_dev::DevRng::new();

        let scalars = iter::once(Scalar::<E>::zero())
            .chain(iter::once(Scalar::one()))
            .chain(iter::once(-Scalar::one()))
            .chain(iter::repeat_with(|| Scalar::random(&mut rng)).take(15));
        for scalar in scalars {
            std::eprintln!("scalar {scalar:?}");

            for w in 2..=8 {
                let naf = super::non_adjacent_form(w, &scalar);
                std::eprintln!("naf: {naf:?}");
                assert!(naf.iter().all(|&k_i| -(1i16 << (w - 1)) <= i16::from(k_i)
                    && i16::from(k_i) < (1i16 << (w - 1))));

                let expected = naf.iter().rev().fold(Scalar::<E>::zero(), |acc, naf_i| {
                    acc + acc + Scalar::from(*naf_i)
                });
                assert_eq!(scalar, expected)
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
