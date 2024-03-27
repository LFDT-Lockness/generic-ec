#[generic_tests::define]
mod tests {
    use generic_ec::{Curve, Point, Scalar};
    use serde_test::{Configure, Token};

    #[test]
    fn serialize_point<E: Curve>() {
        let mut rng = rand_dev::DevRng::new();

        let random_point = Point::<E>::generator() * Scalar::random(&mut rng);
        for point in [Point::zero(), Point::generator().into(), random_point] {
            let point_uncompressed = point.to_bytes(false).to_vec().leak();
            let point_uncompressed_hex = hex::encode(&point_uncompressed).leak();
            let point_compressed = point.to_bytes(true).to_vec().leak();
            let point_compressed_hex = hex::encode(&point_compressed).leak();

            // Human-readable
            serde_test::assert_ser_tokens(
                &point.readable(),
                &[
                    Token::Struct {
                        name: "PointUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("point"),
                    Token::Str(point_uncompressed_hex),
                    Token::StructEnd,
                ],
            );

            // Human-readable, compact
            {
                let tokens = &[
                    Token::NewtypeStruct {
                        name: "PointCompact",
                    },
                    Token::Str(point_compressed_hex),
                ];
                serde_test::assert_ser_tokens(&Compact(point).readable(), tokens);
                serde_test::assert_ser_tokens(&PreferCompact(point).readable(), tokens);
            }

            // Binary
            serde_test::assert_ser_tokens(
                &point.compact(),
                &[
                    Token::Struct {
                        name: "PointUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("point"),
                    Token::Bytes(point_uncompressed),
                    Token::StructEnd,
                ],
            );

            // Binary, compact
            {
                let tokens = &[
                    Token::NewtypeStruct {
                        name: "PointCompact",
                    },
                    Token::Bytes(point_compressed),
                ];
                serde_test::assert_ser_tokens(&Compact(point).compact(), tokens);
                serde_test::assert_ser_tokens(&PreferCompact(point).compact(), tokens);
            }
        }
    }

    #[test]
    fn deserialize_point<E: Curve>() {
        let mut rng = rand_dev::DevRng::new();

        let random_point = Point::<E>::generator() * Scalar::random(&mut rng);
        for point in [Point::zero(), Point::generator().into(), random_point] {
            let point_uncompressed = point.to_bytes(false).to_vec().leak();
            let point_uncompressed_hex = hex::encode(&point_uncompressed).leak();
            let point_compressed = point.to_bytes(true).to_vec().leak();
            let point_compressed_hex = hex::encode(&point_compressed).leak();

            // Uncompressed, human-readable (hex-encoding)
            {
                let tokens = &[
                    Token::Struct {
                        name: "PointUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("point"),
                    Token::Str(point_uncompressed_hex),
                    Token::StructEnd,
                ];
                serde_test::assert_de_tokens(&point.readable(), tokens);
                serde_test::assert_de_tokens(&PreferCompact(point).readable(), tokens);
            }

            // Uncompressed, binary (seq-encoded)
            {
                let mut tokens = vec![
                    Token::Struct {
                        name: "PointUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("point"),
                    Token::Seq { len: None },
                ];
                tokens.extend(point_uncompressed.iter().copied().map(Token::U8));
                tokens.extend([Token::SeqEnd, Token::StructEnd]);
                serde_test::assert_de_tokens(&point.readable(), &tokens);
                serde_test::assert_de_tokens(&PreferCompact(point).readable(), &tokens);
            }

            // Uncompressed, binary (bytes-encoded)
            {
                let tokens = &[
                    Token::Struct {
                        name: "PointUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("point"),
                    Token::Bytes(point_uncompressed),
                    Token::StructEnd,
                ];
                serde_test::assert_de_tokens(&point.readable(), tokens);
                serde_test::assert_de_tokens(&PreferCompact(point).readable(), tokens);
            }

            // Compact, human-readable (hex-encoding)
            {
                let tokens = &[
                    Token::NewtypeStruct {
                        name: "PointCompact",
                    },
                    Token::Str(point_compressed_hex),
                ];
                serde_test::assert_de_tokens(&Compact(point).readable(), tokens);
                serde_test::assert_de_tokens(&PreferCompact(point).readable(), tokens);
            }

            // Compact, binary (seq-encoded)
            {
                let mut tokens = vec![
                    Token::NewtypeStruct {
                        name: "PointCompact",
                    },
                    Token::Seq { len: None },
                ];
                tokens.extend(point_compressed.iter().copied().map(Token::U8));
                tokens.push(Token::SeqEnd);
                serde_test::assert_de_tokens(&Compact(point).readable(), &tokens);
                serde_test::assert_de_tokens(&PreferCompact(point).readable(), &tokens);
            }

            // Seq-encoded struct, human-readable (hex-encoded)
            serde_test::assert_de_tokens(
                &point.readable(),
                &[
                    Token::Seq { len: Some(2) },
                    Token::Str(E::CURVE_NAME),
                    Token::Str(point_uncompressed_hex),
                    Token::SeqEnd,
                ],
            );
            // Seq-encoded struct, binary (bytes-encoded)
            serde_test::assert_de_tokens(
                &point.readable(),
                &[
                    Token::Seq { len: Some(2) },
                    Token::Str(E::CURVE_NAME),
                    Token::Bytes(point_uncompressed),
                    Token::SeqEnd,
                ],
            );
            // Seq-encoded struct, binary (seq-encoded)
            {
                let mut tokens = vec![
                    Token::Seq { len: Some(2) },
                    Token::Str(E::CURVE_NAME),
                    Token::Seq { len: None },
                ];
                tokens.extend(point_uncompressed.iter().copied().map(Token::U8));
                tokens.extend([Token::SeqEnd, Token::SeqEnd]);

                serde_test::assert_de_tokens(&point.readable(), &tokens);
            }

            // PreferCompact doesn't support seq-encoded structs
            serde_test::assert_de_tokens_error::<serde_test::Compact<PreferCompact<Point<E>>>>(
                &[Token::Seq { len: Some(2) }],
                "cannot deserialize in `PreferCompact` mode from sequence: it's ambiguous",
            );
        }
    }

    #[test]
    fn serialize_scalar<E: Curve>() {
        let mut rng = rand_dev::DevRng::new();

        let random_scalar = Scalar::<E>::random(&mut rng);
        for scalar in [Scalar::zero(), Scalar::one(), -Scalar::one(), random_scalar] {
            let scalar_bytes = scalar.to_be_bytes().to_vec().leak();
            let scalar_hex = hex::encode(&scalar_bytes).leak();

            // Human-readable
            serde_test::assert_ser_tokens(
                &scalar.readable(),
                &[
                    Token::Struct {
                        name: "ScalarUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("scalar"),
                    Token::Str(scalar_hex),
                    Token::StructEnd,
                ],
            );

            // Human-readable, compact
            {
                let tokens = &[
                    Token::NewtypeStruct {
                        name: "ScalarCompact",
                    },
                    Token::Str(scalar_hex),
                ];
                serde_test::assert_ser_tokens(&Compact(scalar).readable(), tokens);
                serde_test::assert_ser_tokens(&PreferCompact(scalar).readable(), tokens);
            }

            // Binary
            serde_test::assert_ser_tokens(
                &scalar.compact(),
                &[
                    Token::Struct {
                        name: "ScalarUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("scalar"),
                    Token::Bytes(scalar_bytes),
                    Token::StructEnd,
                ],
            );

            // Binary, compact
            {
                let tokens = &[
                    Token::NewtypeStruct {
                        name: "ScalarCompact",
                    },
                    Token::Bytes(scalar_bytes),
                ];
                serde_test::assert_ser_tokens(&Compact(scalar).compact(), tokens);
                serde_test::assert_ser_tokens(&PreferCompact(scalar).compact(), tokens);
            }
        }
    }

    #[test]
    fn deserialize_scalar<E: Curve>() {
        let mut rng = rand_dev::DevRng::new();

        let random_scalar = Scalar::<E>::random(&mut rng);
        for scalar in [Scalar::zero(), Scalar::one(), -Scalar::one(), random_scalar] {
            let scalar_bytes = scalar.to_be_bytes().to_vec().leak();
            let scalar_hex = hex::encode(&scalar_bytes).leak();

            // Uncompressed, hex-encoding
            {
                let tokens = &[
                    Token::Struct {
                        name: "ScalarUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("scalar"),
                    Token::Str(scalar_hex),
                    Token::StructEnd,
                ];
                serde_test::assert_de_tokens(&scalar.readable(), tokens);
                serde_test::assert_de_tokens(&PreferCompact(scalar).readable(), tokens);
            }

            // Uncompressed, seq-encoded
            {
                let mut tokens = vec![
                    Token::Struct {
                        name: "ScalarUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("scalar"),
                    Token::Seq { len: None },
                ];
                tokens.extend(scalar_bytes.iter().copied().map(Token::U8));
                tokens.extend([Token::SeqEnd, Token::StructEnd]);
                serde_test::assert_de_tokens(&scalar.readable(), &tokens);
                serde_test::assert_de_tokens(&PreferCompact(scalar).readable(), &tokens);
            }

            // Uncompressed, bytes-encoded
            {
                let tokens = &[
                    Token::Struct {
                        name: "ScalarUncompressed",
                        len: 2,
                    },
                    Token::Str("curve"),
                    Token::Str(E::CURVE_NAME),
                    Token::Str("scalar"),
                    Token::Bytes(scalar_bytes),
                    Token::StructEnd,
                ];
                serde_test::assert_de_tokens(&scalar.readable(), tokens);
                serde_test::assert_de_tokens(&PreferCompact(scalar).readable(), tokens);
            }

            // Compact, hex-encoded
            {
                let tokens = &[
                    Token::NewtypeStruct {
                        name: "ScalarCompact",
                    },
                    Token::Str(scalar_hex),
                ];
                serde_test::assert_de_tokens(&Compact(scalar).readable(), tokens);
                serde_test::assert_de_tokens(&PreferCompact(scalar).readable(), tokens);
            }

            // Compact, seq-encoded
            {
                let mut tokens = vec![
                    Token::NewtypeStruct {
                        name: "ScalarCompact",
                    },
                    Token::Seq { len: None },
                ];
                tokens.extend(scalar_bytes.iter().copied().map(Token::U8));
                tokens.extend([Token::SeqEnd]);
                serde_test::assert_de_tokens(&Compact(scalar).readable(), &tokens);
                serde_test::assert_de_tokens(&PreferCompact(scalar).readable(), &tokens);
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    struct Compact<T>(T);
    impl<T> serde::Serialize for Compact<T>
    where
        generic_ec::serde::Compact: serde_with::SerializeAs<T>,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde_with::SerializeAs;
            generic_ec::serde::Compact::serialize_as(&self.0, serializer)
        }
    }
    impl<'de, T> serde::Deserialize<'de> for Compact<T>
    where
        generic_ec::serde::Compact: serde_with::DeserializeAs<'de, T>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde_with::DeserializeAs;
            generic_ec::serde::Compact::deserialize_as(deserializer).map(Self)
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    struct PreferCompact<T>(T);
    impl<T> serde::Serialize for PreferCompact<T>
    where
        generic_ec::serde::PreferCompact: serde_with::SerializeAs<T>,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde_with::SerializeAs;
            generic_ec::serde::PreferCompact::serialize_as(&self.0, serializer)
        }
    }
    impl<'de, T> serde::Deserialize<'de> for PreferCompact<T>
    where
        generic_ec::serde::PreferCompact: serde_with::DeserializeAs<'de, T>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde_with::DeserializeAs;
            generic_ec::serde::PreferCompact::deserialize_as(deserializer).map(Self)
        }
    }

    #[instantiate_tests(<generic_ec::curves::Secp256k1>)]
    mod secp256k1 {}

    #[instantiate_tests(<generic_ec::curves::Secp256r1>)]
    mod secp256r1 {}

    #[instantiate_tests(<generic_ec::curves::Stark>)]
    mod stark {}
}
