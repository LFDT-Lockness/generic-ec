#![allow(non_snake_case)]

#[generic_tests::define]
mod tests {
    use generic_ec::{curves::*, Curve, EncodedScalar, Point, Scalar};
    use rand::{Rng, RngCore};
    use rand_dev::DevRng;

    #[test]
    fn scalar_bytes<E: Curve>() {
        let mut rng = DevRng::new();
        let scalar = Scalar::<E>::random(&mut rng);

        let scalar_bytes_be = scalar.to_be_bytes();
        let scalar_bytes_le = scalar.to_le_bytes();

        let scalar1 = Scalar::<E>::from_be_bytes(&scalar_bytes_be).unwrap();
        let scalar2 = Scalar::<E>::from_le_bytes(&scalar_bytes_le).unwrap();

        assert_eq!(scalar, scalar1);
        assert_eq!(scalar, scalar2);

        let mut be_bytes_rev = scalar_bytes_be;
        be_bytes_rev.as_mut().reverse();
        assert_eq!(be_bytes_rev, scalar_bytes_le);
    }

    #[test]
    fn scalar_zero<E: Curve>() {
        let mut rng = DevRng::new();

        let zero = Scalar::<E>::zero();
        assert_eq!(zero, Scalar::from(0));

        let r = Scalar::<E>::random(&mut rng);
        assert_eq!(r * zero, zero);
        assert_eq!(r + zero, r);
    }

    #[test]
    fn scalar_one<E: Curve>() {
        let mut rng = DevRng::new();

        let one = Scalar::<E>::one();
        assert_eq!(one, Scalar::from(1));

        let r = Scalar::<E>::random(&mut rng);
        assert_eq!(r + one - one, r);
        assert_eq!(r * one, r);
    }

    #[test]
    fn scalar_from_u128<E: Curve>() {
        let mut rng = DevRng::new();

        let r1: u128 = rng.gen_range(0..(u128::MAX / 2));
        let r2: u128 = rng.gen_range(0..(u128::MAX / 2));

        assert_eq!(
            Scalar::from(r1) + Scalar::from(r2),
            Scalar::<E>::from(r1 + r2)
        )
    }

    #[test]
    fn scalar_from_i128<E: Curve>() {
        let mut rng = DevRng::new();

        let r1: i128 = rng.gen_range(0..i128::MAX);
        let r2: i128 = rng.gen_range(0..i128::MAX);

        assert_eq!(
            Scalar::from(r1) + Scalar::from(-r2),
            Scalar::<E>::from(r1 - r2)
        )
    }

    #[test]
    fn scalar_invert<E: Curve>() {
        let mut rng = DevRng::new();

        let s = Scalar::<E>::random(&mut rng);
        let s_inv = s.invert().unwrap();

        assert_eq!(s * s_inv, Scalar::one());
    }

    #[test]
    fn point_zero<E: Curve>() {
        let mut rng = DevRng::new();

        let s = Scalar::<E>::random(&mut rng);
        assert_eq!(
            Point::generator() * s + Point::zero(),
            Point::generator() * s
        );
        assert_eq!(s * Point::zero(), Point::zero());

        assert!(Point::<E>::zero().is_zero());
        assert!(bool::from(Point::<E>::zero().ct_is_zero()))
    }

    #[test]
    fn point_bytes<E: Curve>() {
        let mut rng = DevRng::new();

        let random_point = Point::generator() * Scalar::<E>::random(&mut rng);

        for point in [Point::zero(), Point::generator().into(), random_point] {
            let bytes_compressed = point.to_bytes(true);
            let bytes_uncompressed = point.to_bytes(false);
            assert!(bytes_compressed.len() <= bytes_uncompressed.len());

            let p1 = Point::<E>::from_bytes(&bytes_compressed).unwrap();
            let p2 = Point::<E>::from_bytes(&bytes_uncompressed).unwrap();

            assert_eq!(point, p1);
            assert_eq!(point, p2);
        }
    }

    #[test]
    fn point_at_scalar<E: Curve>() {
        let mut rng = DevRng::new();

        let s1 = Scalar::<E>::random(&mut rng);
        let s2 = Scalar::<E>::random(&mut rng);

        let p = Point::generator() * s1;
        assert_ne!(p, Point::zero());
        assert_ne!(p * s2, Point::zero());

        assert_ne!(p * Scalar::from(-1), Point::zero());
        assert_eq!(p + p * Scalar::from(-1), Point::zero());
    }

    #[test]
    fn scalar_0xFF_not_valid<E: Curve>() {
        let mut encoded_scalar = EncodedScalar::<E>::default();
        encoded_scalar.as_mut().fill(0xFF);
        Scalar::<E>::from_be_bytes(&encoded_scalar).unwrap_err();
        Scalar::<E>::from_le_bytes(&encoded_scalar).unwrap_err();
    }

    #[test]
    fn scalar_0xFF_valid_mod_order<E: Curve>() {
        let mut encoded_0xFF_scalar = EncodedScalar::<E>::default();
        encoded_0xFF_scalar.as_mut().fill(0xFF);
        let decoded_scalar = Scalar::<E>::from_be_bytes_mod_order(&encoded_0xFF_scalar);

        let scalar_0xFF = Scalar::from(0xFF_u8);
        let scalar_0x100 = scalar_0xFF + Scalar::one();
        let scalar_should_be = (0..encoded_0xFF_scalar.len())
            .fold(Scalar::<E>::zero(), |scalar, _| {
                scalar * scalar_0x100 + scalar_0xFF
            });

        assert_eq!(decoded_scalar, scalar_should_be);
    }

    #[test]
    fn scalar_from_bytes_mod_order<E: Curve>() {
        let mut rng = DevRng::new();

        let fixed_len = [8, 16, 32, 48, 64, 80, 96, 128, 256, 512];
        let random_len = core::iter::repeat_with(|| rng.gen_range(1..=200)).take(20);
        let lenghts = fixed_len.into_iter().chain(random_len).collect::<Vec<_>>();

        for len in lenghts {
            let mut bytes = vec![0u8; len];
            rng.fill_bytes(&mut bytes);

            eprintln!("len = {len}");
            eprintln!("bytes = {}", hex::encode(&bytes));

            let expected = super::naive_scalar_from_be_bytes_mod_order::<E>(&bytes);
            let actual = Scalar::from_be_bytes_mod_order(&bytes);
            assert_eq!(expected, actual);

            let expected = super::naive_scalar_from_le_bytes_mod_order::<E>(&bytes);
            let actual = Scalar::from_le_bytes_mod_order(bytes);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn scalar_one_be<E: Curve>() {
        let one = Scalar::<E>::one();
        let one_be = one.to_be_bytes();
        let len = one_be.len();
        assert_eq!(one_be[len - 1], 1);
        assert!(one_be[..len - 1].iter().all(|x| *x == 0));
        assert_eq!(Scalar::<E>::from_be_bytes(one_be).unwrap(), one);
    }

    fn _is_copy<T: Copy>() {}
    fn _test_point_and_scalar_are_copy<E: Curve>() {
        _is_copy::<Scalar<E>>();
        _is_copy::<Point<E>>();
    }

    #[test]
    fn scalar_radix16<E: Curve>() {
        let mut rng = DevRng::new();

        let random_scalar = Scalar::<E>::random(&mut rng);
        for scalar in [Scalar::zero(), Scalar::one(), -Scalar::one(), random_scalar] {
            let radix16_be = scalar.as_radix16_be().collect::<Vec<_>>();
            assert_eq!(radix16_be.len(), Scalar::<E>::serialized_len() * 2);

            let reconstructed_scalar = radix16_be.iter().fold(Scalar::<E>::zero(), |acc, x| {
                assert!(*x < 16, "{x}");
                acc * Scalar::from(16) + Scalar::from(*x)
            });
            assert_eq!(scalar, reconstructed_scalar);

            let radix_le = scalar.as_radix16_le().collect::<Vec<_>>();
            let expected = {
                let mut rev = radix16_be;
                rev.reverse();
                rev
            };
            assert_eq!(radix_le, expected);
        }
    }

    #[test]
    fn scalar_radix16_iter_len<E: Curve>() {
        let scalar = Scalar::<E>::zero();
        let mut radix16 = scalar.as_radix16_be();
        // `serialized_len` is length of the scalar in radix 256.
        // Multiply it by 2 and you get length of scalar in radix 16
        let expected_len = Scalar::<E>::serialized_len() * 2;

        assert_eq!(radix16.len(), expected_len);

        for expected_len in (0..=expected_len - 1).rev() {
            let _ = radix16.next().unwrap();
            assert_eq!(radix16.len(), expected_len)
        }
    }

    #[instantiate_tests(<Secp256k1>)]
    mod secp256k1 {}

    #[instantiate_tests(<Secp256r1>)]
    mod secp256r1 {}

    #[instantiate_tests(<Stark>)]
    mod stark {}

    #[instantiate_tests(<Ed25519>)]
    mod ed25519 {}
}

#[generic_tests::define]
mod scalar_reduce {
    use generic_ec::{traits::Reduce, Curve, Scalar};
    use rand::RngCore;

    #[test]
    fn reduce<E: Curve, const N: usize>()
    where
        Scalar<E>: Reduce<N>,
    {
        let mut rng = rand_dev::DevRng::new();

        let mut bytes = [0u8; N];
        rng.fill_bytes(&mut bytes);

        let expected = super::naive_scalar_from_be_bytes_mod_order::<E>(&bytes);
        let actual = Scalar::from_be_array_mod_order(&bytes);
        assert_eq!(expected, actual);

        let expected = super::naive_scalar_from_le_bytes_mod_order::<E>(&bytes);
        let actual = Scalar::from_le_array_mod_order(&bytes);
        assert_eq!(expected, actual);
    }

    #[instantiate_tests(<generic_ec::curves::Secp256k1, 32>)]
    mod secp256k1_32 {}
    #[instantiate_tests(<generic_ec::curves::Secp256k1, 64>)]
    mod secp256k1_64 {}

    #[instantiate_tests(<generic_ec::curves::Secp256r1, 32>)]
    mod secp256r1_32 {}

    #[instantiate_tests(<generic_ec::curves::Stark, 32>)]
    mod stark_32 {}

    #[instantiate_tests(<generic_ec::curves::Ed25519, 32>)]
    mod ed25519_32 {}
    #[instantiate_tests(<generic_ec::curves::Ed25519, 64>)]
    mod ed25519_64 {}
}

#[generic_tests::define]
mod coordinates {
    use generic_ec::coords::{HasAffineX, HasAffineXAndParity, HasAffineXY, HasAffineY};
    use generic_ec::curves::{Secp256k1, Secp256r1, Stark};
    use generic_ec::{Curve, Point, Scalar};

    use rand_dev::DevRng;

    #[test]
    fn identity_point_doesnt_have_coords<E: Curve>()
    where
        Point<E>: HasAffineXY<E> + HasAffineXAndParity<E>,
    {
        let identity = Point::<E>::zero();
        assert_eq!(identity.x(), None);
        assert_eq!(identity.x_and_parity(), None);
        assert_eq!(identity.y(), None);
        assert_eq!(identity.coords(), None);
    }

    #[test]
    fn point_exposes_x_and_parity<E: Curve>()
    where
        Point<E>: HasAffineXAndParity<E>,
    {
        let mut rng = DevRng::new();
        let random_point = Point::<E>::generator() * Scalar::random(&mut rng);

        let (x, parity) = random_point.x_and_parity().unwrap();
        assert_eq!(random_point.x(), Some(x.clone()));

        let reassembled_point = Point::from_x_and_parity(&x, parity).unwrap();
        assert_eq!(random_point, reassembled_point);
    }

    #[test]
    fn point_exposes_x_and_y<E: Curve>()
    where
        Point<E>: HasAffineXY<E>,
    {
        let mut rng = DevRng::new();
        let random_point = Point::<E>::generator() * Scalar::random(&mut rng);

        let coords = random_point.coords().unwrap();
        assert_eq!(random_point.x(), Some(coords.x.clone()));
        assert_eq!(random_point.y(), Some(coords.y.clone()));

        let reassembled_point = Point::from_coords(&coords).unwrap();
        assert_eq!(random_point, reassembled_point);
    }

    #[instantiate_tests(<Secp256k1>)]
    mod secp256k1 {}

    #[instantiate_tests(<Secp256r1>)]
    mod secp256r1 {}

    #[instantiate_tests(<Stark>)]
    mod stark {}
}

fn naive_scalar_from_be_bytes_mod_order<E: generic_ec::Curve>(
    bytes: &[u8],
) -> generic_ec::Scalar<E> {
    let scalar_0x100 = generic_ec::Scalar::from(0x100);

    bytes
        .iter()
        .fold(generic_ec::Scalar::<E>::zero(), |acc, s_i| {
            acc * scalar_0x100 + generic_ec::Scalar::from(*s_i)
        })
}
fn naive_scalar_from_le_bytes_mod_order<E: generic_ec::Curve>(
    bytes: &[u8],
) -> generic_ec::Scalar<E> {
    let scalar_0x100 = generic_ec::Scalar::from(0x100);

    bytes
        .iter()
        .rev()
        .fold(generic_ec::Scalar::<E>::zero(), |acc, s_i| {
            acc * scalar_0x100 + generic_ec::Scalar::from(*s_i)
        })
}
