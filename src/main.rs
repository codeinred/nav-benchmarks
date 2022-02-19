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
fn main() -> Result<()> {
    std::fs::create_dir_all("output").expect("Unable to create output directory");

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

        let timer = Instant::now();

        let mut total = Duration::ZERO;
        for (time, status) in repeat_time(func) {
            if !status.success() {
                println!("Error encountered. Exiting.");
                break;
            }

            total += time;

            println!("{num_values:>4},{time:>15?}");
            // stdout().flush()?;

            if timer.elapsed() > Duration::new(2, 0) {
                break;
            }
        }
    }

    println!();
    Ok(())
}
