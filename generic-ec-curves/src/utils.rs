use generic_ec_core::Reduce;

/// Interprets `bytes` as little-endian encoding of an integer, takes it modulo curve (prime)
/// order and returns scalar `S`
///
/// Works with scalars for which both [`Reduce<32>`][Reduce] and [`Reduce<64>`][Reduce] are
/// defined.
///
/// Takes:
/// * Little-endian `bytes` representation of the integer
/// * Scalar `one = 1`
pub fn scalar_from_le_bytes_mod_order_reducing_32_64<S>(bytes: &[u8], one: &S) -> S
where
    S: Default + Copy,
    S: Reduce<32> + Reduce<64>,
    S: generic_ec_core::Additive + generic_ec_core::Multiplicative<S, Output = S>,
{
    let len = bytes.len();
    match len {
        ..=31 => {
            let mut padded = [0u8; 32];
            padded[..len].copy_from_slice(bytes);
            S::from_le_array_mod_order(&padded)
        }
        32 => {
            #[allow(clippy::expect_used)]
            let bytes: &[u8; 32] = bytes.try_into().expect("we checked that bytes len == 32");
            S::from_le_array_mod_order(bytes)
        }
        33..=63 => {
            let mut padded = [0u8; 64];
            padded[..len].copy_from_slice(bytes);
            S::from_le_array_mod_order(&padded)
        }
        64 => {
            #[allow(clippy::expect_used)]
            let bytes: &[u8; 64] = bytes.try_into().expect("we checked that bytes len == 64");
            S::from_le_array_mod_order(bytes)
        }
        65.. => {
            let two_to_512 = S::add(&S::from_le_array_mod_order(&[0xff; 64]), one);

            let chunks = bytes.chunks_exact(64);
            let remainder = if !chunks.remainder().is_empty() {
                Some(scalar_from_le_bytes_mod_order_reducing_32_64::<S>(
                    chunks.remainder(),
                    one,
                ))
            } else {
                None
            };

            let chunks = chunks.rev().map(|chunk| {
                #[allow(clippy::expect_used)]
                let chunk: &[u8; 64] = chunk.try_into().expect("wrong chunk size");
                S::from_le_array_mod_order(chunk)
            });

            remainder
                .into_iter()
                .chain(chunks)
                .reduce(|acc, int| S::add(&S::mul(&acc, &two_to_512), &int))
                .unwrap_or_default()
        }
    }
}

/// Interprets `bytes` as big-endian encoding of an integer, takes it modulo curve (prime)
/// order and returns scalar `S`
///
/// Works with scalars for which both [`Reduce<32>`][Reduce] and [`Reduce<64>`][Reduce] are
/// defined.
///
/// Takes:
/// * Big-endian `bytes` representation of the integer
/// * Scalar `one = 1`
pub fn scalar_from_be_bytes_mod_order_reducing_32_64<S>(bytes: &[u8], one: &S) -> S
where
    S: Default + Copy,
    S: Reduce<32> + Reduce<64>,
    S: generic_ec_core::Additive + generic_ec_core::Multiplicative<S, Output = S>,
{
    let len = bytes.len();
    match len {
        ..=31 => {
            let mut padded = [0u8; 32];
            padded[32 - len..].copy_from_slice(bytes);
            S::from_be_array_mod_order(&padded)
        }
        32 => {
            #[allow(clippy::expect_used)]
            let bytes: &[u8; 32] = bytes.try_into().expect("we checked that bytes len == 32");
            S::from_be_array_mod_order(bytes)
        }
        33..=63 => {
            let mut padded = [0u8; 64];
            padded[64 - len..].copy_from_slice(bytes);
            S::from_be_array_mod_order(&padded)
        }
        64 => {
            #[allow(clippy::expect_used)]
            let bytes: &[u8; 64] = bytes.try_into().expect("we checked that bytes len == 64");
            S::from_be_array_mod_order(bytes)
        }
        65.. => {
            let two_to_512 = S::add(&S::from_be_array_mod_order(&[0xff; 64]), one);

            let chunks = bytes.rchunks_exact(64);
            let remainder = if !chunks.remainder().is_empty() {
                Some(scalar_from_be_bytes_mod_order_reducing_32_64::<S>(
                    chunks.remainder(),
                    one,
                ))
            } else {
                None
            };

            let chunks = chunks.rev().map(|chunk| {
                #[allow(clippy::expect_used)]
                let chunk: &[u8; 64] = chunk.try_into().expect("wrong chunk size");
                S::from_be_array_mod_order(chunk)
            });

            remainder
                .into_iter()
                .chain(chunks)
                .reduce(|acc, int| S::add(&S::mul(&acc, &two_to_512), &int))
                .unwrap_or_default()
        }
    }
}

/// Interprets `bytes` as little-endian encoding of an integer, takes it modulo curve (prime)
/// order and returns scalar `S`
///
/// Works with scalars for which only [`Reduce<32>`][Reduce] is defined.
///
/// Takes:
/// * Little-endian `bytes` representation of the integer
/// * Scalar `one = 1`
pub fn scalar_from_le_bytes_mod_order_reducing_32<S>(bytes: &[u8], one: &S) -> S
where
    S: Default + Copy,
    S: Reduce<32>,
    S: generic_ec_core::Additive + generic_ec_core::Multiplicative<S, Output = S>,
{
    let len = bytes.len();
    match len {
        ..=31 => {
            let mut padded = [0u8; 32];
            padded[..len].copy_from_slice(bytes);
            S::from_le_array_mod_order(&padded)
        }
        32 => {
            #[allow(clippy::expect_used)]
            let bytes: &[u8; 32] = bytes.try_into().expect("we checked that bytes len == 32");
            S::from_le_array_mod_order(bytes)
        }
        33.. => {
            let two_to_256 = S::add(&S::from_le_array_mod_order(&[0xff; 32]), one);

            let chunks = bytes.chunks_exact(32);
            let remainder = if !chunks.remainder().is_empty() {
                Some(scalar_from_le_bytes_mod_order_reducing_32::<S>(
                    chunks.remainder(),
                    one,
                ))
            } else {
                None
            };

            let chunks = chunks.rev().map(|chunk| {
                #[allow(clippy::expect_used)]
                let chunk: &[u8; 32] = chunk.try_into().expect("wrong chunk size");
                S::from_le_array_mod_order(chunk)
            });

            remainder
                .into_iter()
                .chain(chunks)
                .reduce(|acc, int| S::add(&S::mul(&acc, &two_to_256), &int))
                .unwrap_or_default()
        }
    }
}

/// Interprets `bytes` as big-endian encoding of an integer, takes it modulo curve (prime)
/// order and returns scalar `S`
///
/// Works with scalars for which only [`Reduce<32>`][Reduce] is defined.
///
/// Takes:
/// * Big-endian `bytes` representation of the integer
/// * Scalar `one = 1`
pub fn scalar_from_be_bytes_mod_order_reducing_32<S>(bytes: &[u8], one: &S) -> S
where
    S: Default + Copy,
    S: Reduce<32>,
    S: generic_ec_core::Additive + generic_ec_core::Multiplicative<S, Output = S>,
{
    let len = bytes.len();
    match len {
        ..=31 => {
            let mut padded = [0u8; 32];
            padded[32 - len..].copy_from_slice(bytes);
            S::from_be_array_mod_order(&padded)
        }
        32 => {
            #[allow(clippy::expect_used)]
            let bytes: &[u8; 32] = bytes.try_into().expect("we checked that bytes len == 32");
            S::from_be_array_mod_order(bytes)
        }
        33.. => {
            let two_to_256 = S::add(&S::from_be_array_mod_order(&[0xff; 32]), one);

            let chunks = bytes.rchunks_exact(32);
            let remainder = if !chunks.remainder().is_empty() {
                Some(scalar_from_be_bytes_mod_order_reducing_32::<S>(
                    chunks.remainder(),
                    one,
                ))
            } else {
                None
            };

            let chunks = chunks.rev().map(|chunk| {
                #[allow(clippy::expect_used)]
                let chunk: &[u8; 32] = chunk.try_into().expect("wrong chunk size");
                S::from_be_array_mod_order(chunk)
            });

            remainder
                .into_iter()
                .chain(chunks)
                .reduce(|acc, int| S::add(&S::mul(&acc, &two_to_256), &int))
                .unwrap_or_default()
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests that the algorithms that take `bytes` mod curve order work on ed25519.
    // Note, that `generic-ec-tests` has more extensive tests. A smaller test here
    // is supposed to detect an issue earlier and more precisely if it ever arises.
    #[test]
    fn works_on_ed25519() {
        let x = 0x11223344_u32;
        let expected = curve25519::Scalar::from(x);
        let one = &crate::ed25519::Scalar::ONE;

        assert_eq!(
            expected,
            super::scalar_from_be_bytes_mod_order_reducing_32_64(&x.to_be_bytes(), one).0
        );
        assert_eq!(
            expected,
            super::scalar_from_be_bytes_mod_order_reducing_32(&x.to_be_bytes(), one).0
        );

        assert_eq!(
            expected,
            super::scalar_from_le_bytes_mod_order_reducing_32_64(&x.to_le_bytes(), one).0
        );
        assert_eq!(
            expected,
            super::scalar_from_le_bytes_mod_order_reducing_32(&x.to_le_bytes(), one).0
        );
    }
}
