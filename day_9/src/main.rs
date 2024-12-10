use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    path: PathBuf,
}

const SPACE: usize = usize::MAX;

fn read_diskmap(path: &Path) -> Result<Vec<usize>, Box<dyn Error>> {
    let map = std::fs::read_to_string(path)?;
    Ok(map
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn gen_filesystem(diskmap: &[usize]) -> Vec<usize> {
    diskmap
        .iter()
        .enumerate()
        .flat_map(|(idx, val)| {
            if (idx % 2) == 0 {
                vec![idx / 2; *val]
            } else {
                vec![SPACE; *val]
            }
        })
        .collect()
}

fn not_space(x: &&usize) -> bool {
    **x != SPACE
}

fn compact_and_checksum_v1(diskmap: &[usize]) -> usize {
    let filesystem = gen_filesystem(diskmap);
    let len = filesystem.iter().filter(not_space).count();
    filesystem
        .iter()
        .scan(filesystem.iter().rev().filter(not_space), |rev_iter, x| {
            if not_space(&x) {
                Some(*x)
            } else {
                Some(*rev_iter.next().unwrap_or(&SPACE)) // unwrap should never fail
            }
        })
        .take(len)
        .enumerate()
        .map(|(idx, val)| idx * val)
        .sum()
}

fn start_locations(v: &[usize]) -> Vec<usize> {
    v.iter()
        .scan(0usize, |state, x| {
            let result = *state;
            *state += x;
            Some(result)
        })
        .collect()
}

fn compact_and_checksum_v2(diskmap: &[usize]) -> usize {
    // Assume odd number of elements to diskmap
    let starts = start_locations(diskmap);
    diskmap
        .iter()
        .enumerate()
        .rev()
        .step_by(2)
        .scan(
            (diskmap.to_vec(), starts.to_vec()),
            |(diskmap, starts), (idx, width)| {
                let num = idx / 2;
                if let Some(space_idx) =
                    &diskmap[1..]
                        .iter()
                        .zip(1..)
                        .step_by(2)
                        .find_map(|(space_width, space_idx)| {
                            if space_width >= width && space_idx < idx {
                                Some(space_idx)
                            } else {
                                None
                            }
                        })
                {
                    let space_idx = *space_idx;
                    let real_idx = starts[space_idx];
                    diskmap[space_idx] -= width;
                    diskmap[space_idx - 1] += width;
                    starts[space_idx] += width;
                    Some((real_idx..real_idx + width).map(|x| x * num).sum::<usize>())
                } else {
                    let real_idx = starts[idx];
                    Some((real_idx..real_idx + width).map(|x| x * num).sum::<usize>())
                }
            },
        )
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let diskmap = read_diskmap(args.path.as_path())?;
    let checksum_v1 = compact_and_checksum_v1(&diskmap);
    println!("Checksum v1: {checksum_v1}");
    let checksum_v2 = compact_and_checksum_v2(&diskmap);
    println!("Checksum v2: {checksum_v2}");
    Ok(())
}
