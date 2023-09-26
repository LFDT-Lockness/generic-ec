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

            // Human-readable, uncompressed
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

            // Human-readable, compressed
            serde_test::assert_ser_tokens(
                &Compressed(point).readable(),
                &[
                    Token::NewtypeStruct {
                        name: "PointCompact",
                    },
                    Token::Str(point_compressed_hex),
                ],
            );

            // Binary, uncompressed
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

            // Binary, compressed
            serde_test::assert_ser_tokens(
                &Compressed(point).compact(),
                &[
                    Token::NewtypeStruct {
                        name: "PointCompact",
                    },
                    Token::Bytes(point_compressed),
                ],
            );
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

            // Uncompressed, hex-encoding
            serde_test::assert_de_tokens(
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
            // Uncompressed, seq-encoded
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
            }
            // Uncompressed, bytes-encoded
            serde_test::assert_de_tokens(
                &point.readable(),
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

            // Compressed, hex-encoding
            serde_test::assert_de_tokens(
                &Compressed(point).readable(),
                &[
                    Token::NewtypeStruct {
                        name: "PointCompact",
                    },
                    Token::Str(point_compressed_hex),
                ],
            );
            // Compressed, seq-encoded
            {
                let mut tokens = vec![
                    Token::NewtypeStruct {
                        name: "PointCompact",
                    },
                    Token::Seq { len: None },
                ];
                tokens.extend(point_compressed.iter().copied().map(Token::U8));
                tokens.push(Token::SeqEnd);
                serde_test::assert_de_tokens(&Compressed(point).readable(), &tokens);
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    struct Compressed<T>(T);
    impl<T> serde::Serialize for Compressed<T>
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
    impl<'de, T> serde::Deserialize<'de> for Compressed<T>
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

    #[instantiate_tests(<generic_ec::curves::Secp256k1>)]
    mod secp256k1 {}

    #[instantiate_tests(<generic_ec::curves::Secp256r1>)]
    mod secp256r1 {}

    #[instantiate_tests(<generic_ec::curves::Stark>)]
    mod stark {}
}
