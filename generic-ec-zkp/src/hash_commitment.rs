//! Hash commitment
//!
//! Hash commitment is a procedure that allows player $\P$ to commit some value (scalar, point, byte string, etc.),
//! and reveal it later on. Player $\V$ can verify that commitment matches revealed value.
//!
//! ## Example
//!
//! 1. $\P$ commits some data (point, scalars, slices, whatever that can be represented as bytes):
//!    ```rust
//!    # use generic_ec::{Point, Scalar, Curve};
//!    # use generic_ec_zkp::hash_commitment::HashCommit;
//!    # use sha2::Sha256; use rand::rngs::OsRng;
//!    #
//!    # fn doc_fn<E: Curve>() {
//!    let point: Point<E> = some_point();
//!    let scalars: &[Scalar<E>] = some_scalars();
//!    let arbitrary_data: &[&[u8]] = some_arbitrary_data();
//!
//!    let (commit, decommit) = HashCommit::<Sha256>::builder()
//!        .mix(point)
//!        .mix_many(scalars)
//!        .mix_many_bytes(arbitrary_data)
//!        .commit(&mut OsRng);
//!    # }
//!    # fn some_point<E: Curve>() -> Point<E> { unimplemented!() }
//!    # fn some_scalars<T>() -> T { unimplemented!() }
//!    # fn some_arbitrary_data<T>() -> T { unimplemented!() }
//!    ```
//! 2. $\P$ sends `commit` to $\V$
//! 3. At some point, $\P$ chooses to reveal committed data. It sends data + `decommit` to $\V$.
//!
//!    $\V$ verifies that revealed data matches `commit`:
//!
//!    ```rust
//!    # use generic_ec::{Point, Scalar, Curve};
//!    # use generic_ec_zkp::hash_commitment::{HashCommit, DecommitNonce, MismatchedRevealedData};
//!    # use sha2::Sha256;
//!    #
//!    # fn doc_fn<E: Curve>() -> Result<(), MismatchedRevealedData> {
//!    # let commit: HashCommit<Sha256> = unimplemented!();
//!    # let (point, scalars, arbitrary_data): (Point<E>, &[Scalar<E>], &[&[u8]]) = unimplemented!();
//!    # let decommit: DecommitNonce<Sha256> = unimplemented!();
//!    HashCommit::<Sha256>::builder()
//!        .mix(point)
//!        .mix_many(scalars)
//!        .mix_many_bytes(arbitrary_data)
//!        .verify(&commit, &decommit)?;
//!    # }
//!    ```
//!
//! ## Algorithm
//! Underlying algorithm is based on hash function $\H$. To commit data, we sample a large random nonce,
//! and hash it along with data. When we hash bytestrings, we prepend its length to it, in that way we
//! ensure that there's only one set of inputs that can be decommitted.
//!
//! Roughly, algorithm is:
//!
//! 1. $commit(i_1, \dots, i_n) =$ \
//!    1. $\mathit{nonce} \gets \\{0,1\\}^k$
//!    2. $\text{return}\ \H(\dots \\| \text{u32\\_to\\_be}(\mathit{len}(i_j)) \\| i_j \\| \dots \\| \mathit{nonce}), \mathit{nonce}$
//!
//! 2. $decommit(commit, nonce, i_1, \dots, i_n) =$
//!    1. $\text{return}\ \H(\dots \\| \text{u32\\_to\\_be}(\mathit{len}(i_j)) \\| i_j \\| \dots \\| nonce) \\? commit$

use digest::{generic_array::GenericArray, Digest, Output};
use rand_core::RngCore;
use subtle::ConstantTimeEq;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use generic_ec::{Curve, Point, Scalar};

/// Builder for commitment/verification
pub struct Builder<D: Digest>(D);

impl<D: Digest> Builder<D> {
    /// Creates an instance of [`Builder`]
    pub fn new() -> Self {
        Self(D::new())
    }

    /// Mixes value serialized to bytes into commitment
    ///
    /// You can use this method with [`Point<E>`](Point) or [`Scalar<E>`](Scalar). Also you can
    /// implement [`EncodesToBytes`] for your own types.
    ///
    /// ```rust
    /// use generic_ec::{Point, Scalar};
    /// use generic_ec_zkp::hash_commitment::HashCommit;
    /// # use sha2::Sha256;
    /// # use rand::rngs::OsRng;
    ///
    /// # use generic_ec::Curve;
    /// # fn doc_fn<E: Curve>() {
    /// let point: Point<E> = some_point();
    /// let scalar: Scalar<E> = some_scalar();
    ///
    /// let (commit, decommit) = HashCommit::<Sha256>::builder()
    ///     .mix(point)
    ///     .mix(scalar)
    ///     .commit(&mut OsRng);
    /// # }
    /// # fn some_point<E: Curve>() -> Point<E> { unimplemented!() }
    /// # fn some_scalar<E: Curve>() -> Scalar<E> { unimplemented!() }
    /// ```
    pub fn mix<T>(self, encodable: T) -> Self
    where
        T: EncodesToBytes,
    {
        self.mix_bytes(encodable.to_bytes())
    }

    /// Mixes values serialized to bytes into commitment
    ///
    /// ```rust
    /// use generic_ec::Point;
    /// use generic_ec_zkp::hash_commitment::HashCommit;
    /// # use sha2::Sha256;
    /// # use rand::rngs::OsRng;
    ///
    /// # use generic_ec::Curve;
    /// # fn doc_fn<E: Curve>() {
    /// let points: Vec<Point<E>> = some_points();
    ///
    /// let (commit, decommit) = HashCommit::<Sha256>::builder()
    ///     .mix_many(&points)
    ///     .commit(&mut OsRng);
    /// # }
    /// # fn some_points<E: Curve>() -> Vec<Point<E>> { unimplemented!() }
    /// ```
    pub fn mix_many(self, encodables: impl IntoIterator<Item = impl EncodesToBytes>) -> Self {
        self.mix_many_bytes(encodables.into_iter().map(|i| i.to_bytes()))
    }

    /// Mixes bytes into commitment
    ///
    /// ```rust
    /// use generic_ec_zkp::hash_commitment::HashCommit;
    /// # use sha2::Sha256;
    /// # use rand::rngs::OsRng;
    ///
    /// # use generic_ec::Curve;
    /// # fn doc_fn<E: Curve>() {
    /// let (commit, decommit) = HashCommit::<Sha256>::builder()
    ///     .mix_bytes(b"some message")
    ///     .commit(&mut OsRng);
    /// # }
    /// ```
    pub fn mix_bytes(self, data: impl AsRef<[u8]>) -> Self {
        Self(self.0.chain_update(D::digest(data)))
    }

    /// Mixes list of byte strings into commitment
    ///
    /// ```rust
    /// use generic_ec_zkp::hash_commitment::HashCommit;
    /// # use sha2::Sha256;
    /// # use rand::rngs::OsRng;
    ///
    /// # use generic_ec::Curve;
    /// # fn doc_fn<E: Curve>() {
    /// let (commit, decommit) = HashCommit::<Sha256>::builder()
    ///     .mix_many_bytes(&[b"some message".as_slice(), b"another message"])
    ///     .commit(&mut OsRng);
    /// # }
    /// ```
    pub fn mix_many_bytes(self, list: impl IntoIterator<Item = impl AsRef<[u8]>>) -> Self {
        let hash = list
            .into_iter()
            .fold(D::new(), |d, i| d.chain_update(D::digest(i)))
            .finalize();
        Self(self.0.chain_update(hash))
    }

    /// Performs commitment
    ///
    /// Decommitment nonce is generated from provided randomness source
    pub fn commit<R: RngCore>(self, rng: &mut R) -> (HashCommit<D>, DecommitNonce<D>) {
        let mut nonce = DecommitNonce::<D>::default();
        rng.fill_bytes(&mut nonce.nonce);
        (self.commit_with_fixed_nonce(&nonce), nonce)
    }

    /// Performs commitment with specified decommitment nonce
    pub fn commit_with_fixed_nonce(mut self, nonce: &DecommitNonce<D>) -> HashCommit<D> {
        self = self.mix_bytes(&nonce.nonce);
        let resulting_hash = self.0.finalize();
        HashCommit(resulting_hash)
    }

    /// Verifies that provided data matches commitment and decommitment
    pub fn verify(
        self,
        commit: &HashCommit<D>,
        nonce: &DecommitNonce<D>,
    ) -> Result<(), MismatchedRevealedData> {
        let should_be = self.commit_with_fixed_nonce(nonce);
        if commit.0.ct_eq(&should_be.0).into() {
            Ok(())
        } else {
            Err(MismatchedRevealedData)
        }
    }
}

impl<D: Digest> Default for Builder<D> {
    fn default() -> Self {
        Self::new()
    }
}

/// Committed value
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(bound = ""))]
pub struct HashCommit<D: Digest>(pub Output<D>);

impl<D: Digest> HashCommit<D> {
    pub fn builder() -> Builder<D> {
        Builder::new()
    }
}

/// Random nonce that was used to "blind" commitment
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(bound = ""))]
pub struct DecommitNonce<D: Digest> {
    pub nonce: GenericArray<u8, D::OutputSize>,
}

impl<D: Digest> Default for DecommitNonce<D> {
    fn default() -> Self {
        Self {
            nonce: Default::default(),
        }
    }
}

/// Infallibly encodable to bytes
///
/// Used in [`hash_commitment::Builder`](Builder) methods [`mix`](Builder::mix) and [`mix_many`](Builder::mix_many) to convert given value to bytes.
pub trait EncodesToBytes {
    /// Value byte representation
    type Bytes: AsRef<[u8]>;
    /// Encodes value to bytes
    fn to_bytes(&self) -> Self::Bytes;
}

impl<T: EncodesToBytes> EncodesToBytes for &T {
    type Bytes = T::Bytes;
    fn to_bytes(&self) -> Self::Bytes {
        <T as EncodesToBytes>::to_bytes(*self)
    }
}

impl<E: Curve> EncodesToBytes for Point<E> {
    type Bytes = generic_ec::EncodedPoint<E>;
    fn to_bytes(&self) -> Self::Bytes {
        self.to_bytes(true)
    }
}

impl<E: Curve> EncodesToBytes for Scalar<E> {
    type Bytes = generic_ec::EncodedScalar<E>;
    fn to_bytes(&self) -> Self::Bytes {
        self.to_be_bytes()
    }
}

impl EncodesToBytes for u16 {
    type Bytes = [u8; 2];
    fn to_bytes(&self) -> Self::Bytes {
        self.to_be_bytes()
    }
}

/// Error indicating that revealed data doesn't match commitment
pub struct MismatchedRevealedData;
