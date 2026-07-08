//! ![License](https://img.shields.io/crates/l/generic-ec.svg)
//! [![Docs](https://docs.rs/generic-ec/badge.svg)](https://docs.rs/generic-ec)
//! [![Crates io](https://img.shields.io/crates/v/generic-ec.svg)](https://crates.io/crates/generic-ec)
//! [![Discord](https://img.shields.io/discord/905194001349627914?logo=discord&logoColor=ffffff&label=Discord)][in Discord]
//! [![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/LFDT-Lockness/generic-ec/badge)](https://scorecard.dev/viewer/?uri=github.com/LFDT-Lockness/generic-ec)
//!
//! # General elliptic curve cryptography
//!
//! The library provides a set of simple abstractions boosting experience of doing elliptic curve arithmetic
//! in Rust. Aim is to **stay simple**, **generic**, and **secure**. It's handy for developers who implement MPC,
//! zero-knowledge protocols, or any other elliptic crypto algorithms.
//!
//! `generic-ec` is `no_std` and web assembly friendly.
//!
//! ## Overview
//!
//! Crate provides three primitives: a point on elliptic curve [`Point<E>`](Point), an integer modulus group order
//! [`Scalar<E>`](Scalar), and a secret scalar carrying some sensitive value (e.g. secret key) [`SecretScalar<E>`](SecretScalar).
//! `E` stands for a choice of elliptic curve, it could be any [supported curve][supported curves], e.g. `Point<Secp256k1>`
//! is an elliptic point on secp256k1 curve.
//!
//! ## Exposed API
//!
//! Limited API is exposed: elliptic point arithmetic (points addition, negation, multiplying at scalar), scalar
//! arithmetic (addition, multiplication, inverse modulo prime group order), and encode/decode to bytes representation.
//!
//! Hash to curve, hash to scalar primitives, accessing affine coordinates of points are available for some curves through
//! `FromHash` and other traits.
//!
//! ## Security & guarantees
//!
//! Library mitigates a bunch of attacks (such as small-group attack) by design by enforcing following checks:
//! * Scalar `Scalar<E>` must be an integer modulo curve prime order
//! * Elliptic point `Point<E>` must be on the curve \
//!   I.e. elliptic point is guaranteed to satisfy equation of `E`
//! * `Point<E>` is torsion-free \
//!   Elliptic points should be free of small-group component. This eliminates any kind of small-group attacks.
//!
//! Point or scalar not meeting above requirements cannot be constructed (in safe Rust), as these checks are
//! always enforced. E.g. if you're deserializing a sequence of bytes that represents an invalid point,
//! deserialization will result into error.
//!
//! ### `SecretScalar<E>`
//!
//! Sometimes your scalar represents some sensitive value like secret key, and you want to keep it safer.
//! `SecretScalar<E>` is in-place replacement of `Scalar<E>` that enforces additional security by storing
//! the scalar value on the heap, and erasing it on drop. Its advantage is that it doesn't leave any trace
//! in memory dump after it's dropped (which is not guaranteed by regular `Scalar<E>`).
//!
//! But keep in mind that we can't control the OS which could potentially load RAM page containing sensitive value
//! to the swap disk (i.e. on your HDD/SSD) if you're running low on memory. Or it could do any other fancy stuff.
//! We avoid writing unsafe or OS-specific code that could mitigate this problem.
//!
//! ### Points at infinity
//!
//! It should be noticed that point at infinity (or identity point) is a valid `Point<E>`. You can construct it by calling
//! `Point::<E>::zero()`, e.g. `Point::<Secp256k1>::zero()` is a point at infinity for secp256k1 curve.
//!
//! If the protocol you're implementing requires points/scalars to be non-zero, you may need to enforce this check by calling
//! `.is_zero()` method or by using [`NonZero<T>`](NonZero) (`NonZero<Point<E>>` or `NonZero<Scalar<E>>`).
//!
//! Using `NonZero<T>` gives some compile-time guarantees. For instance, multiplying non-zero point in the prime group at
//! non-zero scalar mod group order is mathematically guaranteed to output non-zero point in that prime group. Thus,
//! multiplying `NonZero<Point<E>>` at `NonZero<Scalar<E>>` returns `NonZero<Point<E>>`.
//!
//!
//! ## Supported curves
//!
//! Crate provides support for following elliptic curves out of box:
//!
//! | Curve        | Feature            | Backend                  |
//! |--------------|--------------------|--------------------------|
//! | secp256k1    | `curve-secp256k1`  | [RustCrypto/k256]        |
//! | secp256r1    | `curve-secp256r1`  | [RustCrypto/p256]        |
//! | secp384r1    | `curve-secp384r1`  | [RustCrypto/p384]        |
//! | stark-curve  | `curve-stark`      | [Dfns/stark]             |
//! | Ed25519      | `curve-ed25519`    | [curve25519-dalek]       |
//! | Ed448        | `curve-ed448`      | [ed448-goldilocks-plus]  |
//!
//! [RustCrypto/k256]: https://github.com/RustCrypto/elliptic-curves/tree/master/k256
//! [RustCrypto/p256]: https://github.com/RustCrypto/elliptic-curves/tree/master/p256
//! [RustCrypto/p384]: https://github.com/RustCrypto/elliptic-curves/tree/master/p384
//! [Dfns/stark]: https://github.com/LFDT-Lockness/stark-curve/
//! [curve25519-dalek]: https://docs.rs/curve25519-dalek/
//! [ed448-goldilocks-plus]: https://crates.io/crates/ed448-goldilocks-plus
//!
//! In order to use one of the supported curves, you need to turn on corresponding feature. E.g. if you want
//! to use secp256k1 curve, add this to Cargo.toml:
//!
//! ```toml
//! [dependency]
//! generic-ec = { version = "...", features = ["curve-secp256k1"] }
//! ```
//!
//! And now you can generate a point on that curve:
//!
//! ```rust
//! use generic_ec::{Point, Scalar, curves::Secp256k1};
//! # let mut rng = rand::rngs::OsRng;
//!
//! let random_point: Point<Secp256k1> = Point::generator() * Scalar::random(&mut rng);
//! ```
//!
//! ### Adding support for other curves
//!
//! Adding new curve is as easy as implementing [`Curve` trait](Curve)! If you're missing some curve support,
//! or you're not fine with using existing implementation, you may define your implementation of `Curve` trait
//! and enjoy using the same handy primitives `Point<YOUR_EC>`, `Scalar<YOUR_EC>`, and etc.
//!
//! ## Features
//!
//! * `curve-{name}` enables specified curve support. See list of [supported curves].
//! * `all-curves` enables all supported curves
//! * `serde` enables points/scalar (de)serialization support. (enabled by default)
//! * `std` enables support of standard library (enabled by default)
//!
//! ## Examples
//!
//! ### Random scalar / point generation
//!
//! ```rust
//! use generic_ec::{Point, Scalar, curves::Secp256k1};
//! # let mut rng = rand::rngs::OsRng;
//!
//! // Generates random non-zero scalar
//! let random_scalar = Scalar::<Secp256k1>::random(&mut rng);
//! // Produces a point that's result of generator multiplied at the random scalar
//! let point = Point::generator() * &random_scalar;
//! ```
//!
//! ### Diffie-Hellman key exchange
//!
//! ```rust
//! use generic_ec::{Point, SecretScalar, curves::Secp256k1};
//! # let mut rng = rand::rngs::OsRng;
//!
//! let alice_sk = SecretScalar::<Secp256k1>::random(&mut rng);
//! let alice_pk = Point::generator() * &alice_sk;
//!
//! let bob_sk = SecretScalar::<Secp256k1>::random(&mut rng);
//! let bob_pk = Point::generator() * &bob_sk;
//!
//! let shared_secret_learned_by_alice = bob_pk * &alice_sk;
//! let shared_secret_learned_by_bob = alice_pk * &bob_sk;
//! assert_eq!(shared_secret_learned_by_alice, shared_secret_learned_by_bob);
//! ```
//!
//! ### Generic over choice of curve
//!
//! You can simply make your function generic over choice of curve:
//!
//! ```rust
//! use generic_ec::{Point, Scalar, Curve};
//! use rand::RngCore;
//!
//! fn some_generic_computation<E: Curve>(rng: &mut impl RngCore, point: Point<E>) -> Point<E> {
//!     let blinding = Point::<E>::generator() * Scalar::random(rng);
//!     let e = &point + &blinding;
//!     // ... some computation
//!     # e
//! }
//!
//! // You can run this function with any supported curve:
//! use generic_ec::curves::{Secp256k1, Secp256r1};
//! # let mut rng = rand::rngs::OsRng;
//!
//! let point1 = Point::<Secp256k1>::generator().to_point();
//! let _ = some_generic_computation(&mut rng, point1);
//!
//! let point2 = Point::<Secp256r1>::generator().to_point();
//! let _ = some_generic_computation(&mut rng, point2);
//!
//! // ...
//! ```
//!
//! [examples]: #examples
//! [supported curves]: #supported-curves
//!
//! ## Join us in Discord!
//! Feel free to reach out to us [in Discord]!
//!
//! [in Discord]: https://discord.com/invite/hyperledger
//!
//! ## License
//!
//! The crate is licensed under MIT or Apache-2.0 at your choice.

#![forbid(missing_docs)]
#![cfg_attr(not(test), forbid(unused_crate_dependencies))]
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub use generic_ec_core as core;

mod arithmetic;
pub mod as_raw;
pub mod coords;
mod encoded;
pub mod errors;
mod generator;
pub mod multiscalar;
mod non_zero;
mod point;
mod scalar;
mod secret_scalar;

mod _unused_deps {
    // This dependency is not used directly without `alloc` feature. Note that
    // even if `alloc` feature is off, this crate is still present in the
    // dependency tree as `curve-ed25519` feature is enabled, it's just not
    // used directly
    #[cfg(all(feature = "curve-ed25519", not(feature = "alloc")))]
    use curve25519 as _;
}

/// Common traits for points and scalars
pub mod traits {
    #[doc(inline)]
    pub use crate::core::{NoInvalidPoints, One, Reduce, Zero};

    /// Trait that allows you to check whether value is zero
    pub trait IsZero {
        /// Checks whether `self` is zero
        fn is_zero(&self) -> bool;
    }

    /// Uniformly samples an instance of `Self` from source of randomness
    ///
    /// This trait is implemented for scalars in all their variations: `Scalar<E>`,
    /// `SecretScalar<E>`, `NonZero<Scalar<E>>`, etc.
    ///
    /// Under the hood, it uses `NonZero::<Scalar<E>>::{random, random_vartime}`
    /// methods.
    pub trait Samplable {
        /// Uniformly samples an instance of `Self` from source of randomness
        /// using constant-time method
        ///
        /// Under the hood, it uses [`NonZero::<Scalar<E>>::random()`](
        /// crate::NonZero::<Scalar<E>>::random()) method,
        /// therefore it shares the same guarantees and performance drawbacks.
        /// Refer to its documentation to learn more.
        fn random<R: rand_core::RngCore>(rng: &mut R) -> Self;

        /// Uniformly samples an instance of `Self` from source of randomness
        /// using vartime method
        ///
        /// Under the hood, it uses [`NonZero::<Scalar<E>>::random_vartime()`](
        /// crate::NonZero::<Scalar<E>>::random_vartime()) method,
        /// therefore it shares the same guarantees and performance drawbacks.
        /// Refer to its documentation to learn more.
        fn random_vartime<R: rand_core::RngCore>(rng: &mut R) -> Self;
    }
}

pub mod serde;

pub use self::{
    core::Curve,
    encoded::{EncodedPoint, EncodedScalar},
    generator::Generator,
    non_zero::definition::NonZero,
    point::definition::Point,
    scalar::{Radix16Iter, Scalar},
    secret_scalar::definition::SecretScalar,
};

/// Curves supported out of the box
pub mod curves {
    #[cfg(feature = "curve-ed25519")]
    #[cfg_attr(docsrs, doc(cfg(feature = "curve-ed25519")))]
    pub use generic_ec_curves::Ed25519;
    #[cfg(feature = "curve-ed448")]
    #[cfg_attr(docsrs, doc(cfg(feature = "curve-ed448")))]
    pub use generic_ec_curves::Ed448;
    #[cfg(feature = "curve-secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "curve-secp256k1")))]
    pub use generic_ec_curves::Secp256k1;
    #[cfg(feature = "curve-secp256r1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "curve-secp256r1")))]
    pub use generic_ec_curves::Secp256r1;
    #[cfg(feature = "curve-secp384r1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "curve-secp384r1")))]
    pub use generic_ec_curves::Secp384r1;
    #[cfg(feature = "curve-stark")]
    #[cfg_attr(docsrs, doc(cfg(feature = "curve-stark")))]
    pub use generic_ec_curves::Stark;

    macro_rules! create_aliases {
        ($(#[$attr:meta] $mod:ident: $curve:ident),+$(,)?) => {$(
            /// Aliases for [`
            #[doc = stringify!($curve)]
            /// `] curve
            ///
            /// This module provides type aliases to [`Point`](crate::Point), [`Scalar`](crate::Scalar), and other types
            /// instantiated with [`
            #[doc = stringify!($curve)]
            /// `]. It might be convenient to use this module when you don't need your code to be generic over choice
            /// of curve.
            ///
            /// ## Example
            /// The code below only works with [`
            #[doc = stringify!($curve)]
            /// `] curve. By using type aliases from
            #[doc = concat!("[`generic_ec::curves::", stringify!($mod), "`](", stringify!($mod), ")")]
            /// , we never need to deal with generic parameters.
            ///
            /// ```rust
            #[doc = concat!("use generic_ec::curves::", stringify!($mod), "::{Point, SecretScalar};")]
            ///
            /// let mut rng = rand::rngs::OsRng;
            /// let secret_key = SecretScalar::random(&mut rng);
            /// let public_key = Point::generator() * &secret_key;
            /// // ...
            /// ```
            #[$attr]
            pub mod $mod {
                /// Alias for
                #[doc = concat!("[", stringify!($curve), "](super::", stringify!($curve), ")")]
                /// curve
                pub type E = super::$curve;
                /// Point on [`
                #[doc = stringify!($curve)]
                /// `](E) curve
                pub type Point = crate::Point<super::$curve>;
                /// Scalar in [`
                #[doc = stringify!($curve)]
                /// `](E) curve large prime subgroup
                pub type Scalar = crate::Scalar<super::$curve>;
                /// Secret scalar in [`
                #[doc = stringify!($curve)]
                /// `](E) curve large prime subgroup
                pub type SecretScalar = crate::SecretScalar<super::$curve>;
                /// Point on [`
                #[doc = stringify!($curve)]
                /// `](E) curve encoded as bytes
                pub type EncodedPoint = crate::EncodedPoint<super::$curve>;
                /// Scalar in [`
                #[doc = stringify!($curve)]
                /// `](E) curve large prime subgroup encoded as bytes
                pub type EncodedScalar = crate::EncodedScalar<super::$curve>;
                /// Iterator over scalar coefficients in radix 16 representation of [`
                #[doc = stringify!($curve)]
                /// `](E) curve
                pub type Radix16Iter = crate::Radix16Iter<super::$curve>;
                /// Generator of [`
                #[doc = stringify!($curve)]
                /// `](E) curve
                pub type Generator = crate::Generator<super::$curve>;
            }
        )+};
    }

    create_aliases! {
        #[cfg(feature = "curve-secp256k1")]
        secp256k1: Secp256k1,
        #[cfg(feature = "curve-secp256r1")]
        secp256r1: Secp256r1,
        #[cfg(feature = "curve-secp384r1")]
        secp384r1: Secp384r1,
        #[cfg(feature = "curve-stark")]
        stark: Stark,
        #[cfg(feature = "curve-ed25519")]
        ed25519: Ed25519,
        #[cfg(feature = "curve-ed448")]
        ed448: Ed448,
    }
}
