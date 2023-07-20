use generic_ec::{curves::Secp256k1, Point, SecretScalar};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DhSecretKey {
    key: SecretScalar<Secp256k1>,
}

#[wasm_bindgen]
pub struct DhPublicKey {
    point: Point<Secp256k1>,
}

#[wasm_bindgen]
pub struct NonUniformSharedSecret {
    point: Point<Secp256k1>,
}

#[wasm_bindgen]
impl DhSecretKey {
    pub fn random() -> Self {
        Self {
            key: SecretScalar::random(&mut rand_core::OsRng),
        }
    }

    pub fn public_key(&self) -> DhPublicKey {
        DhPublicKey {
            point: Point::generator() * &self.key,
        }
    }

    pub fn derive_shared_secret(
        self,
        other_party_public_key: &DhPublicKey,
    ) -> NonUniformSharedSecret {
        NonUniformSharedSecret {
            point: self.key * other_party_public_key.point,
        }
    }
}

#[wasm_bindgen]
impl DhPublicKey {
    pub fn to_bytes(&self) -> Box<[u8]> {
        self.point.to_bytes(true).to_vec().into_boxed_slice()
    }
}

#[wasm_bindgen]
impl NonUniformSharedSecret {
    pub fn to_bytes(&self) -> Box<[u8]> {
        self.point.to_bytes(true).to_vec().into_boxed_slice()
    }
}
