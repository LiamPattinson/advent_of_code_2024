use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{command, Parser};
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    input: PathBuf,
}

#[allow(clippy::type_complexity)]
fn read_equations(path: &Path) -> Result<Vec<(usize, Vec<usize>)>, Box<dyn Error>> {
    let s = std::fs::read_to_string(path)?;
    Ok(s.trim()
        .lines()
        .map(|line| line.split_once(':').ok_or("No divider"))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|(r, ns)| {
            let result = r.parse::<usize>();
            let numbers = ns
                .trim()
                .split(' ')
                .map(|n| n.parse::<usize>())
                .collect::<Result<Vec<_>, _>>();
            match (result, numbers) {
                (Ok(result), Ok(numbers)) => Ok((result, numbers)),
                (_, _) => Err("Input corrupted"),
            }
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn recurse(target: usize, acc: usize, numbers: &[usize]) -> Option<usize> {
    if numbers.is_empty() || acc > target {
        if target == acc {
            Some(acc)
        } else {
            None
        }
    } else {
        match (
            recurse(target, acc + numbers[0], &numbers[1..]),
            recurse(target, acc * numbers[0], &numbers[1..]),
        ) {
            (Some(x), _) | (_, Some(x)) => Some(x),
            _ => None,
        }
    }
}

fn concatenate(left: usize, right: usize) -> Result<usize, Box<dyn Error>> {
    Ok(format!("{left}{right}").parse::<usize>()?)
}

fn recurse_with_concatenation(target: usize, acc: usize, numbers: &[usize]) -> Option<usize> {
    if numbers.is_empty() || acc > target {
        if target == acc {
            Some(acc)
        } else {
            None
        }
    } else {
        // Parse error can occur if concatenated ints don't fit in usize.
        // If this happens, then this branch is definitely wrong, so just return None.
        if let Ok(concat) = concatenate(acc, numbers[0]) {
            match (
                recurse_with_concatenation(target, acc + numbers[0], &numbers[1..]),
                recurse_with_concatenation(target, acc * numbers[0], &numbers[1..]),
                recurse_with_concatenation(target, concat, &numbers[1..]),
            ) {
                (Some(x), _, _) | (_, Some(x), _) | (_, _, Some(x)) => Some(x),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let equations = read_equations(args.input.as_path())?;

    let result = equations
        .par_iter()
        .map(|(result, numbers)| recurse(*result, numbers[0], &numbers[1..]).unwrap_or(0))
        .sum::<usize>();
    let result_with_concatenation = equations
        .par_iter()
        .map(|(result, numbers)| {
            recurse_with_concatenation(*result, numbers[0], &numbers[1..]).unwrap_or(0)
        })
        .sum::<usize>();

    println!("Result: {result}");
    println!("Result with concatenation: {result_with_concatenation}");
    Ok(())
}
