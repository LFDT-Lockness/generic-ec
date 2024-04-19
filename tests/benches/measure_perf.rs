use core::iter;

use generic_ec::{
    curves,
    multiscalar::{self, MultiscalarMul},
    Curve, Point, Scalar,
};
use rand::{CryptoRng, RngCore};

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, multiscalar);

fn multiscalar(c: &mut criterion::Criterion) {
    let mut rng = rand_dev::DevRng::new();

    multiscalar_for_curve::<curves::Secp256k1>(c, &mut rng, "secp256k1");
    multiscalar_for_curve::<curves::Secp256r1>(c, &mut rng, "secp256r1");
    multiscalar_for_curve::<curves::Stark>(c, &mut rng, "stark");
    multiscalar_for_curve::<curves::Ed25519>(c, &mut rng, "ed25519");

    multiscalar_for_curve_and_algo::<curves::Ed25519, multiscalar::Dalek>(
        c, &mut rng, "ed25519", "dalek",
    );
}

fn multiscalar_for_curve<E: Curve>(
    c: &mut criterion::Criterion,
    rng: &mut (impl RngCore + CryptoRng),
    curve_name: &str,
) where
    multiscalar::Naive: MultiscalarMul<E>,
    multiscalar::Straus: MultiscalarMul<E>,
    multiscalar::Pippenger: MultiscalarMul<E>,
{
    multiscalar_for_curve_and_algo::<E, multiscalar::Naive>(c, rng, curve_name, "naive");
    multiscalar_for_curve_and_algo::<E, multiscalar::Straus>(c, rng, curve_name, "straus");
    multiscalar_for_curve_and_algo::<E, multiscalar::Pippenger>(c, rng, curve_name, "pippenger");
}

fn multiscalar_for_curve_and_algo<E: Curve, M: MultiscalarMul<E>>(
    c: &mut criterion::Criterion,
    rng: &mut (impl RngCore + CryptoRng),
    curve_name: &str,
    multiscalar_algo: &str,
) {
    for n in iter::once(2).chain(10..=250) {
        c.bench_function(
            &format!("multiscalar_mul/{multiscalar_algo}/{curve_name}/n{n}"),
            |b| {
                b.iter_batched(
                    || {
                        iter::repeat_with(|| {
                            (
                                Scalar::<E>::random(rng),
                                Point::generator() * Scalar::<E>::random(rng),
                            )
                        })
                        .take(n)
                        .collect::<Vec<_>>()
                    },
                    |scalar_points| M::multiscalar_mul(scalar_points),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
}
