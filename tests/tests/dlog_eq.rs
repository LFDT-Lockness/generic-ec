#[generic_tests::define]
mod interactive {
    use generic_ec_zkp::dlog_eq::interactive as dlog_eq;

    #[test]
    fn passing_test<E: generic_ec::Curve>() {
        let mut rng = rand_dev::DevRng::new();

        let secret_share = generic_ec::SecretScalar::<E>::random(&mut rng);

        let base = generic_ec::Point::generator() * generic_ec::Scalar::random(&mut rng);
        let data = dlog_eq::Data::from_secret_key(&secret_share, base);

        let r = generic_ec::Scalar::random(&mut rng);

        let comm = dlog_eq::commit_data(data, r);
        let challenge = generic_ec::Scalar::random(&mut rng);
        let proof = dlog_eq::prove(r, challenge, &secret_share);
        dlog_eq::verify(data, comm, challenge, proof).unwrap()
    }
    #[test]
    fn failing_test<E: generic_ec::Curve>() {
        let mut rng = rand_dev::DevRng::new();

        let secret_share = generic_ec::SecretScalar::<E>::random(&mut rng);

        let base = generic_ec::Point::generator() * generic_ec::Scalar::random(&mut rng);
        let mut data = dlog_eq::Data::from_secret_key(&secret_share, base);
        // Replace the exponentiation with something wrong
        data.exp2 += generic_ec::Point::generator();

        let r = generic_ec::Scalar::random(&mut rng);

        let comm = dlog_eq::commit_data(data, r);
        let challenge = generic_ec::Scalar::random(&mut rng);
        let proof = dlog_eq::prove(r, challenge, &secret_share);
        assert!(
            dlog_eq::verify(data, comm, challenge, proof).is_err(),
            "proof should fail"
        )
    }

    #[instantiate_tests(<generic_ec::curves::Secp256k1>)]
    mod secp256k1 {}
    #[instantiate_tests(<generic_ec::curves::Secp256r1>)]
    mod secp256r1 {}
    #[instantiate_tests(<generic_ec::curves::Stark>)]
    mod stark {}
    #[instantiate_tests(<generic_ec::curves::Ed25519>)]
    mod ed25519 {}
}

#[generic_tests::define]
mod non_interactive {
    use generic_ec_zkp::dlog_eq::non_interactive as dlog_eq;

    #[test]
    fn passing_test<E: generic_ec::Curve, D: digest::Digest>() {
        let mut rng = rand_dev::DevRng::new();
        let shared_state = "shared state";

        let secret_share = generic_ec::SecretScalar::random(&mut rng);

        let base = generic_ec::Point::generator() * generic_ec::Scalar::random(&mut rng);
        let data = dlog_eq::Data::from_secret_key(&secret_share, base);

        let r = generic_ec::Scalar::random(&mut rng);
        let proof = dlog_eq::prove::<D, E>(&shared_state, &secret_share, data, r);
        dlog_eq::verify::<D, E>(&shared_state, data, proof).unwrap();
    }

    #[test]
    fn failing_test<E: generic_ec::Curve, D: digest::Digest>() {
        let mut rng = rand_dev::DevRng::new();
        let shared_state = "shared state";

        let secret_share = generic_ec::SecretScalar::random(&mut rng);

        let base = generic_ec::Point::generator() * generic_ec::Scalar::random(&mut rng);
        let mut data = dlog_eq::Data::from_secret_key(&secret_share, base);
        // make exp2 wrong so that proof won't hold
        data.exp2 += generic_ec::Point::generator();

        let r = generic_ec::Scalar::random(&mut rng);

        let proof = dlog_eq::prove::<D, E>(&shared_state, &secret_share, data, r);
        assert!(dlog_eq::verify::<D, E>(&shared_state, data, proof).is_err());
    }

    #[instantiate_tests(<generic_ec::curves::Secp256k1, sha2::Sha256>)]
    mod secp256k1_sha256 {}
    #[instantiate_tests(<generic_ec::curves::Secp256r1, sha2::Sha256>)]
    mod secp256r1_sha256 {}
    #[instantiate_tests(<generic_ec::curves::Stark, sha2::Sha256>)]
    mod stark_sha256 {}
    #[instantiate_tests(<generic_ec::curves::Ed25519, sha2::Sha256>)]
    mod ed25519_sha256 {}
}
