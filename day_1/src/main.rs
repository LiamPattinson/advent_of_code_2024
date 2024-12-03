use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{command, Parser};
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    path: PathBuf,
}

#[derive(serde::Deserialize, Debug)]
struct Record {
    left: i32,
    right: i32,
}

fn read_csv(path: &Path) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    Ok(csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?
        .deserialize()
        .map(|line| line as Result<Record, _>)
        .collect::<Result<Vec<Record>, _>>()?
        .into_iter()
        .map(|record| (record.left, record.right))
        .multiunzip())
}

fn difference(left: &[i32], right: &[i32]) -> i32 {
    left.iter().zip(right).map(|(x, y)| (x - y).abs()).sum()
}

fn similarity(left: &[i32], right: &[i32]) -> i32 {
    let right_frequency = right.iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|x| *x += 1).or_insert(1);
        map
    });
    left.iter()
        .map(|x| x * right_frequency.get(x).unwrap_or(&0))
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let (mut left, mut right) = read_csv(args.path.as_path())?;
    left.sort();
    right.sort();
    println!("Difference: {}", difference(&left, &right));
    println!("Similarity: {}", similarity(&left, &right));
    Ok(())
}
