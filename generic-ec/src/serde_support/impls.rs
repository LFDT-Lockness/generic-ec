use serde::{Deserialize, Serialize};
use serde_with::{DeserializeAs, SerializeAs};

use crate::coords::Coordinate;
use crate::Curve;

use super::utils::{expectation, Bytes};

impl<E: Curve> Serialize for Coordinate<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        <Bytes<expectation::Coordinate> as SerializeAs<Coordinate<E>>>::serialize_as(
            self, serializer,
        )
    }
}

impl<'de, E: Curve> Deserialize<'de> for Coordinate<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <Bytes<expectation::Coordinate> as DeserializeAs<'de, Coordinate<E>>>::deserialize_as(
            deserializer,
        )
    }
}
