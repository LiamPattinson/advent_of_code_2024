use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{command, Parser};
use indicatif::{ParallelProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    input: PathBuf,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Solution {
    Finite(usize),
    Infinite,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Object {
    Space,
    Obstruction,
    Edge,
    Guard,
}

impl Direction {
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
            '.' => Ok(Self::Space),
            '#' => Ok(Self::Obstruction),
            '@' => Ok(Self::Edge),
            '^' => Ok(Self::Guard),
            _ => Err("Snake, you've created a time paradox!"),
        }
    }
}

fn read_map(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(std::fs::read_to_string(path)?
        .trim()
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

fn solve(map: &[Object], cols: usize, start_pos: usize, extra_obstruction: usize) -> Solution {
    let mut guard_idx = start_pos;
    let mut dir = Direction::Up;
    let mut positions = HashMap::new();
    loop {
        if !positions
            .entry(guard_idx)
            .or_insert(BTreeSet::new())
            .insert(dir)
        {
            return Solution::Infinite;
        }

        let next_idx = match dir {
            Direction::Up => guard_idx - cols,
            Direction::Right => guard_idx + 1,
            Direction::Down => guard_idx + cols,
            Direction::Left => guard_idx - 1,
        };
        let next = if next_idx == extra_obstruction {
            Object::Obstruction
        } else {
            map[next_idx]
        };

        match (dir, next) {
            (d, Object::Obstruction) => {
                dir = d.rotate();
            }
            (_, Object::Edge) => {
                return Solution::Finite(positions.len());
            }
            _ => {
                guard_idx = next_idx;
            }
        }
    }
}

fn possible_obstructions(map: &[Object], cols: usize, start_pos: usize) -> usize {
    map.par_iter()
        .progress_with_style(ProgressStyle::default_bar())
        .enumerate()
        .filter(|(_, x)| matches!(x, Object::Space))
        .map(|(idx, _)| solve(map, cols, start_pos, idx))
        .filter(|x| matches!(x, Solution::Infinite))
        .count()
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
    if let Some(start_pos) = object_map.iter().position(|&x| matches!(x, Object::Guard)) {
        if let Solution::Finite(unique_tiles) = solve(&object_map, cols, start_pos, 0) {
            println!("Unique tiles: {unique_tiles}");
        }
        let infinite_loops = possible_obstructions(&object_map, cols, start_pos);
        println!("Infinite Loops: {infinite_loops}");
        Ok(())
    } else {
        Err("What? Where are the guards?".into())
    }
}
