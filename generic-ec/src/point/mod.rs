use core::fmt;
use core::hash::{self, Hash};
use core::iter::Sum;

use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::{
    as_raw::{AsRaw, TryFromRaw},
    core::*,
    errors::InvalidPoint,
    EncodedPoint, Generator,
};

use self::definition::Point;

pub mod coords;
pub mod definition;

impl<E: Curve> Point<E> {
    /// Curve generator
    ///
    /// Curve generator is a regular point defined in curve specs. See [`Generator<E>`](Generator).
    pub fn generator() -> Generator<E> {
        Generator::default()
    }

    /// Returns identity point $\O$ (sometimes called as _point at infinity_)
    ///
    /// Identity point has special properties:
    ///
    /// $$\forall P \in \G: P + \O = P$$
    /// $$\forall s \in \Zq: s \cdot \O = \O$$
    ///
    /// When you validate input from user or message received on wire, you should bear in mind that
    /// any `Point<E>` may be zero. If your algorithm does not accept identity points, you may check
    /// whether point is zero by calling [`.is_zero()`](Point::is_zero). Alternatively, you may accept
    /// [`NonZero<Point<E>>`](crate::NonZero) instead, which is guaranteed to be non zero.
    pub fn zero() -> Self {
        // Correctness:
        // 1. Zero point belongs to curve by definition
        // 2. Zero point is free of any component (including torsion component)
        Self::from_raw_unchecked(E::Point::zero())
    }

    /// Indicates whether it's [identity point](Self::zero)
    ///
    /// ```rust
    /// use generic_ec::{Point, curves::Secp256k1};
    ///
    /// assert!(Point::<Secp256k1>::zero().is_zero());
    /// assert!(!Point::<Secp256k1>::generator().to_point().is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.ct_is_zero().into()
    }

    /// Indicates whether it's [identity point](Self::zero) (in constant time)
    ///
    /// Same as [`.is_zero()`](Self::is_zero) but performs constant-time comparison.
    pub fn ct_is_zero(&self) -> Choice {
        Zero::is_zero(self.as_raw())
    }

    /// Encodes a point as bytes
    ///
    /// Function can return both compressed and uncompressed bytes representation of a point.
    /// Compressed bytes representation is more compact, but parsing takes a little bit more
    /// time. On other hand, uncompressed representation takes ~twice more space, but parsing
    /// is instant.
    ///
    /// For some curves, `compressed` parameter may be ignored, and same bytes representation
    /// is returned.
    ///
    /// ```rust
    /// use generic_ec::{Point, Scalar, curves::Secp256k1};
    /// use rand::rngs::OsRng;
    ///
    /// let random_point = Point::<Secp256k1>::generator() * Scalar::random(&mut OsRng);
    /// let point_bytes = random_point.to_bytes(false);
    /// let point_decoded = Point::from_bytes(&point_bytes)?;
    /// assert_eq!(random_point, point_decoded);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_bytes(&self, compressed: bool) -> EncodedPoint<E> {
        if compressed {
            let bytes = self.as_raw().to_bytes_compressed();
            EncodedPoint::new_compressed(bytes)
        } else {
            let bytes = self.as_raw().to_bytes_uncompressed();
            EncodedPoint::new_uncompressed(bytes)
        }
    }

    /// Decodes a point from bytes
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, InvalidPoint> {
        E::Point::decode(bytes.as_ref())
            .and_then(Self::try_from_raw)
            .ok_or(InvalidPoint)
    }
}

impl<E: Curve> TryFromRaw for Point<E> {
    fn ct_try_from_raw(point: E::Point) -> CtOption<Self> {
        let is_on_curve = point.is_on_curve();
        let is_torsion_free = point.is_torsion_free();
        let is_valid = is_on_curve & is_torsion_free;

        // Correctness: we checked validity of the point. Although invalid point
        // is still given to `from_raw_unchecked`, it's never exposed by CtOption,
        // so no one can obtain "invalid" instance of `Point`.
        CtOption::new(Point::from_raw_unchecked(point), is_valid)
    }
}

impl<E: Curve> ConditionallySelectable for Point<E> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        // Correctness: both `a` and `b` have to be valid points by construction
        Point::from_raw_unchecked(<E::Point as ConditionallySelectable>::conditional_select(
            a.as_raw(),
            b.as_raw(),
            choice,
        ))
    }
}

impl<E: Curve> ConstantTimeEq for Point<E> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_raw().ct_eq(other.as_raw())
    }
}

impl<E: Curve> AsRef<Point<E>> for Point<E> {
    fn as_ref(&self) -> &Point<E> {
        self
    }
}

impl<E: Curve> Sum for Point<E> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Point::zero(), |acc, p| acc + p)
    }
}

impl<'a, E: Curve> Sum<&'a Point<E>> for Point<E> {
    fn sum<I: Iterator<Item = &'a Point<E>>>(iter: I) -> Self {
        iter.fold(Point::zero(), |acc, p| acc + p)
    }
}

impl<E: Curve> fmt::Debug for Point<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Point");
        s.field("curve", &E::CURVE_NAME);

        #[cfg(feature = "std")]
        {
            s.field("value", &hex::encode(self.to_bytes(true)));
        }
        #[cfg(not(feature = "std"))]
        {
            s.field("value", &"...");
        }

        s.finish()
    }
}
#[allow(clippy::derive_hash_xor_eq)]
impl<E: Curve> Hash for Point<E> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(self.to_bytes(true).as_bytes())
    }
}

impl<E: Curve> PartialOrd for Point<E> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.to_bytes(true)
            .as_bytes()
            .partial_cmp(other.to_bytes(true).as_bytes())
    }
}

impl<E: Curve> Ord for Point<E> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_bytes(true)
            .as_bytes()
            .cmp(other.to_bytes(true).as_bytes())
    }
}

impl<E: Curve> crate::traits::IsZero for Point<E> {
    fn is_zero(&self) -> bool {
        *self == Point::zero()
    }
}

impl<E: Curve> crate::traits::Zero for Point<E> {
    fn zero() -> Self {
        Point::zero()
    }

    fn is_zero(x: &Self) -> Choice {
        x.ct_eq(&Self::zero())
    }
}
