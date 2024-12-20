use clap::{command, Parser};
use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::path::{Path, PathBuf};

const BASE10: u32 = 10;
const PAD: u32 = u32::MAX;

// 2D distance same as 1D flattened distance

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    input: PathBuf,
}

#[derive(Debug)]
struct Map {
    grid: Vec<u32>,
    cols: usize,
}

impl Map {
    fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let s = std::fs::read_to_string(path)?;
        let unpadded = s.chars().position(|c| c == '\n').ok_or("Input corrupted")?;
        let cols = unpadded + 2;
        Ok(Self {
            cols,
            grid: std::iter::repeat(Ok(PAD))
                .take(cols)
                .chain(s.trim().lines().flat_map(|s| {
                    std::iter::once(Ok(PAD))
                        .chain(s.chars().map(|c| c.to_digit(BASE10).ok_or("Not a digit")))
                        .chain(std::iter::once(Ok(PAD)))
                }))
                .chain(std::iter::repeat(Ok(PAD)).take(cols))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .collect(),
        })
    }

    fn coord(&self, pos: usize) -> (usize, usize) {
        (pos / self.cols, pos % self.cols)
    }

    fn pos(&self, coord: (usize, usize)) -> usize {
        self.cols * coord.0 + coord.1
    }

    fn peaks(&self, coord: (usize, usize), next: u32) -> Vec<(usize, usize)> {
        let curr = self.grid[self.pos(coord)];
        if curr == PAD || curr > next || next - curr != 1 {
            return vec![];
        }
        if next == 10 {
            return vec![coord];
        }
        [
            (coord.0 - 1, coord.1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1 - 1),
            (coord.0, coord.1 + 1),
        ]
        .iter()
        .flat_map(|c| self.peaks(*c, next + 1))
        .collect()
    }

    fn trailheads(&self) -> usize {
        self.grid
            .par_iter()
            .enumerate()
            .filter_map(|(idx, &x)| {
                if x == 0 {
                    Some(
                        self.peaks(self.coord(idx), 1)
                            .into_iter()
                            .collect::<HashSet<_>>()
                            .len(),
                    )
                } else {
                    None
                }
            })
            .sum()
    }

    fn ratings(&self) -> usize {
        self.grid
            .par_iter()
            .enumerate()
            .filter_map(|(idx, &x)| {
                if x == 0 {
                    Some(self.peaks(self.coord(idx), 1).len())
                } else {
                    None
                }
            })
            .sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let map = Map::from_path(args.input.as_path())?;
    println!("Trailheads: {}", map.trailheads());
    println!("Ratings: {}", map.ratings());
    Ok(())
}
