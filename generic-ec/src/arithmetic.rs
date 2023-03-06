use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};

use crate::{Curve, Generator, NonZero, Point, Scalar, SecretScalar};

mod laws {
    use crate::{
        as_raw::AsRaw,
        core::{self, *},
        Generator, NonZero,
    };
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
        n: impl AsRef<Scalar<E>>,
        a: &Point<E>,
    ) -> Point<E> {
        let prod = Multiplicative::mul(n.as_ref().as_raw(), a.as_raw());
        // Correctness: refer to doc comment of the function
        Point::from_raw_unchecked(prod)
    }

    /// Same as [`mul_of_scalar_at_point_is_valid_point`] but flipped arguments
    #[inline]
    pub fn mul_of_point_at_scalar_is_valid_point<E: Curve>(
        a: &Point<E>,
        b: impl AsRef<Scalar<E>>,
    ) -> Point<E> {
        mul_of_scalar_at_point_is_valid_point(b, a)
    }

    /// If $n$ is valid `Scalar<E>`, then $n \G$ is valid `Point<E>`
    ///
    /// Proof is the same as in [`mul_of_scalar_at_point_is_valid_point`] with $A = \G$
    #[inline]
    pub fn mul_of_scalar_at_generator_is_valid_point<E: Curve>(
        n: impl AsRef<Scalar<E>>,
        _g: &Generator<E>,
    ) -> Point<E> {
        let prod = Multiplicative::mul(n.as_ref().as_raw(), &core::CurveGenerator);
        // Correctness: refer to doc comment of the function
        Point::from_raw_unchecked(prod)
    }

    /// Same as [`mul_of_scalar_at_generator_is_valid_point`] but flipped arguments
    #[inline]
    pub fn mul_of_generator_at_scalar_is_valid_point<E: Curve>(
        g: &Generator<E>,
        n: impl AsRef<Scalar<E>>,
    ) -> Point<E> {
        mul_of_scalar_at_generator_is_valid_point(n, g)
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

    /// If $n$ is valid `NonZero<Scalar<E>>`, then $n \G$ is valid `NonZero<Point<E>>`
    ///
    /// Proof is the same as in [`mul_of_nonzero_scalar_at_nonzero_point_is_valid_nonzero_point`]
    #[inline]
    pub fn mul_of_nonzero_scalar_at_generator_is_valid_nonzero_point<E: Curve>(
        n: &Scalar<E>,
        g: &Generator<E>,
    ) -> NonZero<Point<E>> {
        let prod = mul_of_scalar_at_generator_is_valid_point(n, g);
        // Correctness: refer to doc comment of the function
        NonZero::new_unchecked(prod)
    }

    /// Same as [`mul_of_nonzero_scalar_at_generator_is_valid_nonzero_point`] but flipped arguments
    #[inline]
    pub fn mul_of_generator_at_nonzero_scalar_is_valid_nonzero_point<E: Curve>(
        g: &Generator<E>,
        n: &Scalar<E>,
    ) -> NonZero<Point<E>> {
        mul_of_nonzero_scalar_at_generator_is_valid_nonzero_point(n, g)
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
        let neg = neg_point_is_valid_point(a);
        // Correctness: refer to doc comment of the function
        NonZero::new_unchecked(neg)
    }
}

mod scalar {
    use crate::as_raw::{AsRaw, FromRaw};
    use crate::core::*;
    use crate::NonZero;
    use crate::Scalar;

    #[inline]
    pub fn add<E: Curve>(a: impl AsRef<Scalar<E>>, b: impl AsRef<Scalar<E>>) -> Scalar<E> {
        let sum = Additive::add(a.as_ref().as_raw(), b.as_ref().as_raw());
        Scalar::from_raw(sum)
    }

    #[inline]
    pub fn sub<E: Curve>(a: impl AsRef<Scalar<E>>, b: impl AsRef<Scalar<E>>) -> Scalar<E> {
        let result = Additive::sub(a.as_ref().as_raw(), b.as_ref().as_raw());
        Scalar::from_raw(result)
    }

    #[inline]
    pub fn mul<E: Curve>(a: impl AsRef<Scalar<E>>, b: impl AsRef<Scalar<E>>) -> Scalar<E> {
        let prod = Multiplicative::mul(a.as_ref().as_raw(), b.as_ref().as_raw());
        Scalar::from_raw(prod)
    }

    #[inline]
    pub fn neg<E: Curve>(a: &Scalar<E>) -> Scalar<E> {
        let result = Additive::negate(a.as_raw());
        Scalar::from_raw(result)
    }

    #[inline]
    pub fn neg_nonzero<E: Curve>(a: &NonZero<Scalar<E>>) -> NonZero<Scalar<E>> {
        let neg = neg(a);
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

macro_rules! impl_nonzero_ops {
    ($($op:ident ($lhs:ty, $op_fn:ident, $rhs:ty = $out:ty) $impl_fn:path),+,) => {
        impl_binary_ops! {$(
            $op (NonZero<$lhs>, $op_fn, NonZero<$rhs> = $out) $impl_fn,
            $op ($lhs, $op_fn, NonZero<$rhs> = $out) $impl_fn,
            $op (NonZero<$lhs>, $op_fn, $rhs = $out) $impl_fn,
        )+}
    };
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

macro_rules! impl_op_assign {
    ($($ty:ty, $trait:ident, $rhs:ty, $fn:ident, $op:tt),+,) => {$(
        impl<E: Curve> $trait<$rhs> for $ty {
            fn $fn(&mut self, rhs: $rhs) {
                *self = *self $op rhs;
            }
        }
        impl<E: Curve> $trait<&$rhs> for $ty {
            fn $fn(&mut self, rhs: &$rhs) {
                *self = *self $op rhs;
            }
        }
    )+};
}

// Point <> Point, Point <> Scalar, Scalar <> Scalar arithmetic ops
impl_binary_ops! {
    Add (Point<E>, add, Point<E> = Point<E>) laws::sum_of_points_is_valid_point,
    Sub (Point<E>, sub, Point<E> = Point<E>) laws::sub_of_points_is_valid_point,

    Add (Scalar<E>, add, Scalar<E> = Scalar<E>) scalar::add,
    Sub (Scalar<E>, sub, Scalar<E> = Scalar<E>) scalar::sub,
    Mul (Scalar<E>, mul, Scalar<E> = Scalar<E>) scalar::mul,

    Mul (Point<E>, mul, Scalar<E> = Point<E>) laws::mul_of_point_at_scalar_is_valid_point,
    Mul (Scalar<E>, mul, Point<E> = Point<E>) laws::mul_of_scalar_at_point_is_valid_point,
    Mul (Generator<E>, mul, Scalar<E> = Point<E>) laws::mul_of_generator_at_scalar_is_valid_point,
    Mul (Scalar<E>, mul, Generator<E> = Point<E>) laws::mul_of_scalar_at_generator_is_valid_point,
}

// Point <> SecretScalar, Scalar <> SecretScalar, NonZero<Point> <> SecretScalar,
// NonZero<Scalar> <> SecretScalar arithmetic ops
impl_binary_ops! {
    Add (SecretScalar<E>, add, Scalar<E> = Scalar<E>) scalar::add,
    Add (Scalar<E>, add, SecretScalar<E> = Scalar<E>) scalar::add,
    Add (SecretScalar<E>, add, NonZero<Scalar<E>> = Scalar<E>) scalar::add,
    Add (NonZero<Scalar<E>>, add, SecretScalar<E> = Scalar<E>) scalar::add,

    Sub (SecretScalar<E>, sub, Scalar<E> = Scalar<E>) scalar::sub,
    Sub (Scalar<E>, sub, SecretScalar<E> = Scalar<E>) scalar::sub,
    Sub (SecretScalar<E>, sub, NonZero<Scalar<E>> = Scalar<E>) scalar::sub,
    Sub (NonZero<Scalar<E>>, sub, SecretScalar<E> = Scalar<E>) scalar::sub,

    Mul (SecretScalar<E>, mul, Scalar<E> = Scalar<E>) scalar::mul,
    Mul (Scalar<E>, mul, SecretScalar<E> = Scalar<E>) scalar::mul,
    Mul (SecretScalar<E>, mul, NonZero<Scalar<E>> = Scalar<E>) scalar::mul,
    Mul (NonZero<Scalar<E>>, mul, SecretScalar<E> = Scalar<E>) scalar::mul,

    Mul (Point<E>, mul, SecretScalar<E> = Point<E>) laws::mul_of_point_at_scalar_is_valid_point,
    Mul (SecretScalar<E>, mul, Point<E> = Point<E>) laws::mul_of_scalar_at_point_is_valid_point,
    Mul (Generator<E>, mul, SecretScalar<E> = Point<E>) laws::mul_of_generator_at_scalar_is_valid_point,
    Mul (SecretScalar<E>, mul, Generator<E> = Point<E>) laws::mul_of_scalar_at_generator_is_valid_point,
    Mul (NonZero<Point<E>>, mul, SecretScalar<E> = Point<E>) laws::mul_of_point_at_scalar_is_valid_point,
    Mul (SecretScalar<E>, mul, NonZero<Point<E>> = Point<E>) laws::mul_of_scalar_at_point_is_valid_point,
}

// NonZero<Point> <> NonZero<Scalar> arithmetic ops
impl_binary_ops! {
    Mul (NonZero<Point<E>>, mul, NonZero<Scalar<E>> = NonZero<Point<E>>) laws::mul_of_nonzero_point_at_nonzero_scalar_is_valid_nonzero_point,
    Mul (NonZero<Scalar<E>>, mul, NonZero<Point<E>> = NonZero<Point<E>>) laws::mul_of_nonzero_scalar_at_nonzero_point_is_valid_nonzero_point,
    Mul (Generator<E>, mul, NonZero<Scalar<E>> = NonZero<Point<E>>) laws::mul_of_generator_at_nonzero_scalar_is_valid_nonzero_point,
    Mul (NonZero<Scalar<E>>, mul, Generator<E> = NonZero<Point<E>>) laws::mul_of_nonzero_scalar_at_generator_is_valid_nonzero_point,
}

// Point <> NonZero<Point>, Scalar <> NonZero<Scalar>,
// NonZero<Point> <> NonZero<Point>, NonZero<Scalar> <> NonZero<Scalar> arithmetic ops
impl_nonzero_ops! {
    Add (Point<E>, add, Point<E> = Point<E>) laws::sum_of_points_is_valid_point,
    Sub (Point<E>, sub, Point<E> = Point<E>) laws::sub_of_points_is_valid_point,

    Add (Scalar<E>, add, Scalar<E> = Scalar<E>) scalar::add,
    Sub (Scalar<E>, sub, Scalar<E> = Scalar<E>) scalar::sub,
    Mul (Scalar<E>, mul, Scalar<E> = Scalar<E>) scalar::mul,
}

// Point <> NonZero<Scalar>, NonZero<Point> <> Scalar
impl_binary_ops! {
    Mul (Point<E>, mul, NonZero<Scalar<E>> = Point<E>) laws::mul_of_point_at_scalar_is_valid_point,
    Mul (NonZero<Scalar<E>>, mul, Point<E> = Point<E>) laws::mul_of_scalar_at_point_is_valid_point,
    Mul (NonZero<Point<E>>, mul, Scalar<E> = Point<E>) laws::mul_of_point_at_scalar_is_valid_point,
    Mul (Scalar<E>, mul, NonZero<Point<E>> = Point<E>) laws::mul_of_scalar_at_point_is_valid_point,
}

// -Point, -Scalar, -NonZero<Point>, -NonZero<Scalar>
impl_unary_ops! {
    Neg (neg Point<E>) laws::neg_point_is_valid_point,
    Neg (neg Scalar<E>) scalar::neg,
    Neg (neg NonZero<Point<E>>) laws::neg_nonzero_point_is_nonzero_point,
    Neg (neg NonZero<Scalar<E>>) scalar::neg_nonzero,
}

impl_op_assign! {
    Point<E>, AddAssign, Point<E>, add_assign, +,
    Point<E>, MulAssign, Scalar<E>, mul_assign, *,
    Scalar<E>, AddAssign, Scalar<E>, add_assign, +,
    Scalar<E>, MulAssign, Scalar<E>, mul_assign, *,
}

#[cfg(test)]
#[allow(dead_code, clippy::redundant_clone)]
fn ensure_ops_implemented<E: Curve>(
    g: Generator<E>,
    point: Point<E>,
    scalar: Scalar<E>,
    non_zero_point: NonZero<Point<E>>,
    non_zero_scalar: NonZero<Scalar<E>>,
    secret_scalar: SecretScalar<E>,
) {
    macro_rules! assert_binary_ops {
        ($($a:ident $op:tt $b:expr => $out:ty),+,) => {$(
            let _: $out = $a $op $b;
            let _: $out = &$a $op $b;
            let _: $out = $a $op &$b;
            let _: $out = &$a $op &$b;

            let _: $out = $b $op $a;
            let _: $out = &$b $op $a;
            let _: $out = $b $op &$a;
            let _: $out = &$b $op &$a;
        )+};
    }
    macro_rules! assert_unary_ops {
        ($($op:tt $a:ident => $out:ty),+,) => {$(
            let _: $out = $op $a;
            let _: $out = $op &$a;
        )+};
    }

    assert_binary_ops!(
        g * scalar => Point<E>,
        point * scalar => Point<E>,
        g * non_zero_scalar => NonZero<Point<E>>,
        non_zero_point * non_zero_scalar => NonZero<Point<E>>,

        g * secret_scalar.clone() => Point<E>,
        point * secret_scalar.clone() => Point<E>,
        non_zero_point * secret_scalar.clone() => Point<E>,

        point + point => Point<E>,
        point + non_zero_point => Point<E>,
        non_zero_point + non_zero_point => Point<E>,

        point - point => Point<E>,
        point - non_zero_point => Point<E>,
        non_zero_point - non_zero_point => Point<E>,

        scalar + scalar => Scalar<E>,
        scalar + non_zero_scalar => Scalar<E>,
        non_zero_scalar + non_zero_scalar => Scalar<E>,

        scalar + secret_scalar.clone() => Scalar<E>,
        non_zero_scalar + secret_scalar.clone() => Scalar<E>,

        scalar - scalar => Scalar<E>,
        scalar - non_zero_scalar => Scalar<E>,
        non_zero_scalar - non_zero_scalar => Scalar<E>,

        scalar - secret_scalar.clone() => Scalar<E>,
        non_zero_scalar - secret_scalar.clone() => Scalar<E>,

        scalar * scalar => Scalar<E>,
        scalar * non_zero_scalar => Scalar<E>,
        non_zero_scalar * non_zero_scalar => Scalar<E>,

        scalar * secret_scalar.clone() => Scalar<E>,
        non_zero_scalar * secret_scalar.clone() => Scalar<E>,
    );

    assert_unary_ops!(
        -point => Point<E>,
        -non_zero_point => NonZero<Point<E>>,
        -scalar => Scalar<E>,
        -non_zero_scalar => NonZero<Scalar<E>>,
    );
}
