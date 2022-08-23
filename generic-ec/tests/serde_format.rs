//! These tests are ensuring persistence of serialization format of lib structs.
//! Any changes in the format should trigger a major version change.

use std::iter;

use rand_core::RngCore;
use rand_dev::DevRng;
use serde_test::{assert_de_tokens, assert_tokens, Configure, Token};

use generic_ec::{
    coords::{Coordinate, Parity, Sign},
    dummy_curve::DummyCurve,
};

#[test]
fn coordinate() {
    let mut rng = DevRng::new();

    let zero_coord = Coordinate::<DummyCurve>::default();

    let mut random_coord = Coordinate::<DummyCurve>::default();
    rng.fill_bytes(random_coord.as_mut());

    for coord in [zero_coord, random_coord] {
        // (De)serialization in compact format
        let coord_bytes: &'static [u8] = coord.as_ref().to_vec().leak();
        assert_tokens(&coord.clone().compact(), &[Token::Bytes(coord_bytes)]);

        // (De)serialization in human-readable format
        let coord_hex = Box::leak(hex::encode(coord_bytes).into_boxed_str());
        assert_tokens(&coord.clone().readable(), &[Token::Str(coord_hex)]);

        // Deserialization from sequence
        let tokens = iter::once(Token::Seq {
            len: Some(coord_bytes.len()),
        })
        .chain(coord_bytes.iter().map(|b| Token::U8(*b)))
        .chain(iter::once(Token::SeqEnd))
        .collect::<Vec<_>>();

        assert_de_tokens(&coord.readable(), &tokens);
    }
}

#[test]
fn parity() {
    let possible_values = [(Parity::Odd, 0, "Odd"), (Parity::Even, 1, "Even")];
    for (parity, _repr_int, repr_str) in possible_values {
        // TODO: `_repr_int` is not asserted due to limitation of `serde_test`
        // See: https://github.com/serde-rs/serde/issues/2265

        // (De)serialization in compact format
        assert_tokens(
            &parity.compact(),
            &[Token::UnitVariant {
                name: "Parity",
                variant: repr_str,
            }],
        );

        // (De)serialization in human-readable format
        assert_tokens(
            &parity.readable(),
            &[Token::UnitVariant {
                name: "Parity",
                variant: repr_str,
            }],
        );
    }
}

#[test]
fn sign() {
    let possible_values = [
        (Sign::Negative, 0, "Negative"),
        (Sign::NonNegative, 1, "NonNegative"),
    ];
    for (sign, _repr_int, repr_str) in possible_values {
        // TODO: `_repr_int` is not asserted due to limitation of `serde_test`
        // See: https://github.com/serde-rs/serde/issues/2265

        // (De)serialization in compact format

        assert_tokens(
            &sign.compact(),
            &[Token::UnitVariant {
                name: "Sign",
                variant: repr_str,
            }],
        );

        // (De)serialization in human-readable format
        assert_tokens(
            &sign.readable(),
            &[Token::UnitVariant {
                name: "Sign",
                variant: repr_str,
            }],
        );
    }
}
