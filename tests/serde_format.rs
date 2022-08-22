//! These tests are ensuring persistence of serialization format of lib structs.
//! Any changes in the format should trigger a major version change.

use std::iter;

use rand_core::RngCore;
use rand_dev::DevRng;
use serde_test::{assert_de_tokens, assert_tokens, Configure, Token};

use generic_ec::{coords::Coordinate, dummy_curve::DummyCurve};

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
