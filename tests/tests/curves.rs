#[generic_tests::define]
mod tests {
    use generic_ec::{curves::*, Curve, Scalar};
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

        let mut be_bytes_rev = scalar_bytes_be.clone();
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

    fn _is_copy<T: Copy>() {}
    fn _test_scalar_is_copy<E: Curve>() {
        _is_copy::<Scalar<E>>();
    }

    #[instantiate_tests(<Secp256k1>)]
    mod secp256k1 {}

    #[instantiate_tests(<Secp256r1>)]
    mod secp256r1 {}
}
