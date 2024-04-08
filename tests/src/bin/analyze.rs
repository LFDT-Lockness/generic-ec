use anyhow::{bail, Context, Result};

use plotters::prelude::*;

fn main() -> Result<()> {
    let arg = std::env::args().nth(1);
    match arg.as_deref() {
        Some("multiscalar-estimation") => draw_multiscalar_perf_estimation(),
        Some("multiscalar-perf") => draw_multiscalar_perf(),
        Some(arg) => {
            bail!("Unexpected argument `{arg}`. See {}", file!())
        }
        None => {
            bail!("Expected an argument. See {}", file!())
        }
    }
}

fn draw_multiscalar_perf_estimation() -> Result<()> {
    let out_path = "perf/multiscalar/estimation.svg";
    let mut buffer = String::new();

    {
        let root = SVGBackend::with_string(&mut buffer, (640 * 2, 480 * 2)).into_drawing_area();
        root.fill(&WHITE)?;

        let x_max = 150;
        let y_max = 12000;
        let mut chart = ChartBuilder::on(&root)
            .caption(
                "Multiscalar Performance Estimation",
                ("sans-serif", 35).into_font(),
            )
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(80)
            .build_cartesian_2d(0..x_max, 0..y_max)?;
        chart
            .configure_mesh()
            .x_desc("n")
            .y_desc("A + D")
            .axis_desc_style(("sans-serif", 25))
            .label_style(("sans-serif", 20))
            .draw()?;

        chart
            .draw_series(LineSeries::new(
                (2..x_max)
                    .map(|n| (n, n * (256 + 128) + n))
                    .filter(|(_x, y)| *y <= y_max),
                BLUE.stroke_width(2),
            ))?
            .label("Naive")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.stroke_width(2)));

        chart
            .draw_series(LineSeries::new(
                (2..x_max)
                    .map(|n| (n, 5 * 63 + (n + 30) * 64))
                    .filter(|(_x, y)| *y <= y_max),
                GREEN.stroke_width(2),
            ))?
            .label("Pippeger")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN.stroke_width(2)));

        chart
            .draw_series(LineSeries::new(
                (2..x_max)
                    .map(|n| (n, 5 * 63 + (n - 1) * 64 + 16 * n))
                    .filter(|(_x, y)| *y <= y_max),
                RED.stroke_width(2),
            ))?
            .label("Straus")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(2)));

        chart
            .configure_series_labels()
            .background_style(WHITE)
            .border_style(BLACK)
            .label_font(("sans-serif", 20))
            .draw()?;
        root.present()?;
    }

    fmt_and_write_svg(out_path, &buffer)?;

    Ok(())
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
pub struct MultiscalarId {
    algo: String,
    curve: String,
    n: usize,
}
impl std::str::FromStr for MultiscalarId {
    type Err = anyhow::Error;
    fn from_str(id: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut id = id.split('/');

        let operation = id.next().context("`id` doesn't have enough parts")?;
        if operation != "multiscalar_mul" {
            bail!("unexpected operation {operation}, expected `multiscalar_mul`")
        }

        let algo = id
            .next()
            .context("`id` doesn't have enough parts")?
            .to_owned();
        let curve = id
            .next()
            .context("`id` doesn't have enough parts")?
            .to_owned();

        let n = id.next().context("`id` doesn't have enough parts")?;
        if !n.starts_with('n') {
            bail!("malformed `n`")
        }
        let n = n[1..].parse().context("n is not an integer")?;

        Ok(Self { algo, curve, n })
    }
}

const PALLETE: &[RGBColor] = &[RED, BLUE, MAGENTA, CYAN];

const CURVES: &[&str] = &["secp256k1", "secp256r1", "stark", "ed25519"];
const ALGOS: &[&str] = &["naive", "straus", "pippenger"];

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

    assert!(results
        .iter()
        .all(|res| CURVES.contains(&res.id.curve.as_str())));

    for curve in CURVES {
        let sub_results = results
            .iter()
            .filter(|res| res.id.curve == *curve)
            .cloned()
            .collect::<Vec<_>>();
        draw_multiscalar_perf_for_curve(&sub_results)?
    }

    Ok(())
}

fn draw_multiscalar_perf_for_curve(results: &[BenchmarkComplete<MultiscalarId>]) -> Result<()> {
    let curve = &results[0].id.curve;
    let unit = &results[0].mean.unit;

    assert!(results.iter().all(|res| res.id.curve == *curve
        && res.mean.unit == *unit
        && ALGOS.contains(&res.id.algo.as_str())));

    let results_per_algo = ALGOS
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

        for (i, (algo, results)) in ALGOS.iter().zip(&results_per_algo).enumerate() {
            let color = PALLETE[i];
            chart
                .draw_series(LineSeries::new(
                    results.iter().copied().filter(move |(_x, y)| *y <= y_max),
                    color,
                ))?
                .label(*algo)
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
