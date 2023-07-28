//! Provides [polynomial](Polynomial) primitive, typically used in secret sharing and threshold DKG

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use requires_alloc::*;

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
mod requires_alloc {
    use alloc::{vec, vec::Vec};
    use core::{iter, ops};

    use generic_ec::traits::{IsZero, Samplable, Zero};
    use rand_core::RngCore;

    /// Polynomial $f(x) = \sum_i a_i x^i$ defined as a list of coefficients $[a_0, \dots, a_{\text{degree}}]$
    ///
    /// Polynomial is generic over type of coefficients `C`, it can be `Scalar<E>`, `NonZero<Scalar<E>>`, `SecretScalar<E>`, `Point<E>`,
    /// or any other type that implements necessary traits.
    #[derive(Debug, Clone)]
    pub struct Polynomial<C> {
        /// `coefs[i]` is coefficient of `x^i` term
        ///
        /// Last element of `coefs` must be non-zero
        coefs: Vec<C>,
    }

    impl<C: IsZero> Polynomial<C> {
        /// Constructs a polynomial from its coefficients
        ///
        /// `coefs[i]` is coefficient of `x^i` term. Resulting polynomial will be
        /// $f(x) = \sum_i \\text{coefs}_i \cdot x^i$
        ///
        /// ## Example
        /// ```rust
        /// # use rand_core::OsRng;
        /// use generic_ec::{Scalar, curves::Secp256k1};
        /// use generic_ec_zkp::polynomial::Polynomial;
        ///
        /// let coefs: [Scalar<Secp256k1>; 3] = [
        ///     Scalar::random(&mut OsRng),
        ///     Scalar::random(&mut OsRng),
        ///     Scalar::random(&mut OsRng),    
        /// ];
        /// let polynomial = Polynomial::from_coefs(coefs.to_vec());
        ///
        /// let x = Scalar::random(&mut OsRng);
        /// assert_eq!(
        ///     coefs[0] + x * coefs[1] + x * x * coefs[2],
        ///     polynomial.value(&x),
        /// );
        /// ```
        pub fn from_coefs(mut coefs: Vec<C>) -> Self {
            // Truncate trailing zeroes
            let zeroes_count = coefs
                .iter()
                .rev()
                .take_while(|coef_i| coef_i.is_zero())
                .count();
            let coefs_len = coefs.len();
            coefs.truncate(coefs_len - zeroes_count);

            Self { coefs }
        }
    }

    impl<C> Polynomial<C> {
        /// Returns polynomial degree
        ///
        /// Polynomial degree is index of most significant non-zero coefficient. Polynomial $f(x) = 0$
        /// considered to have degree $deg(f) = 0$.
        ///
        /// $$
        /// \begin{dcases}
        /// deg(f(x) = \sum_i a_i \cdot x^i) &= n \\text{, where } a_n \ne 0 \land n \to max \\\\
        /// deg(f(x) = 0) &= 0
        /// \end{dcases}
        /// $$
        pub fn degree(&self) -> usize {
            #[allow(clippy::expect_used)]
            match self.coefs.len() {
                0 => 0,
                len => len - 1,
            }
        }

        /// Returns polynomial coefficients
        pub fn coefs(&self) -> &[C] {
            &self.coefs
        }

        /// Destructs polynomial, returns its coefficients
        pub fn into_coefs(self) -> Vec<C> {
            self.coefs
        }
    }

    impl<C: Samplable> Polynomial<C> {
        /// Samples a random polynomial with specified degree
        pub fn sample(rng: &mut impl RngCore, degree: usize) -> Self {
            Self {
                coefs: iter::repeat_with(|| C::random(rng))
                    .take(degree + 1)
                    .collect(),
            }
        }

        /// Samples a random polynomial with specified degree and constant term
        ///
        /// Constant term determines value of polynomial at point zero: $f(0) = \\text{const\\_term}$
        ///
        /// ## Example
        /// ```rust
        /// use generic_ec::{Scalar, curves::Secp256k1};
        /// use generic_ec_zkp::polynomial::Polynomial;
        /// # use rand_core::OsRng;
        ///
        /// let const_term = Scalar::<Secp256k1>::from(1234);
        /// let polynomial = Polynomial::sample_with_const_term(&mut OsRng, 3, const_term);
        /// assert_eq!(const_term, polynomial.value(&Scalar::zero()));
        /// ```
        pub fn sample_with_const_term(
            rng: &mut impl RngCore,
            degree: usize,
            const_term: C,
        ) -> Self {
            Self {
                coefs: iter::once(const_term)
                    .chain(iter::repeat_with(|| C::random(rng)).take(degree))
                    .collect(),
            }
        }
    }

    impl<C> Polynomial<C> {
        /// Evaluates polynomial value at given point: $f(\\text{point})$
        ///
        /// Polynomial coefficients, point, and output can all be differently typed.
        ///
        /// ## Example: polynomial with coefficients typed as non-zero scalars vs elliptic points
        /// Let $f(x) = a_1 \cdot x + a_0$ and $F(x) = G \cdot f(x)$. Coefficients
        /// of $f(x)$ are typed as `NonZero<Scalar<E>>`, and $F(x)$ has coefficients typed as `NonZero<Point<E>>`.
        ///
        /// When we evaluate $f(x)$, we have coefficients `C` of type `NonZero<Scalar<E>>`, and both point `P`
        /// and output `O` of type `Scalar<E>`.
        ///
        /// On other hand, when $F(x)$ is evaluated, coefficients `C` have type `NonZero<Point<E>>`, point `P` has
        /// type `Scalar<E>`, and output `O` is of type `Point<E>`.
        ///
        /// ```rust
        /// use generic_ec::{Point, Scalar, NonZero, curves::Secp256k1};
        /// use generic_ec_zkp::polynomial::Polynomial;
        /// # use rand_core::OsRng;
        ///
        /// let f: Polynomial<NonZero<Scalar<Secp256k1>>> = Polynomial::sample(&mut OsRng, 1);
        /// let F: Polynomial<NonZero<Point<_>>> = &f * &Point::generator();
        ///
        /// let x = Scalar::random(&mut OsRng);
        /// assert_eq!(
        ///     f.value::<_, Scalar<_>>(&x) * Point::generator(),    
        ///     F.value::<_, Point<_>>(&x)
        /// );
        /// ```
        pub fn value<P, O>(&self, point: &P) -> O
        where
            O: Zero,
            for<'a> O: ops::Mul<&'a P, Output = O> + ops::Add<&'a C, Output = O>,
        {
            self.coefs
                .iter()
                .rev()
                .fold(O::zero(), |acc, coef_i| acc * point + coef_i)
        }
    }

    /// Multiplies polyinomial $F(x)$ at $k$ returning resulting polyinomial
    /// $F'(x) = k \cdot F(x)$ without allocations
    ///
    /// $k$ needs to be of type `C`.
    impl<C> ops::Mul<&C> for Polynomial<C>
    where
        for<'a> &'a C: ops::Mul<&'a C, Output = C>,
    {
        type Output = Polynomial<C>;

        fn mul(mut self, rhs: &C) -> Self::Output {
            self.coefs
                .iter_mut()
                .for_each(|coef_i| *coef_i = &*coef_i * rhs);
            self
        }
    }

    /// Multiplies polynomial $F(x)$ at $k$ returning resulting polynomial
    /// $F'(x) = k \cdot F(x)$, resulting polynomial is allocated at heap
    ///
    /// $k$ can be any type as long as it can be multiplied at `C`
    impl<B, C, O> ops::Mul<&B> for &Polynomial<C>
    where
        for<'a> &'a C: ops::Mul<&'a B, Output = O>,
    {
        type Output = Polynomial<O>;

        fn mul(self, rhs: &B) -> Self::Output {
            Polynomial {
                coefs: self.coefs.iter().map(|coef_i| coef_i * rhs).collect(),
            }
        }
    }

    impl<C> ops::AddAssign<&Polynomial<C>> for Polynomial<C>
    where
        C: Clone + for<'a> ops::AddAssign<&'a C>,
    {
        fn add_assign(&mut self, rhs: &Polynomial<C>) {
            self.coefs
                .iter_mut()
                .zip(&rhs.coefs)
                .for_each(|(f1_coef_i, f2_coef_i)| *f1_coef_i += f2_coef_i);
            if self.coefs.len() < rhs.coefs.len() {
                let self_len = self.coefs.len();
                self.coefs.extend_from_slice(&rhs.coefs[self_len..])
            }
        }
    }

    impl<C> ops::Add<&Polynomial<C>> for Polynomial<C>
    where
        C: Clone + for<'a> ops::AddAssign<&'a C>,
    {
        type Output = Polynomial<C>;

        fn add(mut self, rhs: &Polynomial<C>) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl<'a, C> iter::Sum<&'a Polynomial<C>> for Polynomial<C>
    where
        C: Clone + 'a,
        C: for<'c> ops::AddAssign<&'c C>,
    {
        fn sum<I: Iterator<Item = &'a Polynomial<C>>>(mut iter: I) -> Self {
            let Some(mut sum) = iter.next().cloned() else {
            return Self{ coefs: vec![] };
        };
            for polynomial in iter {
                sum += polynomial;
            }
            sum
        }
    }

    impl<C> iter::Sum<Polynomial<C>> for Polynomial<C>
    where
        C: for<'a> ops::AddAssign<&'a C> + Clone,
    {
        fn sum<I: Iterator<Item = Polynomial<C>>>(mut iter: I) -> Self {
            let Some(mut sum) = iter.next() else {
            return Self{ coefs: vec![] };
        };
            for polynomial in iter {
                sum += &polynomial
            }
            sum
        }
    }

    #[cfg(feature = "serde")]
    impl<C> serde::Serialize for Polynomial<C>
    where
        C: serde::Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.coefs.serialize(serializer)
        }
    }

    #[cfg(feature = "serde")]
    impl<'de, C: IsZero> serde::Deserialize<'de> for Polynomial<C>
    where
        C: serde::Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let coefs = Vec::<C>::deserialize(deserializer)?;
            Ok(Self::from_coefs(coefs))
        }
    }
}

use generic_ec::{Curve, NonZero, Scalar};

/// Calculates lagrange coefficient $\lambda_j$ to interpolate a polynomial at point $x$
///
/// Lagrange coefficient are often used to turn polynomial key shares into additive
/// key shares.
///
/// ## Inputs
///
/// `xs` denotes the points with known values that define the polynomial. `j` is a index
/// of element in `xs` for which lagrange coefficient is calculated. `x` is a point at
/// which the polynomial is interpolated.
///
/// `xs` usually refer to "index of a party" of MPC protocol, and shared secret is assigned
/// a coordinate `x=0`. For that reason, elements of `xs` are restricted to be non-zero to
/// avoid an implementation flaw when one of the parties can occupy `xs[j] = 0`.
///
/// ## Returns
/// Returns `None` if `j >= xs.len()` or if there's `m` such that `xs[j] == xs[m]` or
/// `x == xs[m]`. Note that, generally, lagrange interpolation is only defined when
/// elements in `xs` are pairwise distinct.
///
/// ## Example
/// ```rust
/// use generic_ec::{Scalar, SecretScalar, NonZero, curves::Secp256k1};
/// use generic_ec_zkp::polynomial::{Polynomial, lagrange_coefficient};
/// # use rand_core::OsRng;
///
/// let secret = SecretScalar::<Secp256k1>::random(&mut OsRng);
/// // polynomial `f(x)` shares `f(0) = secret`
/// let f = Polynomial::sample_with_const_term(&mut OsRng, 2, secret.clone());
///
/// // Publicly-known shares indexes
/// let I = [
///     NonZero::from_scalar(Scalar::from(1)).unwrap(),
///     NonZero::from_scalar(Scalar::from(2)).unwrap(),
///     NonZero::from_scalar(Scalar::from(3)).unwrap(),
/// ];
/// // Secret shares
/// let shares: [Scalar<_>; 3] = I.map(|i| f.value(&i));
///
/// // Reconstruct a `secret = f(0)`
/// let lambdas = [0, 1, 2].map(|j| lagrange_coefficient(Scalar::zero(), j, &I).unwrap());
/// let reconstructed_secret = lambdas
///     .into_iter()
///     .zip(shares)
///     .map(|(lambda_i, x_i)| lambda_i * x_i)
///     .sum::<Scalar<_>>();
/// assert_eq!(secret.as_ref(), &reconstructed_secret);
/// ```
pub fn lagrange_coefficient<E: Curve>(
    x: Scalar<E>,
    j: usize,
    xs: &[NonZero<Scalar<E>>],
) -> Option<NonZero<Scalar<E>>> {
    let xs_without_j = xs
        .iter()
        .enumerate()
        .filter(|(i, _x_i)| *i != j)
        .map(|(_, x_i)| x_i);
    let nom = xs_without_j
        .clone()
        .map(|x_m| x - x_m)
        .product::<Scalar<E>>();

    let x_j = xs.get(j)?;
    let denom = xs_without_j.map(|x_m| x_j - x_m).product::<Scalar<E>>();
    let denom_inv = denom.invert()?;

    NonZero::from_scalar(nom * denom_inv)
}

#[cfg(all(test, feature = "alloc"))]
#[generic_tests::define]
#[allow(non_snake_case)]
mod tests {
    use alloc::vec::Vec;
    use core::iter;

    use generic_ec::{Curve, NonZero, Point, Scalar, SecretScalar};
    use rand::Rng;
    use rand_dev::DevRng;

    use crate::polynomial::lagrange_coefficient;

    use super::Polynomial;

    #[test]
    fn secret_sharing<E: Curve>() {
        let mut rng = DevRng::new();

        // 1. Sample secret
        let secret = SecretScalar::<E>::random(&mut rng);
        let f = Polynomial::sample_with_const_term(&mut rng, 3, secret.clone());

        // Chech that `f(0) = secret`
        {
            let f_0: Scalar<_> = f.value(&Scalar::zero());
            assert_eq!(secret.as_ref(), &f_0);
        }

        // 2. Commit to the secret
        // F(x) = G * f(x)
        let F = &f * &Point::generator();
        assert_eq!(&secret * Point::generator(), F.value(&Scalar::zero()));

        // 3. Derive 4 secret and public shares
        let shares_indexes: [NonZero<Scalar<E>>; 4] = [
            NonZero::from_scalar(Scalar::from(1)).unwrap(),
            NonZero::from_scalar(Scalar::from(2)).unwrap(),
            NonZero::from_scalar(Scalar::from(3)).unwrap(),
            NonZero::from_scalar(Scalar::from(4)).unwrap(),
        ];
        let shares: [Scalar<_>; 4] = shares_indexes.map(|i| f.value(&i));
        let public_shares = shares.map(|secret_share| Point::generator() * secret_share);

        // Chech that `public_shares[i] = F(i)`
        {
            for (i, public_share) in (1..).zip(&public_shares) {
                assert_eq!(public_share, &F.value(&Scalar::from(i)));
            }
        }

        // 4. Reconstruct the secret
        let lambdas =
            [0, 1, 2, 3].map(|j| lagrange_coefficient(Scalar::zero(), j, &shares_indexes).unwrap());
        let reconstructed_secret: Scalar<E> = lambdas
            .into_iter()
            .zip(shares)
            .map(|(lambda_i, x_i)| lambda_i * x_i)
            .sum();
        assert_eq!(secret.as_ref(), &reconstructed_secret);
    }

    #[test]
    fn polynomial_sum<E: Curve>() {
        let mut rng = DevRng::new();

        // Sample 10 polynomials of different degree
        let polynomials: Vec<Polynomial<Scalar<E>>> = iter::repeat_with(|| {
            let degree = rng.gen_range(5..15);
            Polynomial::sample(&mut rng, degree)
        })
        .take(10)
        .collect();

        let polynomials_sum1: Polynomial<Scalar<E>> = polynomials.iter().sum();
        let polynomials_sum2: Polynomial<Scalar<E>> = polynomials.iter().cloned().sum();

        let point = Scalar::random(&mut rng);

        let value_actual1 = polynomials_sum1.value(&point);
        let value_actual2 = polynomials_sum2.value(&point);

        let value_expected: Scalar<E> = polynomials
            .iter()
            .map(|f_i| f_i.value::<_, Scalar<E>>(&point))
            .sum();

        assert_eq!(value_expected, value_actual1);
        assert_eq!(value_expected, value_actual2);
    }

    #[test]
    fn polynomial_from_coefs<E: Curve>() {
        let mut rng = DevRng::new();

        let coefs = [
            Scalar::<E>::random(&mut rng),
            Scalar::random(&mut rng),
            Scalar::random(&mut rng),
        ];
        let f = Polynomial::from_coefs(coefs.to_vec());

        assert_eq!(f.degree(), 2);

        for _ in 0..100 {
            let x = Scalar::random(&mut rng);

            let f_x: Scalar<E> = f.value(&x);
            let expected = coefs[0] + x * coefs[1] + x * x * coefs[2];

            assert_eq!(f_x, expected);
        }
    }

    #[instantiate_tests(<generic_ec::curves::Secp256k1>)]
    mod secp256k1 {}
    #[instantiate_tests(<generic_ec::curves::Secp256r1>)]
    mod secp256r1 {}
}
