#![allow(non_snake_case)]

#[generic_tests::define]
mod tests {
    use generic_ec::{curves::*, Curve, EncodedScalar, Point, Scalar};
    use rand::Rng;
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

        let s = Scalar::<E>::random(&mut rng);
        let p = Point::generator() * s;

        let bytes_compressed = p.to_bytes(true);
        let bytes_uncompressed = p.to_bytes(false);
        assert!(bytes_compressed.len() <= bytes_uncompressed.len());

        let p1 = Point::<E>::from_bytes(&bytes_compressed).unwrap();
        let p2 = Point::<E>::from_bytes(&bytes_uncompressed).unwrap();

        assert_eq!(p, p1);
        assert_eq!(p, p2);
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

        let s = Scalar::<E>::random(&mut rng);
        let s_be = s.to_be_bytes();
        let s_le = s.to_le_bytes();

        let s1 = Scalar::<E>::from_be_bytes_mod_order(&s_be);
        let s2 = Scalar::<E>::from_le_bytes_mod_order(&s_le);

        assert_eq!(s, s1);
        assert_eq!(s, s2);
    }

    fn _is_copy<T: Copy>() {}
    fn _test_point_and_scalar_are_copy<E: Curve>() {
        _is_copy::<Scalar<E>>();
        _is_copy::<Point<E>>();
    }

    #[instantiate_tests(<Secp256k1>)]
    mod secp256k1 {}

    #[instantiate_tests(<Secp256r1>)]
    mod secp256r1 {}
}

#[generic_tests::define]
mod coordinates {
    use generic_ec::coords::{HasAffineX, HasAffineXAndParity, HasAffineXY, HasAffineY};
    use generic_ec::curves::{Secp256k1, Secp256r1};
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
}
