use indicatif::{ProgressBar, ProgressStyle};
use std::io::Result;
use std::iter::repeat_with;
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
fn main() -> Result<()> {
    std::fs::create_dir_all("output").expect("Unable to create output directory");

    let time_per_n = Duration::from_secs(3);
    let t0 = Instant::now();
    let max_time = time_per_n * 25;

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

    for n in 1..=25 {
        let num_values = n * 10;

        let num_values_arg = format!("-DNUM_VALUES={num_values}");
        let mut test_values_arg = String::from("-DTEST_VALUES=");

        for i in 0..num_values {
            test_values_arg += format!("E{i},").as_str();
        }

        let func = || {
            Command::new("g++")
                .args([
                    "-Inav/include",
                    "-std=c++20",
                    "input/test.cpp",
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
            let portion = t0.elapsed().as_secs_f64() / max_time.as_secs_f64();
            bar.set_position((portion * 1000.0) as u64);
            bar.set_message(format!(
                "Timing enums with {num_values} values (most recent time: {time:?})"
            ));
        }
    }

    bar.finish();
    println!();
    Ok(())
}
