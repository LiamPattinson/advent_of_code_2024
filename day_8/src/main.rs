use clap::{command, Parser};
use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::path::{Path, PathBuf};

// 2D distance same as 1D flattened distance

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    input: PathBuf,
}

struct Map {
    grid: Vec<u8>,
    cols: usize,
}

impl Map {
    fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let s = std::fs::read_to_string(path)?;
        Ok(Self {
            cols: s.chars().position(|c| c == '\n').ok_or("Input corrupted")?,
            grid: s
                .trim()
                .lines()
                .flat_map(|s| s.as_bytes())
                .copied()
                .collect(),
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let map = Map::from_path(args.input.as_path())?;
    let char_set = HashSet::<u8>::from_iter(map.grid.iter().copied());
    let antinodes = char_set
        .iter()
        .filter(|c| **c != b'.')
        .flat_map(|antenna| {
            let locations: Vec<_> = map
                .grid
                .iter()
                .enumerate()
                .filter_map(|(idx, c)| if c == antenna { Some(idx) } else { None })
                .collect();
            locations
                .iter()
                .permutations(2)
                .filter_map(|perms| {
                    // implement this in a function on Map...
                    // Part 2, work out greatest common factor of (row, col), apply in both
                    // directions. Also use combinations instead of permutations.
                    let l = *perms[0];
                    let r = *perms[1];
                    let l_row = l / map.cols;
                    let r_row = r / map.cols;
                    let l_col = l % map.cols;
                    let r_col = r % map.cols;
                    // avoid underflow
                    if 2 * r_row < l_row || 2 * r_col < l_col {
                        None
                    } else {
                        let row = 2 * r_row - l_row;
                        let col = 2 * r_col - l_col;
                        let position = row * map.cols + col;
                        if position < map.grid.len() && col < map.cols {
                            Some(position)
                        } else {
                            None
                        }
                    }
                })
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();

    println!("Antinodes: {}", antinodes.len());
    Ok(())
}
