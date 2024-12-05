use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{command, Parser};
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    ordering: PathBuf,
    input: PathBuf,
}

#[derive(serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct OrderRule {
    page: usize,
    before: usize,
}

impl OrderRule {
    fn applicable(&self, pages: &[usize]) -> bool {
        pages.contains(&self.page) && pages.contains(&self.before)
    }
}

trait OrderRuleSet {
    fn subset(&self, pages: &[usize]) -> Self;

    fn order_map(&self) -> HashMap<usize, usize>;

    fn sort(&self, pages: &[usize]) -> Vec<usize>;
}

impl OrderRuleSet for HashSet<OrderRule> {
    fn subset(&self, pages: &[usize]) -> Self {
        // Get relevant subset of the ordering rules for a given set of pages.
        // There are cycles in the total rule set, so this step is necessary
        // before building an ordering map.
        self.iter()
            .filter(|x| x.applicable(pages))
            .copied()
            .collect::<Self>()
    }

    fn order_map(&self) -> HashMap<usize, usize> {
        // Get set of all pages to be ordered
        let pages = self
            .iter()
            .map(|x| x.page)
            .chain(self.iter().map(|x| x.before))
            .collect::<HashSet<_>>();
        // Map each page number to itself: the default ordering
        let mut map = pages
            .iter()
            .map(|val| (*val, *val))
            .collect::<HashMap<_, _>>();
        loop {
            // For all permutation of ((page1, order1), (page2, order2)), check to see
            // if there is a rule saying page page1 must be before page 2, but
            // order 1 is greater than order 2
            let swap = map.iter().permutations(2).find(|c| {
                let (&k1, &v1) = c[0];
                let (&k2, &v2) = c[1];
                self.contains(&OrderRule {
                    page: k1,
                    before: k2,
                }) && v1 > v2
            });
            // If such a rule violation were found, swap the order of those pages.
            // Continue until no violations are found.
            match swap {
                Some(c) => {
                    let (&k1, &v1) = c[0];
                    let (&k2, &v2) = c[1];
                    map.insert(k1, v2);
                    map.insert(k2, v1);
                }
                None => break,
            }
        }
        map
    }

    fn sort(&self, pages: &[usize]) -> Vec<usize> {
        let mut sorted = pages.to_vec();
        let map = self.order_map();
        sorted.sort_by(|a, b| match (map.get(a), map.get(b)) {
            (Some(x), Some(y)) => x.cmp(y),
            _ => std::cmp::Ordering::Equal,
        });
        sorted
    }
}

fn read_order_rules(path: &Path) -> Result<HashSet<OrderRule>, Box<dyn Error>> {
    Ok(csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(path)?
        .deserialize()
        .map(|line| line as Result<OrderRule, _>)
        .collect::<Result<HashSet<OrderRule>, _>>()?)
}

fn read_manuals(path: &Path) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let order_rules = read_order_rules(args.ordering.as_path())?;
    let manuals = read_manuals(args.input.as_path())?;

    // Sort all into correct order
    let sorted_manuals = manuals
        .iter()
        .map(|pages| order_rules.subset(pages).sort(pages))
        .collect::<Vec<_>>();

    // Sum middle pages for the ordered and unordered parts respectively
    let middle_pages = manuals
        .iter()
        .zip(sorted_manuals.iter())
        .map(|(left, right)| {
            if left.iter().zip(right.iter()).all(|(l, r)| l == r) {
                (left[left.len() / 2], 0usize)
            } else {
                (0usize, right[right.len() / 2])
            }
        })
        .reduce(|acc, (x, y)| (acc.0 + x, acc.1 + y))
        .unwrap_or((0usize, 0usize));

    println!("Ordered middle pages: {}", middle_pages.0);
    println!("Unordered middle pages: {}", middle_pages.1);

    Ok(())
}
