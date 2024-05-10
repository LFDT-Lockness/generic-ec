use std::iter;

use generic_ec::{Curve, Scalar};
use rand::Rng;

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, bench_zkp);

fn bench_zkp(c: &mut criterion::Criterion) {
    let mut rng = rand_dev::DevRng::new();
    bench_zkp_on_curve::<generic_ec::curves::Secp256k1>(c, &mut rng, "secp256k1");
    bench_zkp_on_curve::<generic_ec::curves::Ed25519>(c, &mut rng, "ed25519");
}

fn bench_zkp_on_curve<E: Curve>(
    c: &mut criterion::Criterion,
    rng: &mut rand_dev::DevRng,
    curve_name: &str,
) {
    for size in [3, 5, 10, 15] {
        c.bench_function(&format!("{curve_name}/lagrange_coefficient/{size}"), |b| {
            b.iter_batched(
                || {
                    let j = rng.gen_range(0..size);
                    let xs = iter::repeat_with(|| Scalar::<E>::random(rng))
                        .take(size)
                        .collect::<Vec<_>>();
                    (j, xs)
                },
                |(j, xs)| {
                    generic_ec_zkp::polynomial::lagrange_coefficient(Scalar::zero(), j, &xs)
                        .unwrap()
                },
                criterion::BatchSize::SmallInput,
            )
        });
        c.bench_function(
            &format!("{curve_name}/lagrange_coefficient_at_zero/{size}"),
            |b| {
                b.iter_batched(
                    || {
                        let j = rng.gen_range(0..size);
                        let xs = iter::repeat_with(|| Scalar::<E>::random(rng))
                            .take(size)
                            .collect::<Vec<_>>();
                        (j, xs)
                    },
                    |(j, xs)| {
                        generic_ec_zkp::polynomial::lagrange_coefficient_at_zero(j, &xs).unwrap()
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
}
