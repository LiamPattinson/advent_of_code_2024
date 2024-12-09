use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    path: PathBuf,
}

const SPACE: usize = usize::MAX;

fn read_diskmap(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let map = std::fs::read_to_string(path)?;
    Ok(map
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn gen_filesystem(diskmap: &[u8]) -> Vec<usize> {
    diskmap
        .iter()
        .enumerate()
        .flat_map(|(idx, map)| {
            if (idx % 2) == 0 {
                vec![idx / 2; *map as usize]
            } else {
                vec![SPACE; *map as usize]
            }
        })
        .collect()
}

fn not_space(x: &&usize) -> bool {
    **x != SPACE
}

fn compact_and_checksum(diskmap: &[u8]) -> usize {
    let filesystem = gen_filesystem(diskmap);
    let len = filesystem.iter().filter(not_space).count();
    let mut rev_iter = filesystem.iter().rev().filter(not_space);
    filesystem
        .iter()
        .map(|x| {
            if not_space(&x) {
                *x
            } else {
                *rev_iter.next().unwrap_or(&SPACE) // unwrap should never fail
            }
        })
        .take(len)
        .enumerate()
        .map(|(idx, val)| idx * val)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let diskmap = read_diskmap(args.path.as_path())?;
    let checksum_v1 = compact_and_checksum(&diskmap);
    println!("Checksum v1: {checksum_v1}");
    Ok(())
}
