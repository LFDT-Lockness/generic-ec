use generic_ec::curves;
use generic_ec_v04::curves as curves_v04;

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, random);

fn random(c: &mut criterion::Criterion) {
    let mut rng = rand_dev::DevRng::new();

    random_for_curve::<curves::Secp256k1, curves_v04::Secp256k1>(c, &mut rng, "secp256k1");
    random_for_curve::<curves::Secp256r1, curves_v04::Secp256r1>(c, &mut rng, "secp256r1");
    random_for_curve::<curves::Stark, curves_v04::Stark>(c, &mut rng, "stark");
    random_for_curve::<curves::Ed25519, curves_v04::Ed25519>(c, &mut rng, "ed25519");
}

fn random_for_curve<E: generic_ec::Curve, E04: generic_ec_v04::Curve>(
    c: &mut criterion::Criterion,
    rng: &mut impl rand::RngCore,
    curve_name: &str,
) {
    let mut g = c.benchmark_group(format!("random/{curve_name}"));
    g.bench_function("v04", |b| {
        b.iter(|| generic_ec_v04::Scalar::<E04>::random(rng))
    });
    g.bench_function("latest", |b| {
        b.iter(|| generic_ec::Scalar::<E>::random(rng))
    });
}
