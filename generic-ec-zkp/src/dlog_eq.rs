//! Knowledge of equality of two discrete logarithms $\Pi^\text{dlog_eq}$
//!
//! This is a protocol that lets the prover $\P$ convince the
//! verifier $\V$ that it knows the discrete logarithms of two values, and that
//! those logarithms are equal. $\P$ knows the secret data $x$ - the discrete
//! logarithm. $\P$ and $\V$ share common data:
//! - $H$ - a point on an elliptic curve
//! - $X = x \cdot G$
//! - $Z = x \cdot H$
//!
//! Thus $log_G X = log_H Z = x$. Then $\P$ proves this fact with this protocol.  
//! A common instantiation is when $x$ is a private key. Then $X$ is a public
//! key, a fact known to both parties, and the proof is that $H \cdot x = Z$
//! without disclosing $x$.
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
//! let data = non_interactive::Data {
//!     pub_share: X,
//!     base: H,
//!     exp: Z,
//! };
//!
//! // Prover chooses a private nonce
//! let r = Scalar::random(&mut OsRng);
//! // Prover proves in zero knowledge the equality of `H * x = Z`
//! let proof = non_interactive::prove::<sha2::Sha256, E>(&"shared_state", &x, data, r);
//! // The proof is sent to the Verifier
//! send(proof);
//!
//! // Verifier checks that the proof is correct
//! non_interactive::verify::<sha2::Sha256, E>(&"shared_state", data, proof)
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
//! let data = interactive::Data {
//!     pub_share: X,
//!     base: H,
//!     exp: Z,
//! };
//!
//! // Prover chooses a nonce for itself
//! let r = Scalar::random(&mut OsRng);
//! // Prover commits to the data
//! let commitment = interactive::commit_data(data, r);
//! // Prover sends this commitment to the verifier
//! send(commitment);
//!
//! // Verifier sends a challenge to the prover
//! let challenge = Scalar::random(&mut OsRng);
//! send(challenge);
//!
//! // Prover receives the challenge and computes the proof of equality `H * x = Z`
//! let proof = interactive::prove(r, challenge, &x);
//! // This proof is sent to the verifier
//! send(proof);
//!
//! // Verifier checks that the proof is correct
//! interactive::verify(data, commitment, challenge, proof)
//!     .expect("Verification failed!");
//! ```

use generic_ec::{Curve, Point};

/// Object of the proof
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "udigest", derive(udigest::Digestable), udigest(bound = ""))]
pub struct Data<E: Curve> {
    /// `X`, the value inside `log_G`
    pub pub_share: Point<E>,
    /// `H`, the base for the second logarithm
    pub base: Point<E>,
    /// `Z`, the value inside the second logarithm `log_H`
    pub exp: Point<E>,
}

/// Functions for interactive protocol for the proof. See usage example in
/// [`dlog_eq`](crate::dlog_eq)
///
/// Based on "Zero-Knowledge Proofs Notes", 2024 by Jorge L. Villar
pub mod interactive {
    use generic_ec::{Curve, Point, Scalar, SecretScalar};

    /// Commitment to `H`, sent in the first round
    pub type Commitment<E> = (Point<E>, Point<E>);
    /// Challenge, sent to the Prover in the first round
    pub type Challenge<E> = Scalar<E>;
    /// Proof data, sent by prover to the verifier
    pub type Proof<E> = Scalar<E>;
    pub use super::Data;
    pub use super::InvalidProof;

    /// First round of the protocol: commit to `H` by itself. Produces
    /// [`Commitment`] to be sent
    ///
    /// - `r` - random nonce. Can be produced by `Scalar::random(&mut rng)`
    pub fn commit<E: Curve>(base: Point<E>, r: Scalar<E>) -> Commitment<E> {
        (Point::generator() * r, base * r)
    }
    /// First round of the protocol: commit to well-constructed [`Data`] by
    /// itself. Produces [`Commitment`] to be sent
    ///
    /// - `r` - random nonce. Can be produced by `Scalar::random(&mut rng)`
    pub fn commit_data<E: Curve>(data: Data<E>, r: Scalar<E>) -> Commitment<E> {
        commit(data.base, r)
    }

    /// Second round of the protocol: produce the proof for [`Data`] commited to
    /// with the given nonce `r`
    ///
    /// - `r` - nonce, the same as used in [`commit`] or [`commit_data`]
    /// - `challenge` - challenge sent by the Verifier
    /// - `x` - secret value of discrete logarithm
    pub fn prove<E: Curve>(r: Scalar<E>, challenge: Scalar<E>, x: &SecretScalar<E>) -> Proof<E> {
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
        let lhs = Point::generator() * proof;
        let rhs = a1 + data.pub_share * challenge;
        if lhs != rhs {
            return Err(InvalidProof);
        }
        // equation 2
        let lhs = data.base * proof;
        let rhs = a2 + data.exp * challenge;
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
/// optimized to send less data in proof. It's based on `PrEq` functionality in
/// this paper: <https://eprint.iacr.org/2020/096>
#[cfg(feature = "udigest")]
pub mod non_interactive {
    use generic_ec::{Curve, Point, Scalar, SecretScalar};

    const TAG: &str = "bls-style-digest.zkp";

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
    /// - `share` - `x`, secret value of discrete logarithm
    /// - `data` - data to compute the proof for
    /// - `r` - random nonce. Can be produced by `Scalar::random(&mut rng)`
    pub fn prove<D: digest::Digest, E: Curve>(
        shared_state: &impl udigest::Digestable,
        share: &SecretScalar<E>,
        data: Data<E>,
        r: Scalar<E>,
    ) -> Proof<E> {
        let com1 = Point::generator() * r;
        let com2 = data.base * r;

        let seed = udigest::inline_struct!(TAG {
            shared_state,
            data,
            com1,
            com2,
        });
        let mut rng = rand_hash::HashRng::<D, _>::from_seed(seed);
        let ch = Scalar::random(&mut rng);

        let res = r + share * ch;
        Proof { ch, res }
    }

    /// Verify the validity of the ZK proof
    ///
    /// - `shared_state` - shared data not known to this proof, used to protect
    ///   from replay attacks
    /// - `data` - data for which the proof was computed
    /// - `proof` - produced by the Prover in the second round for this data and
    ///   shared_state
    pub fn verify<D: digest::Digest, E: Curve>(
        shared_state: &impl udigest::Digestable,
        data: Data<E>,
        proof: Proof<E>,
    ) -> Result<(), InvalidProof> {
        let com1 = Point::generator() * proof.res - data.pub_share * proof.ch;
        let com2 = data.base * proof.res - data.exp * proof.ch;

        let seed = udigest::inline_struct!(TAG {
            shared_state,
            data,
            com1,
            com2,
        });
        let mut rng = rand_hash::HashRng::<D, _>::from_seed(seed);
        let ch = Scalar::random(&mut rng);

        if ch != proof.ch {
            Err(InvalidProof)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct InvalidProof;
