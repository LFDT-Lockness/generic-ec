//! Hash-based Commitment Scheme $\mathcal{F}_\text{com}$
//!
//! A statistically hiding, computationally binding commitment scheme over arbitrary
//! digestable values. The committer chooses a random nonce $r$ and publishes
//! $\text{com} = H(\text{tag} \| r \| \text{msg})$; the opening reveals $r$ so
//! any verifier can recompute and check the commitment.
//!
//! This primitive is used in threshold protocols such as DKLs23 key generation,
//! where each party commits to its key-share contribution before revealing it,
//! preventing a rushing adversary from biasing the combined key.
//!
//! ## Example
//!
//! ```rust
//! # use generic_ec::curves::Secp256k1;
//! # use generic_ec::{Point, Scalar};
//! # use generic_ec_zkp::hash_commitment;
//! # use rand::rngs::OsRng;
//! # use sha2::Sha256;
//!
//! // Committer picks a value and commits to it
//! let secret_point = Point::<Secp256k1>::generator().to_point();
//! let (commitment, opening) = hash_commitment::commit::<Sha256, _>(&secret_point, &mut OsRng);
//!
//! // Later, the committer reveals the value and the opening
//! hash_commitment::verify::<Sha256, _>(&commitment, &secret_point, &opening)
//!     .expect("commitment verification failed");
//! ```
//!
//! ## Security
//!
//! - **Hiding**: the commitment reveals nothing about `msg` beyond what `H` leaks,
//!   which is nothing under the random-oracle assumption. The 32-byte random nonce
//!   provides $2^{128}$ bits of hiding security even when `msg` is low-entropy.
//! - **Binding**: finding a collision under `H` is required to open the same
//!   commitment to a different message.

use digest::Digest;
use rand_core::{CryptoRng, RngCore};
use subtle::ConstantTimeEq;

const NONCE_SIZE: usize = 32;

/// A commitment to some value, produced by [`commit`].
///
/// Internally stores the hash output `H(tag || nonce || msg)`.
#[derive(Clone, Debug)]
pub struct HashCommitment<D: Digest>(digest::Output<D>);

#[cfg(feature = "serde")]
impl<D: Digest> serde::Serialize for HashCommitment<D> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.0.as_slice())
    }
}

#[cfg(feature = "serde")]
impl<'de, D: Digest> serde::Deserialize<'de> for HashCommitment<D> {
    fn deserialize<De: serde::Deserializer<'de>>(deserializer: De) -> Result<Self, De::Error> {
        use serde::de::Error;

        struct BytesVisitor<D>(core::marker::PhantomData<D>);

        impl<'de, D: Digest> serde::de::Visitor<'de> for BytesVisitor<D> {
            type Value = HashCommitment<D>;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "byte array of length {}", <D as Digest>::output_size())
            }

            fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                if v.len() != <D as Digest>::output_size() {
                    return Err(E::invalid_length(v.len(), &self));
                }
                let mut out = digest::Output::<D>::default();
                out.copy_from_slice(v);
                Ok(HashCommitment(out))
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Self::Value, A::Error> {
                let mut out = digest::Output::<D>::default();
                for (i, byte) in out.iter_mut().enumerate() {
                    *byte = seq
                        .next_element()?
                        .ok_or_else(|| A::Error::invalid_length(i, &self))?;
                }
                Ok(HashCommitment(out))
            }
        }

        deserializer.deserialize_bytes(BytesVisitor(core::marker::PhantomData))
    }
}

/// The opening (randomness) corresponding to a [`HashCommitment`].
///
/// Must be kept secret until the committer is ready to reveal. Passed to
/// [`verify`] together with the original message to confirm the commitment.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Opening([u8; NONCE_SIZE]);

/// Commits to `msg` under a freshly sampled random nonce.
///
/// Returns the public [`HashCommitment`] and the secret [`Opening`]. The
/// commitment should be broadcast immediately; the opening is revealed only
/// after all parties have posted their commitments.
pub fn commit<D, V>(msg: &V, rng: &mut (impl RngCore + CryptoRng)) -> (HashCommitment<D>, Opening)
where
    D: Digest,
    V: udigest::Digestable,
{
    let mut nonce = [0u8; NONCE_SIZE];
    rng.fill_bytes(&mut nonce);
    let output = hash::<D, V>(msg, &nonce);
    (HashCommitment(output), Opening(nonce))
}

/// Verifies that `commitment` was produced by committing to `msg` with `opening`.
///
/// Returns `Ok(())` on success, or [`InvalidOpening`] if the commitment does not
/// match. The comparison is performed in constant time to avoid timing side-channels.
pub fn verify<D, V>(
    commitment: &HashCommitment<D>,
    msg: &V,
    opening: &Opening,
) -> Result<(), InvalidOpening>
where
    D: Digest,
    V: udigest::Digestable,
{
    let expected = hash::<D, V>(msg, &opening.0);
    if commitment.0.ct_eq(&expected).into() {
        Ok(())
    } else {
        Err(InvalidOpening)
    }
}

fn hash<D, V>(msg: &V, nonce: &[u8; NONCE_SIZE]) -> digest::Output<D>
where
    D: Digest,
    V: udigest::Digestable,
{
    let tagged = udigest::inline_struct!("generic-ec-zkp.hash_commitment" {
        nonce,
        msg,
    });
    udigest::hash::<D>(&tagged)
}

/// Error returned when an opening does not match the commitment.
#[derive(Debug, Clone, Copy)]
pub struct InvalidOpening;

impl core::fmt::Display for InvalidOpening {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("commitment opening is invalid")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidOpening {}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_ec::{curves::Secp256k1, Point, Scalar};
    use rand::rngs::OsRng;
    use sha2::Sha256;

    #[test]
    fn commit_then_verify_succeeds() {
        let point = Point::<Secp256k1>::generator().to_point();
        let (commitment, opening) = commit::<Sha256, _>(&point, &mut OsRng);
        verify::<Sha256, _>(&commitment, &point, &opening).expect("valid opening must verify");
    }

    #[test]
    fn wrong_message_is_rejected() {
        let point = Point::<Secp256k1>::generator().to_point();
        let other = Point::<Secp256k1>::generator() * Scalar::random(&mut OsRng);
        let (commitment, opening) = commit::<Sha256, _>(&point, &mut OsRng);
        assert!(
            verify::<Sha256, _>(&commitment, &other, &opening).is_err(),
            "commitment to a different point must not verify"
        );
    }

    #[test]
    fn wrong_opening_is_rejected() {
        let point = Point::<Secp256k1>::generator().to_point();
        let (commitment, _opening) = commit::<Sha256, _>(&point, &mut OsRng);
        let (_other_commitment, other_opening) = commit::<Sha256, _>(&point, &mut OsRng);
        assert!(
            verify::<Sha256, _>(&commitment, &point, &other_opening).is_err(),
            "commitment opened with the wrong nonce must not verify"
        );
    }

    #[test]
    fn two_commits_to_same_value_differ() {
        let point = Point::<Secp256k1>::generator().to_point();
        let (com1, _) = commit::<Sha256, _>(&point, &mut OsRng);
        let (com2, _) = commit::<Sha256, _>(&point, &mut OsRng);
        // Commitments should be different due to fresh randomness
        assert_ne!(com1.0.as_slice(), com2.0.as_slice());
    }
}
