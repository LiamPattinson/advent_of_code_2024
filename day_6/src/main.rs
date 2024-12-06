use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{command, Parser};
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    input: PathBuf,
}

// To do part 2, will need to move away from strings and start using arrays of u8...

fn read_map(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|s| s.to_string())
        .collect_vec())
}

fn pad(map: Vec<String>) -> Vec<String> {
    let line_len = map[0].len() + 2;
    let mut padded = vec!["@".repeat(line_len).to_string()];
    padded.extend(map.iter().map(|line| format!("@{line}@")));
    padded.push(padded[0].clone());
    padded
}

fn rotate(guard: char) -> Result<char, &'static str> {
    match guard {
        '^' => Ok('>'),
        '>' => Ok('v'),
        'v' => Ok('<'),
        '<' => Ok('^'),
        _ => Err("Snake, get out of there, they're using active camo!"),
    }
}

fn advance(map: &mut String, cols: usize) -> Result<bool, &'static str> {
    let guard_idx = map.find(['^', '>', '<', 'v']).unwrap_or(0);
    let guard = map.chars().nth(guard_idx).unwrap_or('?');
    let next_idx = match guard {
        '^' => guard_idx - cols,
        '>' => guard_idx + 1,
        'v' => guard_idx + cols,
        '<' => guard_idx - 1,
        _ => guard_idx,
    };
    let next = map.chars().nth(next_idx).unwrap_or('!');
    match (guard, next) {
        (guard, '.') | (guard, 'X') => {
            map.replace_range(guard_idx..guard_idx + 1, "X");
            map.replace_range(next_idx..next_idx + 1, guard.to_string().as_str());
            Ok(true)
        }
        (guard, '#') => {
            map.replace_range(
                guard_idx..guard_idx + 1,
                rotate(guard)?.to_string().as_str(),
            );
            Ok(true)
        }
        (_, '@') => {
            map.replace_range(guard_idx..guard_idx + 1, "X");
            Ok(false)
        }
        _ => Err("Snake, you've created a time paradox!"),
    }
}

fn solve(map: &mut String, cols: usize) -> Result<usize, Box<dyn Error>> {
    let mut on_map = true;
    let mut count = 0;
    while on_map {
        on_map = advance(map, cols)?;
        count += 1;
        if count > map.len() {
            break;
        }
    }
    Ok(map.matches('X').count())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let map = pad(read_map(args.input.as_path())?);
    let cols = map[0].len();
    let mut flattened: String = map
        .into_iter()
        .flat_map(|s| s.chars().collect_vec())
        .collect();
    println!("Unique tiles: {}", solve(&mut flattened, cols)?);
    Ok(())
}
