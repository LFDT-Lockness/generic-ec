//! Knowledge of equality of two discrete logarithms $\Pi^\text{dlog_eq}$
//!
//! This is a protocol that lets the prover $\P$ convince the
//! verifier $\V$ that it knows the discrete logarithms of two values, and that
//! those logarithms are equal. $\P$ knows the secret data $x$ - the discrete
//! logarithm. $\P$ and $\V$ share common data:
//! - $G$, $H$ - some generators of the elliptic curve
//! - $X = x \cdot G$
//! - $Z = x \cdot H$
//!
//! Thus $log_G X = log_H Z = x$. Then $\P$ proves this fact with this protocol.  
//! A common instantiation is when $x$ is a private key. Then $X$ is a public
//! key, a fact known to both parties, and the proof is that $H \cdot x = Z$
//! without disclosing $x$.
//!
//! Since in this library we use additive notation, some terminology has been
//! adjusted in function arguments, for example what would be an `exponent` is
//! called a `product`.
//!
//! ## Non-interactive example
//!
//! ```rust
//! # use generic_ec::{Curve, Scalar, SecretScalar, Point};
//! # use generic_ec_zkp::dlog_eq::non_interactive;
//! # use rand::rngs::OsRng;
//! # use generic_ec::curves::Secp256k1 as E;
//! # fn send<T>(_: T) {}
//!
//! // `x` is a secret discrete logarithm only known to prover
//! let x = SecretScalar::<E>::random(&mut OsRng);
//!
//! // `X` is a public point corresponding to `x`
//! let X = Point::generator() * &x;
//! // `H` is some point known to both parties
//! let H = Point::generator() * Scalar::random(&mut OsRng);
//! // `Z` is the object of the proof
//! let Z = H * &x;
//!
//! let data = non_interactive::Data::from_secret_key(&x, H);
//! assert_eq!(Z, data.prod2);
//!
//! // Prover proves in zero knowledge the equality of `H * x = Z`
//! let proof = non_interactive::prove::<E, sha2::Sha256>(&mut OsRng, &"shared_state", &x, data);
//! // The proof is sent to the Verifier
//! send(proof);
//!
//! // Verifier checks that the proof is correct
//! non_interactive::verify::<E, sha2::Sha256>(&"shared_state", data, proof)
//!     .expect("Verification failed!");
//! ```
//!
//! ## Interactive example
//!
//! ```rust
//! # use generic_ec::{Curve, Scalar, SecretScalar, Point};
//! # use generic_ec_zkp::dlog_eq::interactive;
//! # use rand::rngs::OsRng;
//! # use generic_ec::curves::Secp256k1 as E;
//! # fn send<T>(_: T) {}
//!
//! // `x` is a secret discrete logarithm only known to prover
//! let x = SecretScalar::<E>::random(&mut OsRng);
//!
//! // `X` is a public point corresponding to `x`
//! let X = Point::generator() * &x;
//! // `H` is some point known to both parties
//! let H = Point::generator() * Scalar::random(&mut OsRng);
//! // `Z` is the object of the proof
//! let Z = H * &x;
//!
//! let data = interactive::Data::from_secret_key(&x, H);
//! assert_eq!(Z, data.prod2);
//!
//! // Prover commits to the data
//! let (commitment, private_commitment) = interactive::commit_data(&mut OsRng, data);
//! // Prover sends this commitment to the verifier
//! send(commitment);
//!
//! // Verifier sends a challenge to the prover
//! let challenge = Scalar::random(&mut OsRng);
//! send(challenge);
//!
//! // Prover receives the challenge and computes the proof of equality `H * x = Z`
//! let proof = interactive::prove(private_commitment, challenge, &x);
//! // This proof is sent to the verifier
//! send(proof);
//!
//! // Verifier checks that the proof is correct
//! interactive::verify(data, commitment, challenge, proof)
//!     .expect("Verification failed!");
//! ```

use generic_ec::{Curve, NonZero, Point};

/// Object of the proof: `log_gen1 prod1 == log_gen2 prod2`
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "udigest", derive(udigest::Digestable), udigest(bound = ""))]
pub struct Data<E: Curve> {
    /// `G`, a generator, in multiplicative notation the base for one logarithm
    pub gen1: NonZero<Point<E>>,
    /// `X`, a point `G * x`, in multiplicative notation the value inside one logarithm
    pub prod1: Point<E>,
    /// `H`, a generator, in multiplicative notation the base for the other logarithm
    pub gen2: NonZero<Point<E>>,
    /// `Z`, a point `H * x`, in multiplicative notation the value inside the other logarithm
    pub prod2: Point<E>,
}

impl<E: Curve> Data<E> {
    /// Create the data for the common case where `G` is the principal group
    /// generator, and `x` is a secret key
    ///
    /// In this case, we set `H = gen` as gen2 and `Z = G * x` as prod2
    pub fn from_secret_key(x: &generic_ec::SecretScalar<E>, gen: NonZero<Point<E>>) -> Data<E> {
        Self {
            gen1: Point::generator().to_nonzero_point(),
            prod1: Point::generator() * x,
            gen2: gen,
            prod2: gen * x,
        }
    }
}

/// Functions for interactive protocol for the proof. See usage example in
/// [`dlog_eq`](crate::dlog_eq)
///
/// Based on "Zero-Knowledge Proofs Notes", 2024 by Jorge L. Villar
pub mod interactive {
    use generic_ec::{Curve, Point, Scalar, SecretScalar};

    /// Commitment to `H`, sent in the first round
    pub type Commitment<E> = (Point<E>, Point<E>);
    /// Private part of commitment, the nonce `r`. Kept by the Prover
    pub type PrivateCommitment<E> = Scalar<E>;
    /// Challenge, sent to the Prover in the first round
    pub type Challenge<E> = Scalar<E>;
    /// Proof data, sent by prover to the verifier
    pub type Proof<E> = Scalar<E>;
    pub use super::Data;
    pub use super::InvalidProof;

    /// First round of the protocol: commit to `H` by itself. Produces
    /// [`Commitment`] to be sent
    ///
    /// `rng` is used to generate the nonce for the private commitment
    pub fn commit<E: Curve>(
        rng: &mut (impl rand_core::RngCore + rand_core::CryptoRng),
        gen1: NonZero<Point<E>>,
        gen2: NonZero<Point<E>>,
    ) -> (Commitment<E>, PrivateCommitment<E>) {
        let r = Scalar::random(rng);
        let comm = (gen1 * r, gen2 * r);
        (comm, r)
    }
    /// First round of the protocol: commit to well-constructed [`Data`] by
    /// itself. Produces [`Commitment`] to be sent
    ///
    /// `rng` is used to generate the nonce for the private commitment
    pub fn commit_data<E: Curve>(
        rng: &mut (impl rand_core::RngCore + rand_core::CryptoRng),
        data: Data<E>,
    ) -> (Commitment<E>, PrivateCommitment<E>) {
        commit(rng, data.gen1, data.gen2)
    }

    /// First round of the protocol: verifier generates a challenge. A challenge
    /// is simply a random scalar
    pub fn challenge<E: Curve>(
        rng: &mut (impl rand_core::RngCore + rand_core::CryptoRng),
    ) -> Challenge<E> {
        Scalar::random(rng)
    }

    /// Second round of the protocol: produce the proof for [`Data`] with the
    /// given private commitment, public part of which was communicated to the
    /// verifier
    ///
    /// - `r` - private commitment, the same as given by [`commit`] or [`commit_data`]
    /// - `challenge` - challenge sent by the Verifier
    /// - `x` - secret value of discrete logarithm
    pub fn prove<E: Curve>(
        r: PrivateCommitment<E>,
        challenge: Scalar<E>,
        x: &SecretScalar<E>,
    ) -> Proof<E> {
        r + challenge * x
    }

    /// Verify the validity of the ZK proof
    ///
    /// - `data` - data for which the proof was computed
    /// - `comm` - commitment sent by the Prover in the first round
    /// - `challenge` - the same challenge as was sent by this verifier in the
    ///   first round
    /// - `proof` - produced by the Prover in the second round for this data,
    ///   commitment and challenge
    pub fn verify<E: Curve>(
        data: Data<E>,
        comm: Commitment<E>,
        challenge: Scalar<E>,
        proof: Proof<E>,
    ) -> Result<(), InvalidProof> {
        let (a1, a2) = comm;
        // equation 1
        let lhs = data.gen1 * proof;
        let rhs = a1 + data.prod1 * challenge;
        if lhs != rhs {
            return Err(InvalidProof);
        }
        // equation 2
        let lhs = data.gen2 * proof;
        let rhs = a2 + data.prod2 * challenge;
        if lhs != rhs {
            return Err(InvalidProof);
        }

        Ok(())
    }
}

/// Functions for non interactive variant of the proof. See usage example in
/// [`dlog_eq`](crate::dlog_eq)
///
/// Compared to naive protocol produced by Fiat-Shamir heuristic, this is
/// optimized to send less data in the proof. It's based on `PrEq` functionality
/// in this paper: <https://eprint.iacr.org/2020/096>
#[cfg(feature = "udigest")]
pub mod non_interactive {
    use generic_ec::{Curve, Scalar, SecretScalar};

    const TAG: &str = "generic-ec-zkp.dlog_eq.non_interactive";

    pub use super::Data;
    pub use super::InvalidProof;

    /// Proof data, sent by prover to the verifier
    #[derive(Debug, Clone, Copy)]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Serialize, serde::Deserialize),
        serde(bound = "")
    )]
    pub struct Proof<E: generic_ec::Curve> {
        /// Deterministic challenge
        pub ch: generic_ec::Scalar<E>,
        /// Proof itself
        pub res: generic_ec::Scalar<E>,
    }

    /// Create a proof deterministically for given data
    ///
    /// - `shared_state` - shared data not known to this proof, used to protect
    ///   from replay attacks
    /// - `x` - the secret value of discrete logarithm
    /// - `data` - data to compute the proof for
    /// - `rng` - used to generate a random nonce for proof
    pub fn prove<E: Curve, D: digest::Digest>(
        rng: &mut (impl rand_core::RngCore + rand_core::CryptoRng),
        shared_state: &impl udigest::Digestable,
        x: &SecretScalar<E>,
        data: Data<E>,
    ) -> Proof<E> {
        let r = Scalar::random(rng);
        let com1 = data.gen1 * r;
        let com2 = data.gen2 * r;

        let seed = udigest::inline_struct!(TAG {
            shared_state,
            data,
            com1,
            com2,
        });
        let ch = Scalar::from_hash::<D>(&seed);

        let res = r + x * ch;
        Proof { ch, res }
    }

    /// Verify the validity of the ZK proof
    ///
    /// - `shared_state` - shared data not known to this proof, used to protect
    ///   from replay attacks
    /// - `data` - data for which the proof was computed
    /// - `proof` - produced by the Prover in the second round for this data and
    ///   shared_state
    pub fn verify<E: Curve, D: digest::Digest>(
        shared_state: &impl udigest::Digestable,
        data: Data<E>,
        proof: Proof<E>,
    ) -> Result<(), InvalidProof> {
        let com1 = data.gen1 * proof.res - data.prod1 * proof.ch;
        let com2 = data.gen2 * proof.res - data.prod2 * proof.ch;

        let seed = udigest::inline_struct!(TAG {
            shared_state,
            data,
            com1,
            com2,
        });
        let ch = Scalar::from_hash::<D>(&seed);

        if ch != proof.ch {
            Err(InvalidProof)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct InvalidProof;
