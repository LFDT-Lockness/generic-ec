use generic_ec_core::*;
use rand_core::RngCore;

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, bench_curves);

fn bench_curves(c: &mut criterion::Criterion) {
    let mut rng = rand_dev::DevRng::new();

    bench_curve::<generic_ec_curves::Secp256k1>(c, &mut rng, "secp256k1");
    bench_bytes_reduction::<generic_ec_curves::Secp256k1, 32>(c, &mut rng, "secp256k1");
    bench_bytes_reduction::<generic_ec_curves::Secp256k1, 64>(c, &mut rng, "secp256k1");

    bench_curve::<generic_ec_curves::Secp256r1>(c, &mut rng, "secp256r1");
    bench_bytes_reduction::<generic_ec_curves::Secp256r1, 32>(c, &mut rng, "secp256r1");

    bench_curve::<generic_ec_curves::Stark>(c, &mut rng, "stark");
    bench_bytes_reduction::<generic_ec_curves::Stark, 32>(c, &mut rng, "stark");

    bench_curve::<generic_ec_curves::Ed25519>(c, &mut rng, "ed25519");
    bench_bytes_reduction::<generic_ec_curves::Ed25519, 32>(c, &mut rng, "ed25519");
    bench_bytes_reduction::<generic_ec_curves::Ed25519, 64>(c, &mut rng, "ed25519");
}

fn bench_curve<E: Curve>(
    c: &mut criterion::Criterion,
    rng: &mut rand_dev::DevRng,
    curve_name: &str,
) {
    let mut g = c.benchmark_group(curve_name);

    g.bench_function("A+B", |b| {
        b.iter_batched(
            || (random_point::<E>(rng), random_point::<E>(rng)),
            |(a, b)| E::Point::add(&a, &b),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("[k]P", |b| {
        b.iter_batched(
            || (E::Scalar::random(rng), random_point::<E>(rng)),
            |(k, p)| E::Scalar::mul(&k, &p),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("SmallFactorCheck", |b| {
        b.iter_batched(
            || random_point::<E>(rng),
            |p| p.is_torsion_free(),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("EncodeCompressedPoint", |b| {
        b.iter_batched(
            || random_point::<E>(rng),
            |p| p.to_bytes_compressed(),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("EncodeUncompressedPoint", |b| {
        b.iter_batched(
            || random_point::<E>(rng),
            |p| p.to_bytes_uncompressed(),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("DecodeCompressedPoint", |b| {
        b.iter_batched(
            || random_point::<E>(rng).to_bytes_compressed(),
            |bytes| E::Point::decode(bytes.as_ref()),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("DecodeUncompressedPoint", |b| {
        b.iter_batched(
            || random_point::<E>(rng).to_bytes_uncompressed(),
            |bytes| E::Point::decode(bytes.as_ref()),
            criterion::BatchSize::SmallInput,
        );
    });

    g.bench_function("a+b", |b| {
        b.iter_batched(
            || (E::Scalar::random(rng), E::Scalar::random(rng)),
            |(a, b)| E::Scalar::add(&a, &b),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("a*b", |b| {
        b.iter_batched(
            || (E::Scalar::random(rng), E::Scalar::random(rng)),
            |(a, b)| E::Scalar::mul(&a, &b),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("inv(a)", |b| {
        b.iter_batched(
            || E::Scalar::random(rng),
            |a| E::Scalar::invert(&a),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("RandomScalar", |b| {
        b.iter(|| E::Scalar::random(rng));
    });

    g.bench_function("EncodeScalarBE", |b| {
        b.iter_batched(
            || E::Scalar::random(rng),
            |a| a.to_be_bytes(),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("EncodeScalarLE", |b| {
        b.iter_batched(
            || E::Scalar::random(rng),
            |a| a.to_le_bytes(),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("DecodeScalarBE", |b| {
        b.iter_batched(
            || E::Scalar::random(rng).to_be_bytes(),
            |bytes| E::Scalar::from_be_bytes_exact(&bytes).unwrap(),
            criterion::BatchSize::SmallInput,
        );
    });
    g.bench_function("DecodeScalarLE", |b| {
        b.iter_batched(
            || E::Scalar::random(rng).to_le_bytes(),
            |bytes| E::Scalar::from_le_bytes_exact(&bytes).unwrap(),
            criterion::BatchSize::SmallInput,
        );
    });

    for size in [32, 64, 128, 512] {
        g.bench_with_input(
            criterion::BenchmarkId::new("BeBytesModOrder", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || {
                        let mut bytes = vec![0u8; size];
                        rng.fill_bytes(&mut bytes);
                        bytes
                    },
                    |bytes| E::Scalar::from_be_bytes_mod_order(&bytes),
                    criterion::BatchSize::SmallInput,
                );
            },
        );
        g.bench_with_input(
            criterion::BenchmarkId::new("LeBytesModOrder", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || {
                        let mut bytes = vec![0u8; size];
                        rng.fill_bytes(&mut bytes);
                        bytes
                    },
                    |bytes| E::Scalar::from_le_bytes_mod_order(&bytes),
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
}

fn bench_bytes_reduction<E: Curve, const N: usize>(
    c: &mut criterion::Criterion,
    rng: &mut rand_dev::DevRng,
    curve: &str,
) where
    E::Scalar: Reduce<N>,
{
    c.bench_function(&format!("{curve}/ReduceBe/{N}"), |b| {
        b.iter_batched(
            || {
                let mut bytes = [0u8; N];
                rng.fill_bytes(&mut bytes);
                bytes
            },
            |bytes| E::Scalar::from_be_array_mod_order(&bytes),
            criterion::BatchSize::SmallInput,
        );
    });
    c.bench_function(&format!("{curve}/ReduceLe/{N}"), |b| {
        b.iter_batched(
            || {
                let mut bytes = [0u8; N];
                rng.fill_bytes(&mut bytes);
                bytes
            },
            |bytes| E::Scalar::from_le_array_mod_order(&bytes),
            criterion::BatchSize::SmallInput,
        );
    });
}

fn random_point<E: Curve>(rng: &mut rand_dev::DevRng) -> E::Point {
    let scalar = E::Scalar::random(rng);
    E::Scalar::mul(&scalar, &CurveGenerator)
}
