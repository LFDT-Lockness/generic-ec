use std::{collections::BTreeSet, iter};

use anyhow::{bail, Context, Result};

use plotters::prelude::*;

fn main() -> Result<()> {
    let cmd = std::env::args().nth(1);

    let expected = "`multiscalar` or `curves`";
    match cmd.as_deref() {
        Some("multiscalar") => draw_multiscalar_perf(),
        Some("curves") => analyze_curves_perf(),
        Some(unknown) => bail!("unknown command `{unknown}`, expected {expected}"),
        None => bail!("missing command, expected {expected}"),
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct BenchmarkComplete<Id = String> {
    id: Id,
    mean: Measurement,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Measurement {
    estimate: f64,
    unit: String,
}

fn parse_completed_benchmarks<'a>(
    raw_json: impl serde_json::de::Read<'a>,
) -> Result<Vec<BenchmarkComplete>> {
    #[derive(serde::Deserialize)]
    pub struct CriterionOut {
        reason: String,
        #[serde(flatten)]
        data: serde_json::Value,
    }

    serde_json::Deserializer::new(raw_json)
        .into_iter::<CriterionOut>()
        .flat_map(|x| match x {
            Ok(out) if out.reason == "benchmark-complete" => {
                Some(serde_json::from_value::<BenchmarkComplete>(out.data))
            }
            Ok(_) => {
                // `reason` isn't what we looking for - just ignore
                None
            }
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<_, _>>()
        .context("parse raw json")
}

#[derive(Debug, Clone)]
struct MultiscalarId {
    algo: String,
    curve: String,
    n: usize,
}
impl std::str::FromStr for MultiscalarId {
    type Err = anyhow::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let regex = regex::Regex::new(r"^([^/]+?)/([^/]+?)/([^/]+?)/n(\d+?)$")
            .context("construct regex")?;
        let captures = regex.captures(id).context("id doesn't match regex")?;
        let (_, [operation, algo, curve, n]) = captures.extract();

        if operation != "multiscalar_mul" {
            bail!("unexpected operation {operation}, expected `multiscalar_mul`")
        }

        Ok(Self {
            algo: algo.to_owned(),
            curve: curve.to_owned(),
            n: n.parse().context("invalid n")?,
        })
    }
}

const PALLETE: &[RGBColor] = &[
    RED,
    full_palette::PURPLE,
    full_palette::INDIGO,
    full_palette::LIGHTBLUE,
    full_palette::TEAL,
    full_palette::LIME,
    full_palette::YELLOW,
    full_palette::ORANGE,
    full_palette::BROWN,
    full_palette::BLUEGREY,
];

fn draw_multiscalar_perf() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let stdin = serde_json::de::IoRead::new(stdin);

    let results = parse_completed_benchmarks(stdin).context("parse results")?;
    let mut results = results
        .into_iter()
        .map(|res| {
            Ok(BenchmarkComplete {
                id: res.id.parse::<MultiscalarId>()?,
                mean: res.mean,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    {
        assert!(results.iter().all(|res| res.mean.unit == "ns"));
        results.iter_mut().for_each(|res| {
            res.mean.estimate /= 1000000.;
            res.mean.unit = "ms".to_owned();
        });
    }

    let curves = results
        .iter()
        .map(|res| res.id.curve.clone())
        .collect::<BTreeSet<_>>();

    for curve in curves {
        let sub_results = results
            .iter()
            .filter(|res| res.id.curve == *curve)
            .cloned()
            .collect::<Vec<_>>();
        eprintln!("Draw plot for curve {curve}");
        draw_multiscalar_perf_for_curve(&sub_results)?
    }

    Ok(())
}

fn draw_multiscalar_perf_for_curve(results: &[BenchmarkComplete<MultiscalarId>]) -> Result<()> {
    let curve = &results[0].id.curve;
    let unit = &results[0].mean.unit;

    let algos = results
        .iter()
        .map(|res| res.id.algo.clone())
        .collect::<BTreeSet<_>>();

    assert!(results
        .iter()
        .all(|res| res.id.curve == *curve && res.mean.unit == *unit));

    let results_per_algo = algos
        .iter()
        .map(|algo| {
            results
                .iter()
                .filter(|res| res.id.algo == *algo)
                .map(|res| (res.id.n, res.mean.estimate))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let range_x = plotters::data::fitting_range(results_per_algo[1].iter().map(|res| &res.0));
    let range_y = plotters::data::fitting_range(results_per_algo[1].iter().map(|res| &res.1));
    let y_max = range_y.end;

    let out_path = format!("perf/multiscalar/{curve}.svg");
    let mut buffer = String::new();

    {
        let root = SVGBackend::with_string(&mut buffer, (640 * 2, 480 * 2)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .margin(20)
            .caption(format!("MultiscalarMul, {curve}"), ("sans-serif", 35.0))
            .build_cartesian_2d(range_x, range_y)?;

        chart
            .configure_mesh()
            .x_desc("n")
            .y_desc(format!("Mean, {unit}"))
            .axis_desc_style(("sans-serif", 25))
            .label_style(("sans-serif", 20))
            .draw()?;

        for (i, (algo, results)) in algos.iter().zip(&results_per_algo).enumerate() {
            let color = PALLETE[i];
            chart
                .draw_series(LineSeries::new(
                    results.iter().copied().filter(move |(_x, y)| *y <= y_max),
                    color,
                ))?
                .label(algo)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .label_font(("sans-serif", 20))
            .draw()?;

        root.present()?;
    }

    fmt_and_write_svg(out_path, &buffer)?;

    Ok(())
}

/// Deletes width and height attrs from svg to display images prettier in docs
fn fmt_and_write_svg(path: impl AsRef<std::path::Path>, svg: &str) -> Result<()> {
    let r = regex::Regex::new(r#"<svg width="\d+" height="\d+""#)?;
    let out = r.replace_all(svg, "<svg");
    std::fs::write(&path, out.as_ref())?;
    Ok(())
}

struct CurveId {
    curve: String,
    operation: String,
}
impl std::str::FromStr for CurveId {
    type Err = anyhow::Error;
    fn from_str(id: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let regex = regex::Regex::new(r"^([^/]+?)/(.+?)$").context("construct regex")?;
        let captures = regex.captures(id).context("id doesn't match regex")?;
        let (_, [curve, operation]) = captures.extract();

        Ok(Self {
            curve: curve.to_owned(),
            operation: operation.to_owned(),
        })
    }
}

fn analyze_curves_perf() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let stdin = serde_json::de::IoRead::new(stdin);

    let results = parse_completed_benchmarks(stdin).context("parse results")?;
    let results = results
        .into_iter()
        .map(|res| {
            Ok(BenchmarkComplete {
                id: res.id.parse::<CurveId>()?,
                mean: res.mean,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let mut grouped_results: Vec<(String, Vec<(String, Measurement)>)> = vec![];
    for res in results {
        let out = match grouped_results.iter_mut().find(|r| r.0 == res.id.operation) {
            Some(o) => o,
            None => {
                grouped_results.push((res.id.operation.clone(), vec![]));
                grouped_results.last_mut().unwrap()
            }
        };
        out.1.push((res.id.curve.clone(), res.mean.clone()))
    }

    let curves = grouped_results[0]
        .1
        .iter()
        .map(|(curve, _mean)| curve.clone())
        .collect::<Vec<_>>();

    let mut table = tabled::builder::Builder::new();
    table.push_record(iter::once("").chain(curves.iter().map(String::as_str)));

    for res in &grouped_results {
        let operation = res.0.clone().replace('[', "\\[").replace(']', "\\]");
        let mut row = vec![operation];
        row.extend(iter::repeat_with(|| String::new()).take(curves.len()));

        let (means, unit) = choose_uniform_unit(res.1.iter().map(|(_, m)| m));
        for (curve, mean) in res.1.iter().map(|(curve, _)| curve).zip(means) {
            let pos = curves.iter().position(|c| c == curve).unwrap();
            row[1 + pos] = if unit == "ns" {
                format!("{mean:.0}{unit}")
            } else {
                format!("{mean:.1}{unit}")
            };
        }

        table.push_record(row);
    }

    println!(
        "{}",
        table
            .build()
            .with(tabled::settings::style::Style::markdown())
    );

    Ok(())
}

fn choose_uniform_unit<'a>(
    measurements: impl Iterator<Item = &'a Measurement> + Clone,
) -> (Vec<f64>, &'static str) {
    assert!(measurements.clone().all(|m| m.unit == "ns"));

    #[derive(Ord, Eq, PartialEq, PartialOrd)]
    enum Unit {
        Nano,
        Micro,
        Mili,
    }
    debug_assert!(Unit::Nano < Unit::Micro);

    let suggested_units = measurements.clone().map(|m| {
        if m.estimate >= 1_000_000. {
            Unit::Mili
        } else if m.estimate >= 1000. {
            Unit::Micro
        } else {
            Unit::Nano
        }
    });
    let chosen_unit = suggested_units.min().unwrap();
    let unit_str = match &chosen_unit {
        Unit::Nano => "ns",
        Unit::Micro => "μs",
        Unit::Mili => "ms",
    };

    let measurements = measurements
        .map(|m| match chosen_unit {
            Unit::Nano => m.estimate,
            Unit::Micro => m.estimate / 1000.,
            Unit::Mili => m.estimate / 1_000_000.,
        })
        .collect();

    (measurements, unit_str)
}
