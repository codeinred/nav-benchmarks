use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Result;
use std::iter::repeat_with;
use std::ops::Range;
use std::process::Command;
use std::time::{Duration, Instant};

trait Foldable<T, F> {
    fn fold(self, initial: T, func: F) -> T;
}

impl<T, F, Iterable> Foldable<T, F> for Iterable
where
    Iterable: IntoIterator,
    F: FnMut(T, Iterable::Item) -> T,
{
    fn fold(self, initial: T, func: F) -> T {
        let mut func = func;
        let mut result = initial;
        for item in self {
            result = func(result, item);
        }
        result
    }
}

fn repeat_time<Result>(
    mut func: impl FnMut() -> Result,
) -> impl Iterator<Item = (Duration, Result)> {
    repeat_with(move || {
        let t0 = Instant::now();
        let result = func();
        (t0.elapsed(), result)
    })
}

fn take_for_t<Item>(
    time: Duration,
    values: impl Iterator<Item = Item>,
) -> impl Iterator<Item = Item> {
    let t0 = Instant::now();

    values.take_while(move |_| t0.elapsed() < time)
}

fn slices<'a>(rep: i32, s: &'a str) -> Vec<&'a str> {
    let mut strings = Vec::new();

    for i in s.char_indices() {
        for _ in 0..rep {
            let (i, _) = i;
            strings.push(&s[0..i])
        }
    }
    for _ in 0..rep {
        strings.push(s);
    }

    strings
}

fn get_times(mut writer: Writer<File>) -> Result<Vec<(u32, Duration)>> {
    std::fs::create_dir_all("output").expect("Unable to create output directory");

    let time_per_n = Duration::from_secs(10);
    let t0 = Instant::now();
    let samples = 256;
    let multiplier = 1;

    let max_time = time_per_n * samples;

    let tick_strings = slices(2, "UwUwUwUwUwUwU 🐈 ");
    let bar = ProgressBar::new(1000);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(concat!(
                "Computing benchmarks... {spinner}\n",
                "{msg}\n",
                "Elapsed / Remaining: {elapsed_precise} / {eta_precise}\n",
                "{wide_bar}"
            ))
            .tick_strings(&tick_strings),
    );

    let mut data = Vec::new();

    for n in 1..=samples {
        let num_values = n * multiplier;

        let num_values_arg = format!("-DNUM_VALUES={num_values}");
        let mut test_values_arg = String::from("-DTEST_VALUES=");

        test_values_arg += "E0";
        for i in 1..num_values {
            test_values_arg += format!(", E{i} = {i}").as_str();
        }

        let func = || {
            Command::new("g++")
                .args([
                    "-Inav/include",
                    "-std=c++20",
                    "input/test_plain_enum.cpp",
                    "-o",
                    "output/test",
                    test_values_arg.as_str(),
                    num_values_arg.as_str(),
                ])
                .status()
                .expect("Failed to execute command")
        };
        for (time, status) in take_for_t(time_per_n, repeat_time(func)) {
            if !status.success() {
                println!("Error encountered. Exiting.");
                break;
            }
            data.push((num_values, time));
            writer.serialize((num_values, time.as_secs_f64()))?;
            writer.flush()?;
            let portion = t0.elapsed().as_secs_f64() / max_time.as_secs_f64();
            bar.set_position((portion * 1000.0) as u64);
            bar.set_message(format!(
                "Timing enums with {num_values} values (most recent time: {time:?})"
            ));
        }
    }

    bar.finish();

    Ok(data)
}

fn get_bounds(data: impl Iterator<Item = (f64, f64)>) -> (Range<f64>, Range<f64>) {
    let ((x_min, x_max), (y_min, y_max)) = data.fold(
        (
            (f64::INFINITY, f64::NEG_INFINITY),
            (f64::INFINITY, f64::NEG_INFINITY),
        ),
        |((x_min, x_max), (y_min, y_max)), (x, y)| {
            ((x_min.min(x), x_max.max(x)), (y_min.min(y), y_max.max(y)))
        },
    );
    (x_min..x_max, y_min..y_max)
}
use plotters::prelude::*;
fn draw_data(data: Vec<(f64, f64)>) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("output.svg", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let (x_bounds, y_bounds) = get_bounds(data.iter().copied());
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_bounds, y_bounds)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(
            data.iter()
                .map(|(x, y)| Circle::new((*x, *y), 3, RED.filled())),
        )?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!();
    let data = get_times( Writer::from_path("bench-data.csv")?)?;
    let data: Vec<(f64, f64)> = data
        .iter()
        .map(|(x, y)| (*x as f64, y.as_secs_f64()))
        .collect();

    draw_data(data)?;
    Ok(())
}
