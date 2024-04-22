#[generic_tests::define]
mod tests {
    use core::iter;

    use generic_ec::{
        curves::{Ed25519, Secp256k1, Secp256r1, Stark},
        multiscalar::{Dalek, MultiscalarMul, Naive, Straus},
        Curve, Point, Scalar,
    };
    use rand::Rng;

    #[test]
    fn multiscalar_mul<E: Curve, M: MultiscalarMul<E>>() {
        let mut rng = rand_dev::DevRng::new();
        let lengths = [1, 2, rng.gen_range(3..=20), rng.gen_range(50..=150)];

        for len in lengths {
            let scalar_points = iter::repeat_with(|| {
                (
                    Scalar::<E>::random(&mut rng),
                    Scalar::<E>::random(&mut rng) * Point::generator(),
                )
            })
            .take(len)
            .collect::<Vec<_>>();

            let actual = M::multiscalar_mul(scalar_points.iter().copied());
            let expected = Naive::multiscalar_mul(scalar_points.iter().copied());

            assert_eq!(actual, expected);
        }
    }

    #[instantiate_tests(<Secp256k1, Straus>)]
    mod secp256k1_straus {}
    #[instantiate_tests(<Secp256r1, Straus>)]
    mod secp256r1_straus {}
    #[instantiate_tests(<Stark, Straus>)]
    mod stark_straus {}
    #[instantiate_tests(<Ed25519, Straus>)]
    mod ed25519_straus {}
    #[instantiate_tests(<Ed25519, Dalek>)]
    mod ed25519_dalek {}
}
