use clap::{command, Parser};
use itertools::Itertools;
use num::integer::gcd;
use rayon::prelude::*;
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

    fn cols(&self) -> usize {
        self.cols
    }

    fn rows(&self) -> usize {
        self.grid.len() / self.cols()
    }

    fn coord(&self, pos: usize) -> (isize, isize) {
        let (row, col) = (pos / self.cols(), pos % self.cols());
        (row as isize, col as isize)
    }

    fn pos(&self, row: isize, col: isize) -> Option<usize> {
        if row >= 0 && row < (self.rows() as isize) && col >= 0 && col < (self.cols() as isize) {
            let result = row * (self.cols() as isize) + col;
            Some(result as usize)
        } else {
            None
        }
    }

    fn antinode(&self, pos_1: usize, pos_2: usize) -> Option<usize> {
        let (row_1, col_1) = self.coord(pos_1);
        let (row_2, col_2) = self.coord(pos_2);
        self.pos(2 * row_2 - row_1, 2 * col_2 - col_1)
    }

    fn nearest_harmonic(&self, pos_1: usize, pos_2: usize) -> (isize, isize) {
        let (row_1, col_1) = self.coord(pos_1);
        let (row_2, col_2) = self.coord(pos_2);
        let distance = (row_2 - row_1, col_2 - col_1);
        let divisor = gcd(distance.0, distance.1);
        (distance.0 / divisor, distance.1 / divisor)
    }

    fn antinodes(&self, pos_1: usize, pos_2: usize) -> Vec<usize> {
        // Accounts for 'resonant harmonics'
        let (row_1, col_1) = self.coord(pos_1);
        let nearest = self.nearest_harmonic(pos_1, pos_2);
        (1..)
            .map_while(|harmonic| {
                self.pos(row_1 + harmonic * nearest.0, col_1 + harmonic * nearest.1)
            })
            .collect()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let map = Map::from_path(args.input.as_path())?;
    let char_set = HashSet::<u8>::from_iter(map.grid.iter().copied());
    let antinodes = char_set
        .par_iter()
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
                .filter_map(|perms| map.antinode(*perms[0], *perms[1]))
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();

    // There's probably a clever way of avoiding all the repeat code here...
    // It's the same as before but with flat_map and antinodes at the end
    let resonant_antinodes = char_set
        .par_iter()
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
                .flat_map(|perms| map.antinodes(*perms[0], *perms[1]))
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();

    println!("Antinodes: {}", antinodes.len());
    println!("Resonant antinodes: {}", resonant_antinodes.len());
    Ok(())
}
