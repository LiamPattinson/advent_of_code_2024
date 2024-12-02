use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{command, Parser};
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    path: PathBuf,
}

fn read_report(path: &Path) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    Ok(std::fs::read_to_string(path)?
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn is_strictly_safe(levels: &[i32]) -> bool {
    let diff: Vec<_> = levels
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(x, y)| y - x)
        .collect();
    let sign = diff
        .iter()
        .map(|x| x.signum())
        .tuple_windows::<(_, _)>()
        .all(|(x, y)| x == y);
    let magnitude = diff.iter().all(|x| matches!(x.abs(), 1..=3));
    sign && magnitude
}

fn is_partially_safe(levels: &[i32]) -> bool {
    levels
        .iter()
        .combinations(levels.len() - 1)
        .map(|x| x.into_iter().copied().collect::<Vec<_>>()) // Vec<&i32> to Vec<i32>
        .any(|x| is_strictly_safe(&x))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let report = read_report(args.path.as_path())?;
    println!(
        "Strictly safe: {}",
        report.iter().filter(|x| is_strictly_safe(x)).count()
    );
    println!(
        "Partially safe: {}",
        report.iter().filter(|x| is_partially_safe(x)).count()
    );
    Ok(())
}
