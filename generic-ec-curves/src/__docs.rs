//! Documentation-only module

/// Curves performance
///
/// Below you can find benchmark results for all curves provided in `generic-ec`.
///
/// Capital letter stand for a point, and small letter stand for a scalar. E.g.
/// `A+B` is point addition, `a+b` is scalar addition, and `[k]P` is multiplication
/// of scalar at point.
///
#[doc = include_str!("../perf/curves/table.md")]
///
/// ## Reproducibility
/// Benchmarks were carried out on c5.2xlarge EC2 instance.
///
/// In order to reproduce the experiments, you need [`cargo-criterion`](https://crates.io/crates/cargo-criterion)
/// to be installed. Then you can run benchmarks via:
///
/// ```text
/// cargo criterion --message-format json --bench curves > ./perf/curves/results.json
/// ```
///
/// Then you can rebuild the table above:
/// ```text
/// cargo run --bin analyze -- curves < perf/curves/results.json > perf/curves/table.md
/// ```
pub mod curves_perf {}
