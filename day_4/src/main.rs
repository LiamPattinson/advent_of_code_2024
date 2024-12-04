// There must be a cleverer way to do this...
use std::error::Error;
use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    path: PathBuf,
}

fn is_xmas(chars: &[char]) -> bool {
    matches!(String::from_iter(chars).as_str(), "XMAS" | "SAMX")
}

fn horizontal_matches(puzzle: &str, rows: usize, cols: usize) -> usize {
    // This is much easier to do by just scanning lines one-by-one and using
    // `string.matches()`. It's solved this way for consistency with the other methods.
    (0..rows)
        .map(|row| {
            puzzle[row * cols..(row + 1) * cols - 3]
                .chars()
                .zip(puzzle[row * cols + 1..(row + 1) * cols - 2].chars())
                .zip(puzzle[row * cols + 2..(row + 1) * cols - 1].chars())
                .zip(puzzle[row * cols + 3..(row + 1) * cols].chars())
                .map(|(((a, b), c), d)| is_xmas(&[a, b, c, d]) as usize)
                .sum::<usize>()
        })
        .sum()
}

fn vertical_matches(puzzle: &str, rows: usize, cols: usize) -> usize {
    (0..rows - 3)
        .map(|row| {
            puzzle[row * cols..(row + 1) * cols]
                .chars()
                .zip(puzzle[(row + 1) * cols..(row + 2) * cols].chars())
                .zip(puzzle[(row + 2) * cols..(row + 3) * cols].chars())
                .zip(puzzle[(row + 3) * cols..(row + 4) * cols].chars())
                .map(|(((a, b), c), d)| is_xmas(&[a, b, c, d]) as usize)
                .sum::<usize>()
        })
        .sum()
}

fn backward_diagonal_matches(puzzle: &str, rows: usize, cols: usize) -> usize {
    (0..rows - 3)
        .map(|row| {
            puzzle[row * cols..(row + 1) * cols - 3]
                .chars()
                .zip(puzzle[(row + 1) * cols + 1..(row + 2) * cols - 2].chars())
                .zip(puzzle[(row + 2) * cols + 2..(row + 3) * cols - 1].chars())
                .zip(puzzle[(row + 3) * cols + 3..(row + 4) * cols].chars())
                .map(|(((a, b), c), d)| is_xmas(&[a, b, c, d]) as usize)
                .sum::<usize>()
        })
        .sum()
}

fn forward_diagonal_matches(puzzle: &str, rows: usize, cols: usize) -> usize {
    (0..rows - 3)
        .map(|row| {
            puzzle[row * cols + 3..(row + 1) * cols]
                .chars()
                .zip(puzzle[(row + 1) * cols + 2..(row + 2) * cols - 1].chars())
                .zip(puzzle[(row + 2) * cols + 1..(row + 3) * cols - 2].chars())
                .zip(puzzle[(row + 3) * cols..(row + 4) * cols - 3].chars())
                .map(|(((a, b), c), d)| is_xmas(&[a, b, c, d]) as usize)
                .sum::<usize>()
        })
        .sum()
}

fn xmas_matches(puzzle: &str, rows: usize, cols: usize) -> usize {
    [
        horizontal_matches,
        vertical_matches,
        forward_diagonal_matches,
        backward_diagonal_matches,
    ]
    .iter()
    .map(|f| f(puzzle, rows, cols))
    .sum()
}

fn x_mas_matches(puzzle: &str, rows: usize, cols: usize) -> usize {
    (0..rows - 2)
        .map(|row| {
            puzzle[row * cols..(row + 1) * cols - 2]
                .chars() // top left
                .zip(puzzle[row * cols + 2..(row + 1) * cols].chars()) // top right
                .zip(puzzle[(row + 2) * cols..(row + 3) * cols - 2].chars()) // bottom left
                .zip(puzzle[(row + 2) * cols + 2..(row + 3) * cols].chars()) // bottom right
                .zip(puzzle[(row + 1) * cols + 1..(row + 2) * cols - 1].chars()) // centre
                .map(|((((a, b), c), d), e)| {
                    matches!(
                        (a, b, c, d, e),
                        ('M', 'M', 'S', 'S', 'A')
                            | ('M', 'S', 'M', 'S', 'A')
                            | ('S', 'M', 'S', 'M', 'A')
                            | ('S', 'S', 'M', 'M', 'A')
                    ) as usize
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let puzzle = std::fs::read_to_string(args.path.as_path())?;
    let cols = puzzle.lines().map(|x| x.len()).take(1).collect::<Vec<_>>()[0];
    let flattened = puzzle.replace('\n', "");
    let rows = flattened.len() / cols;
    println!("XMAS: {}", xmas_matches(&flattened, rows, cols));
    println!("X-MAS: {}", x_mas_matches(&flattened, rows, cols));
    Ok(())
}
