// A very inefficent implementation here...
// Ways to speed it up:
// - Don't copy the whole map every time, just put the guard back at the start.
// - If the guard walks on a 'covered' tile facing a direction they've been in before,
//   they must be in an infinite loop.
// - You only need to check the tiles that were covered in the first part of the
//   problem.
// - No need to modify the map tiles as the guard walks in part 2.
// - Maybe just work with u8 instead of the enums.
// - Unwrap rather than passing Results everywhere.
use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{command, Parser};
use indicatif::ProgressIterator;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    input: PathBuf,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Status {
    Empty,
    Covered,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Solution {
    Finished(usize),
    InfiniteLoop,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Object {
    Space(Status),
    Guard(Direction),
    Obstruction,
    Edge,
}

impl Status {
    fn new() -> Self {
        Self::Empty
    }
}

impl Direction {
    fn new() -> Self {
        Self::Up
    }

    fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl Object {
    fn from(c: char) -> Result<Self, &'static str> {
        match c {
            '.' => Ok(Self::Space(Status::new())),
            '^' => Ok(Self::Guard(Direction::new())),
            '#' => Ok(Self::Obstruction),
            '@' => Ok(Self::Edge),
            _ => Err("Colonel... It's Metal Gear!"),
        }
    }

    fn is_guard(&self) -> bool {
        matches!(self, Object::Guard(_))
    }
}

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

fn convert(map: String) -> Result<Vec<Object>, Box<dyn Error>> {
    Ok(map
        .chars()
        .map(Object::from)
        .collect::<Result<Vec<_>, _>>()?)
}

fn advance(map: &mut [Object], cols: usize) -> Result<bool, &'static str> {
    let guard_idx = map.iter().position(|&x| x.is_guard()).unwrap_or(0);
    let guard = map[guard_idx];
    let next_idx = match guard {
        Object::Guard(Direction::Up) => Ok(guard_idx - cols),
        Object::Guard(Direction::Right) => Ok(guard_idx + 1),
        Object::Guard(Direction::Down) => Ok(guard_idx + cols),
        Object::Guard(Direction::Left) => Ok(guard_idx - 1),
        _ => Err("Snake, they're using active camo, get out of there!"),
    }?;
    let next = map[next_idx];
    match (guard, next) {
        (Object::Guard(_), Object::Space(_)) => {
            map[next_idx] = guard;
            map[guard_idx] = Object::Space(Status::Covered);
            Ok(true)
        }
        (Object::Guard(direction), Object::Obstruction) => {
            map[guard_idx] = Object::Guard(direction.rotate());
            Ok(true)
        }
        (_, Object::Edge) => {
            map[guard_idx] = Object::Space(Status::Covered);
            Ok(false)
        }
        _ => Err("Snake, you've created a time paradox!"),
    }
}

fn solve(map: &mut [Object], cols: usize) -> Result<Solution, Box<dyn Error>> {
    let mut on_map = true;
    let mut count = 0;
    while on_map {
        on_map = advance(map, cols)?;
        count += 1;
        if count > 4 * map.len() {
            // This is excessive!
            return Ok(Solution::InfiniteLoop);
        }
    }
    Ok(Solution::Finished(
        map.iter()
            .filter(|&x| *x == Object::Space(Status::Covered))
            .count(),
    ))
}

fn possible_obstructions(map: &[Object], cols: usize) -> Result<usize, Box<dyn Error>> {
    Ok(map
        .iter()
        .progress()
        .enumerate()
        .filter(|(_, x)| matches!(x, Object::Space(_)))
        .map(|(idx, _)| {
            let mut new_map = map.to_vec();
            new_map[idx] = Object::Obstruction;
            solve(&mut new_map, cols)
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter(|x| matches!(x, Solution::InfiniteLoop))
        .count())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let map = pad(read_map(args.input.as_path())?);
    let cols = map[0].len();
    let flattened: String = map
        .into_iter()
        .flat_map(|s| s.chars().collect_vec())
        .collect();
    let object_map = convert(flattened)?;
    if let Solution::Finished(unique_tiles) = solve(&mut object_map.to_vec(), cols)? {
        println!("Unique tiles: {unique_tiles}");
    }
    let infinite_loops = possible_obstructions(&object_map, cols)?;
    println!("Infinite Loops: {infinite_loops}");
    Ok(())
}
