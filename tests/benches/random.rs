use generic_ec::curves;

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, random);

fn random(c: &mut criterion::Criterion) {
    let mut rng = rand_dev::DevRng::new();

    random_for_curve::<curves::Secp256k1>(c, &mut rng, "secp256k1");
    random_for_curve::<curves::Secp256r1>(c, &mut rng, "secp256r1");
    random_for_curve::<curves::Secp384r1>(c, &mut rng, "secp384r1");
    random_for_curve::<curves::Stark>(c, &mut rng, "stark");
    random_for_curve::<curves::Ed25519>(c, &mut rng, "ed25519");
    random_for_curve::<curves::Curve448>(c, &mut rng, "curve448");
}

fn random_for_curve<E: generic_ec::Curve>(
    c: &mut criterion::Criterion,
    rng: &mut impl rand::RngCore,
    curve_name: &str,
) {
    let mut g = c.benchmark_group(format!("random/{curve_name}"));
    g.bench_function("const-time", |b| {
        b.iter(|| generic_ec::Scalar::<E>::random(rng))
    });
    g.bench_function("vartime", |b| {
        b.iter(|| generic_ec::Scalar::<E>::random_vartime(rng))
    });
}
