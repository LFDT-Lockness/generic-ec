use core::ops::{Add, Mul, Neg, Sub};

use crate::{Curve, NonZero, Point, Scalar};

mod laws {
    use crate::{ec_core::*, NonZero};
    use crate::{Point, Scalar};

    /// If $A$ and $B$ are valid `Point<E>`, then $A + B$ is a valid `Point<E>`
    ///
    /// For `Point<E>` to be valid it needs to meet two conditions:
    /// 1. It has to be on curve
    /// 2. It has to be free of torsion component
    ///
    /// Sum of two points on curve is always a point on curve by definition, so (1) holds.
    ///
    /// Recall that, generally, any point on elliptic curve can be represented as sum of its
    /// components:
    ///
    /// $$P = p_0 \G + p_1 \T_1 + \dots + p_t \T_t$$
    ///
    /// where $\G$ is a group of large prime order, and $\T_{1,\dots,t}$ are torsion small groups.
    /// Then sum of two points can be represented as:
    ///
    /// $$A + B = (a_0 + b_0) \G + (a_1 + b_1) \T_1 + \dots + (a_t + b_t) \T_t$$
    ///
    /// $A$ and $B$ are valid `Point<E>`, so they are torsion free, which means that
    /// $a_{1,\dots,t} = b_{1,\dots,t} = 0$, so their sum is also torsion free:
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
    /// 2. It has to be free of torsion component
    ///
    /// Point on curve multiplied at any integer is always a point on curve by definition, so
    /// (1) holds.
    ///
    /// Recall that, generally, any point on elliptic curve can be represented as sum of its
    /// components:
    ///
    /// $$P = p_0 \G + p_1 \T_1 + \dots + p_t \T_t$$
    ///
    /// where $\G$ is a group of large prime order, and $\T_{1,\dots,t}$ are torsion small groups.
    /// Then multiplication of point at scalar can be represented as:
    ///
    /// $$nA = n a_0 \G + n a_1 \T_1 + \dots + n a_t \T_t$$
    ///
    /// $A$ is valid `Point<E>`, so it is torsion free, which means that $a_{1,\dots,t} = 0$, so
    /// resulting point is also torsion free:
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

    /// If $n$ is valid `NonZero<Scalar<E>>` and $A$ is valid `NonZero<Point<E>>`, then $n A$ is a valid `NonZero<Point<E>>`
    ///
    /// As shown in [`mul_of_scalar_at_point_is_valid_point`], $n A$ is a valid `Point<E>`.
    ///
    /// Since $A$ is free of torsion component and non zero, it has order equal to curve `group_order`,
    /// which means (be definition):
    ///
    /// $$\forall n' < \mathit{group\\_order}: n' A \ne O$$
    ///
    /// As $n$ is valid `Scalar<E>`, it's less than curve `group_order`, therefore $n A \ne O$.
    #[inline]
    pub fn mul_of_nonzero_scalar_at_nonzero_point_is_valid_nonzero_point<E: Curve>(
        n: &NonZero<Scalar<E>>,
        a: &NonZero<Point<E>>,
    ) -> NonZero<Point<E>> {
        let prod = mul_of_scalar_at_point_is_valid_point(n, a);
        // Correctness: refer to doc comment of the function
        NonZero::new_unchecked(prod)
    }

    /// Same as [`mul_of_nonzero_scalar_at_nonzero_point_is_valid_nonzero_point`] but flipped arguments
    #[inline]
    pub fn mul_of_nonzero_point_at_nonzero_scalar_is_valid_nonzero_point<E: Curve>(
        a: &NonZero<Point<E>>,
        n: &NonZero<Scalar<E>>,
    ) -> NonZero<Point<E>> {
        mul_of_nonzero_scalar_at_nonzero_point_is_valid_nonzero_point(n, a)
    }

    /// If $A$ is valid `NonZero<Point<E>>`, then $-A$ is valid `NonZero<Point<E>>`
    ///
    /// As shown in [`neg_point_is_valid_point`], $-A$ is a valid `Point<E>`.
    ///
    /// Since $A$ is not zero, $-A$ is not zero as well.
    #[inline]
    pub fn neg_nonzero_point_is_nonzero_point<E: Curve>(
        a: &NonZero<Point<E>>,
    ) -> NonZero<Point<E>> {
        let neg = neg_point_is_valid_point(&a);
        NonZero::new_unchecked(neg)
    }
}

mod scalar {
    use crate::ec_core::*;
    use crate::NonZero;
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

    #[inline]
    pub fn neg_nonzero<E: Curve>(a: &NonZero<Scalar<E>>) -> NonZero<Scalar<E>> {
        let neg = neg(&a);
        // Correctness: since `a` is not zero, `-a` is not zero by definition
        NonZero::new_unchecked(neg)
    }
}

macro_rules! impl_binary_ops {
    ($($op:ident ($lhs:ty, $op_fn:ident, $rhs:ty = $out:ty) $impl_fn:path),+,) => {$(
        impl<E: Curve> $op<$rhs> for $lhs {
            type Output = $out;
            #[inline]
            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $impl_fn(&self, &rhs)
            }
        }
        impl<E: Curve> $op<&$rhs> for $lhs {
            type Output = $out;
            #[inline]
            fn $op_fn(self, rhs: &$rhs) -> Self::Output {
                $impl_fn(&self, rhs)
            }
        }
        impl<E: Curve> $op<$rhs> for &$lhs {
            type Output = $out;
            #[inline]
            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $impl_fn(self, &rhs)
            }
        }
        impl<E: Curve> $op<&$rhs> for &$lhs {
            type Output = $out;
            #[inline]
            fn $op_fn(self, rhs: &$rhs) -> Self::Output {
                $impl_fn(self, rhs)
            }
        }
    )+};
}

macro_rules! impl_unary_ops {
    ($($op:ident ($op_fn:ident $ty:ty) $impl_fn:path),*,) => {$(
        impl<E: Curve> $op for $ty {
            type Output = $ty;
            #[inline]
            fn $op_fn(self) -> Self::Output {
                $impl_fn(&self)
            }
        }
        impl<E: Curve> $op for &$ty {
            type Output = $ty;
            #[inline]
            fn $op_fn(self) -> Self::Output {
                $impl_fn(self)
            }
        }
    )*};
}

impl_binary_ops! {
    Add (Point<E>, add, Point<E> = Point<E>) laws::sum_of_points_is_valid_point,
    Sub (Point<E>, sub, Point<E> = Point<E>) laws::sub_of_points_is_valid_point,

    Add (Scalar<E>, add, Scalar<E> = Scalar<E>) scalar::add,
    Sub (Scalar<E>, sub, Scalar<E> = Scalar<E>) scalar::sub,
    Mul (Scalar<E>, mul, Scalar<E> = Scalar<E>) scalar::mul,

    Mul (Point<E>, mul, Scalar<E> = Point<E>) laws::mul_of_point_at_scalar_is_valid_point,
    Mul (Scalar<E>, mul, Point<E> = Point<E>) laws::mul_of_scalar_at_point_is_valid_point,

    Add (NonZero<Point<E>>, add, NonZero<Point<E>> = Point<E>) laws::sum_of_points_is_valid_point,
    Sub (NonZero<Point<E>>, sub, NonZero<Point<E>> = Point<E>) laws::sub_of_points_is_valid_point,
    Add (NonZero<Scalar<E>>, add, NonZero<Scalar<E>> = Scalar<E>) scalar::add,
    Sub (NonZero<Scalar<E>>, sub, NonZero<Scalar<E>> = Scalar<E>) scalar::sub,
    Mul (NonZero<Scalar<E>>, mul, NonZero<Scalar<E>> = Scalar<E>) scalar::mul,

    Mul (NonZero<Point<E>>, mul, NonZero<Scalar<E>> = NonZero<Point<E>>) laws::mul_of_nonzero_point_at_nonzero_scalar_is_valid_nonzero_point,
    Mul (NonZero<Scalar<E>>, mul, NonZero<Point<E>> = NonZero<Point<E>>) laws::mul_of_nonzero_scalar_at_nonzero_point_is_valid_nonzero_point,
}

impl_unary_ops! {
    Neg (neg Point<E>) laws::neg_point_is_valid_point,
    Neg (neg Scalar<E>) scalar::neg,
    Neg (neg NonZero<Point<E>>) laws::neg_nonzero_point_is_nonzero_point,
    Neg (neg NonZero<Scalar<E>>) scalar::neg_nonzero,
}
