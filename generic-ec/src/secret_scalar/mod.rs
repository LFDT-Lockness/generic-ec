use rand_core::{CryptoRng, RngCore};

use crate::Curve;

use self::definition::SecretScalar;

pub mod definition;

impl<E: Curve> SecretScalar<E> {
    pub fn random<R: RngCore + CryptoRng>() -> Self {
        todo!()
    }
}
