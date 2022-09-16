use core::ops::{Add, Mul, Neg, Sub};

use crate::{Curve, Point, Scalar};

mod laws {
    use crate::ec_core::*;
    use crate::{Point, Scalar};

    /// If $A$ and $B$ are valid `Point<E>`, then $A + B$ is a valid `Point<E>`
    ///
    /// For `Point<E>` to be valid it needs to meet two conditions:
    /// 1. It has to be on curve
    /// 2. It has to be free of torison component
    ///
    /// Sum of two points on curve is always a point on curve by definition, so (1) holds.
    ///
    /// Recall that, generally, any point on elliptic curve can be represented as sum of its
    /// components:
    ///
    /// $$P = p_0 \G + p_1 \T_1 + \dots + p_t \T_t$$
    ///
    /// where $\G$ is a group of large prime order, and $\T_{1,\dots,t}$ are torison small groups.
    /// Then sum of two points can be represented as:
    ///
    /// $$A + B = (a_0 + b_0) \G + (a_1 + b_1) \T_1 + \dots + (a_t + b_t) \T_t$$
    ///
    /// $A$ and $B$ are valid `Point<E>`, so they are torison free, which means that
    /// $a_{1,\dots,t} = b_{1,\dots,t} = 0$, so their sum is also torison free:
    ///
    /// $$A + B = (a_0 + b_0) \G$$
    ///
    /// Therefore, (2) holds.
    #[inline]
    pub fn sum_of_points_is_valid_point<E: Curve>(a: &Point<E>, b: &Point<E>) -> Point<E> {
        let sum = Additive::add(a.as_raw(), b.as_raw());
        // Correctness: refer to doc comment of the function
        Point::from_raw_unchecked(sum)
    }

    /// If $A$ and $B$ are valid `Point<E>`, then $A - B$ are valid `Point<E>`
    ///
    /// Please, refer to [`sum_of_points_is_valid_point`], as the proof is pretty much the same.
    #[inline]
    pub fn sub_of_points_is_valid_point<E: Curve>(a: &Point<E>, b: &Point<E>) -> Point<E> {
        let result = Additive::sub(a.as_raw(), b.as_raw());
        // Correctness: refer to doc comment of the function
        Point::from_raw_unchecked(result)
    }

    /// If $A$ is a valid `Point<E>`, then $-A$ is a valid `Point<E>`
    ///
    /// [`sub_of_points_is_valid_point`] proves that subtraction of two valid `Point<E>` is a
    /// valid `Point<E>`, so $O - A$ is also a valid `Point<E>`
    #[inline]
    pub fn neg_point_is_valid_point<E: Curve>(a: &Point<E>) -> Point<E> {
        let neg = Additive::negate(a.as_raw());
        // Correctness: refer to doc comment of the function
        Point::from_raw_unchecked(neg)
    }

    /// If $n$ is valid `Scalar<E>` and $A$ is valid `Point<E>`, then $n A$ is a valid `Point<E>`
    ///
    /// For `Point<E>` to be valid it needs to meet two conditions:
    /// 1. It has to be on curve
    /// 2. It has to be free of torison component
    ///
    /// Point on curve multiplied at any integer is always a point on curve by definition, so
    /// (1) holds.
    ///
    /// Recall that, generally, any point on elliptic curve can be represented as sum of its
    /// components:
    ///
    /// $$P = p_0 \G + p_1 \T_1 + \dots + p_t \T_t$$
    ///
    /// where $\G$ is a group of large prime order, and $\T_{1,\dots,t}$ are torison small groups.
    /// Then multiplication of point at scalar can be represented as:
    ///
    /// $$nA = n a_0 \G + n a_1 \T_1 + \dots + n a_t \T_t$$
    ///
    /// $A$ is valid `Point<E>`, so it is torison free, which means that $a_{1,\dots,t} = 0$, so
    /// resulting point is also torison free:
    ///
    /// $$nA = n a_0 \G$$
    ///
    /// Therefore, (2) holds.
    #[inline]
    pub fn mul_of_scalar_at_point_is_valid_point<E: Curve>(
        n: &Scalar<E>,
        a: &Point<E>,
    ) -> Point<E> {
        let prod = Multiplicative::mul(n.as_raw(), a.as_raw());
        // Correctness: refer to doc comment of the function
        Point::from_raw_unchecked(prod)
    }

    /// Same as [`mul_of_scalar_at_point_is_valid_point`] but flipped arguments
    #[inline]
    pub fn mul_of_point_at_scalar_is_valid_point<E: Curve>(
        a: &Point<E>,
        b: &Scalar<E>,
    ) -> Point<E> {
        mul_of_scalar_at_point_is_valid_point(b, a)
    }
}

mod scalar {
    use crate::ec_core::*;
    use crate::Scalar;

    #[inline]
    pub fn add<E: Curve>(a: &Scalar<E>, b: &Scalar<E>) -> Scalar<E> {
        let sum = Additive::add(a.as_raw(), b.as_raw()).reduce();
        // Correctness: `sum` is reduced
        Scalar::from_raw_unchecked(sum)
    }

    #[inline]
    pub fn sub<E: Curve>(a: &Scalar<E>, b: &Scalar<E>) -> Scalar<E> {
        let result = Additive::sub(a.as_raw(), b.as_raw()).reduce();
        // Correctness: `result` is reduced
        Scalar::from_raw_unchecked(result)
    }

    #[inline]
    pub fn mul<E: Curve>(a: &Scalar<E>, b: &Scalar<E>) -> Scalar<E> {
        let prod = Multiplicative::mul(a.as_raw(), b.as_raw()).reduce();
        // Correctness: `prod` is reduced
        Scalar::from_raw_unchecked(prod)
    }

    #[inline]
    pub fn neg<E: Curve>(a: &Scalar<E>) -> Scalar<E> {
        let result = Additive::negate(a.as_raw()).reduce();
        // Correctness: `result` is reduced
        Scalar::from_raw_unchecked(result)
    }
}

macro_rules! impl_binary_ops {
    ($($op:ident ($lhs:ident $op_fn:ident $rhs:ident = $out:ident) $impl_fn:path),+,) => {$(
        impl<E: Curve> $op<$rhs<E>> for $lhs<E> {
            type Output = $out<E>;
            #[inline]
            fn $op_fn(self, rhs: $rhs<E>) -> Self::Output {
                $impl_fn(&self, &rhs)
            }
        }
        impl<E: Curve> $op<&$rhs<E>> for $lhs<E> {
            type Output = $out<E>;
            #[inline]
            fn $op_fn(self, rhs: &$rhs<E>) -> Self::Output {
                $impl_fn(&self, rhs)
            }
        }
        impl<E: Curve> $op<$rhs<E>> for &$lhs<E> {
            type Output = $out<E>;
            #[inline]
            fn $op_fn(self, rhs: $rhs<E>) -> Self::Output {
                $impl_fn(self, &rhs)
            }
        }
        impl<E: Curve> $op<&$rhs<E>> for &$lhs<E> {
            type Output = $out<E>;
            #[inline]
            fn $op_fn(self, rhs: &$rhs<E>) -> Self::Output {
                $impl_fn(self, rhs)
            }
        }
    )+};
}

macro_rules! impl_unary_ops {
    ($($op:ident $op_fn:ident $ty:ident $impl_fn:path),*,) => {$(
        impl<E: Curve> $op for $ty<E> {
            type Output = $ty<E>;
            #[inline]
            fn $op_fn(self) -> Self::Output {
                $impl_fn(&self)
            }
        }
        impl<E: Curve> $op for &$ty<E> {
            type Output = $ty<E>;
            #[inline]
            fn $op_fn(self) -> Self::Output {
                $impl_fn(self)
            }
        }
    )*};
}

impl_binary_ops! {
    Add (Point add Point = Point) laws::sum_of_points_is_valid_point,
    Sub (Point sub Point = Point) laws::sub_of_points_is_valid_point,

    Add (Scalar add Scalar = Scalar) scalar::add,
    Sub (Scalar sub Scalar = Scalar) scalar::sub,
    Mul (Scalar mul Scalar = Scalar) scalar::mul,

    Mul (Point mul Scalar = Point) laws::mul_of_point_at_scalar_is_valid_point,
    Mul (Scalar mul Point = Point) laws::mul_of_scalar_at_point_is_valid_point,
}

impl_unary_ops! {
    Neg neg Point laws::neg_point_is_valid_point,
    Neg neg Scalar scalar::neg,
}
