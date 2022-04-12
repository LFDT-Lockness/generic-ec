# General elliptic curve cryptography

The library provides a set of simple abstractions boosting your experience of doing elliptic curve arithmetic
in Rust. Aim is to **stay simple**, **generic**, and **secure**. It's handy for developers who implement MPC,
zero-knowledge protocols, or any other elliptic crypto algorithms. 

## Overview

Crate provides three primitives: `Point<E>` which stands for elliptic point, `Scalar<E>` is an integer modulus 
group order, and `SecretScalar<E>` is a scalar that carries some sensitive value (e.g. secret key). `E` here
indicates the choice of elliptic curve, it could be any [supported curve][supported curves], e.g. `Point<Secp256k1>` is 
an elliptic point on secp256k1 curve.

## Arithmetic

All arithmetic operations are supported: you can add two points, multiply point by a scalar, inverse the scalar 
modulo group order, and so on. See [examples] sections.

## Security & guarantees

Library mitigates a bunch of attacks (such as small-group attack) by design by enforcing following checks:
* Scalar `Scalar<E>` must be modulo group order

  I.e. scalar is guaranteed to be in range `[0; group_order)`
* Elliptic point `Point<E>` must be on the curve

  I.e. elliptic point is guaranteed to satisfy equation of `E`
* Non-zero elliptic point `Point<E>` must have order equal to group order

  Combining to fact that scalar is guaranteed to be modulo group order, it basically means that result of
  multiplication (non-zero point × non-zero scalar) is always a valid non-zero point.

By saying "it's guaranteed" we mean that point or scalar not meeting above requirements cannot be constructed, 
as these checks are always enforced. E.g. if you're deserializing a sequence of bytes that represents an invalid 
point, deserialization will result into error.

### `SecretScalar<E>`

Sometimes your scalar represents some sensitive value like secret key, and you want to keep it safer.
`SecretScalar<E>` is in-place replacement of `Scalar<E>` just for that! It enforces additional security
by storing the scalar value on the heap, and erasing the value on drop. Its advantage is that it doesn't
leave any trace in memory dump after it's dropped (which is not guaranteed by regular `Scalar<E>`). 

But keep in mind that we can't control the OS which could potentially load RAM page containing sensitive value 
to the swap disk (i.e. on your HDD/SSD) if you're running low on memory. Or it could do any other fancy stuff.
We avoid writing unsafe or OS-specific code that could mitigate this problem.

### Points at infinity

It should be noticed that point at infinity is a valid `Point<E>`. You can construct it by calling `Point::<E>::zero()`,
e.g. `Point::<Secp256k1>::zero()` is a point at infinity for secp256k1 curve.

If the protocol you're implementing requires points to be non-zero, you need to enforce this check by calling
`.is_zero()` method. Be aware of it because sometimes missing check could lead to vulnerability.

## Supported curves

Crate provides support for following elliptic curves out of box:

| Curve      | Feature            | Backend                               |
|------------|--------------------|---------------------------------------|
| secp256k1  | `curve-secp256k1`* | [rust-bitcoin/rust-secp256k1]         |
| secp256r1  | `curve-secp256r1`  | [RustCrypto/p256]                     |
| Curve25519 | `curve-25519`      | [dalek-cryptography/curve25519-dalek] |
| Ristretto  | `curve-ristretto`  | [dalek-cryptography/curve25519-dalek] |

\* enabled by default

[rust-bitcoin/rust-secp256k1]: https://github.com/rust-bitcoin/rust-secp256k1/
[RustCrypto/p256]: https://github.com/RustCrypto/elliptic-curves/tree/master/p256
[dalek-cryptography/curve25519-dalek]: https://github.com/dalek-cryptography/curve25519-dalek

In order to use one of the supported curves, you need to turn on corresponding feature. E.g. if you want
to use Ristretto curve, add this to Cargo.toml:

```toml
[dependency]
generic-ec = { version = "...", features = ["curve-ristretto"] }
```

And now you can generate a point on that curve:

```rust
use generic_ec::{Point, Scalar, curves::Ristretto};

let random_point: Point<Ristretto> = Point::generator() * Scalar::random();
```

### Adding support for other curves

Adding new curve is as easy as implementing [`Curve` trait]! If you're missing some curve support, or you're
not fine with using existing implementation, you may define your implementation of `Curve` trait and enjoy 
using the same handy primitives `Point<YOUR_EC>`, `Scalar<YOUR_EC>`, and etc.

## Features

* `curve-{name}` enables specified curve support. See list of [supported curves].
* `serde` enables points/scalar (de)serialization support.

## Examples

### Random scalar / point generation

```rust
use generic_ec::{Point, Scalar, curves::Secp256k1};

// Generates random non-zero scalar
let random_scalar = Scalar::<Secp256k1>::random();
// Produces a point that's result of generator multiplied at the random scalar
let point = Point::generator() * &random_scalar;
```

### Diffie-Hellman key exchange

```rust
use generic_ec::{Point, SecretScalar, curves::Secp256k1};

let alice_sk = SecretScalar::<Secp256k1>::random();
let alice_pk = Point::generator() * &alice_sk;

let bob_sk = SecretScalar::<Secp256k1>::random();
let bob_pk = Point::generator() * &bob_sk;

let shared_secret_learned_by_alice = bob_pk * &alice_sk;
let shared_secret_learned_by_bob = alice_pk * &bob_sk;
assert_eq!(shared_secret_learned_by_alice, shared_secret_learned_by_bob);
```

### Generic over choice of curve

You can simply make your function generic over choice of curve:

```rust
use generic_ec::{Point, Scalar, Curve};

fn some_generic_computation<E: Curve>(point: Point<E>) -> Point<E> {
    let blinding = Point::<E>::generator() * Scalar::random();
    let e = &point + &blinding;
    // ... some computation
}

// You can run this function with any supported curve:
use generic_ec::curves::{Secp256k1, Curve25519};

let point1 = Point::<Secp256k1>::generator();
let _ = some_generic_computation(point1);

let point2 = Point::<Curve25519>::generator();
let _ = some_generic_computation(point2);

// ...
```

[examples]: #examples
[supported curves]: #supported-curves
[`Curve` trait]: 123

## Similar crates

* [ZenGo-X/curv](https://github.com/ZenGo-X/curv) crate, provides similar tools for general elliptic cryptography, plus big number arithmetic, and a bunch 
  of implemented zero-knowledge proofs.

## License 

The crate is licensed under [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at your choice.
