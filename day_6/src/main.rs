use std::error::Error;
use std::path::{Path, PathBuf};

use bitflags::bitflags;
use clap::{command, Parser};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    input: PathBuf,
}

bitflags! {
    #[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
    pub struct Direction: u8 {
        const Up = 0b0000_0001;
        const Down = 0b0000_0010;
        const Left = 0b0000_0100;
        const Right = 0b0000_1000;
        const _ = !0;
    }
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
        match *self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            _ => Self::default(), // Cheating here, should really return Result
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
    let mut positions = vec![Direction::default(); map.len()];
    loop {
        if positions[guard_idx].intersects(dir) {
            return Solution::Infinite;
        }
        positions[guard_idx].insert(dir);

        let next_idx = match dir {
            Direction::Up => guard_idx - cols,
            Direction::Right => guard_idx + 1,
            Direction::Down => guard_idx + cols,
            Direction::Left => guard_idx - 1,
            _ => 0, // Cheating here, should return result
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
                if extra_obstruction > 0 {
                    return Solution::Finite(0);
                } else {
                    return Solution::Finite(
                        positions
                            .iter()
                            .filter(|x| **x != Direction::default())
                            .count(),
                    );
                }
            }
            _ => {
                guard_idx = next_idx;
            }
        }
    }
}

fn possible_obstructions(map: &[Object], cols: usize, start_pos: usize) -> usize {
    map.par_iter()
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
