use std::error::Error;
use std::path::PathBuf;

use clap::{command, Parser};
use lazy_regex::{regex, regex_replace};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    path: PathBuf,
}

fn unscramble_and_solve(code: &str) -> Result<i64, Box<dyn Error>> {
    // Everything that matches `mul(x,y)` should be included, where `x` and `y`
    // are integers up to 3 digits. Any other characters including whitespace
    // void the pattern.
    Ok(regex!(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")
        .captures_iter(code)
        .map(|captures| {
            let (_, [left, right]) = captures.extract();
            match (left.parse::<i64>(), right.parse::<i64>()) {
                (Ok(left), Ok(right)) => Ok(left * right),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum())
}

fn with_do_and_dont(code: &str) -> String {
    // Enable code following a `do()`, disable after a `don't()`.
    // The code starts enabled.
    //
    // Can't use the regex `r"don't\(\).*do\(\)"`, as greedy algorithm eats
    // everything up to the last `do()`.
    //
    // Instead:
    // - Split on do()
    // - Delete everything after don't()
    // - Stick back together
    regex!(r"do\(\)")
        .split(code)
        .map(|x| regex_replace!(r"don't\(\).*", &x, ""))
        .collect::<String>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let code = std::fs::read_to_string(args.path.as_path())?.replace('\n', "");
    println!("Standard result: {}", unscramble_and_solve(code.as_str())?);
    println!(
        "Conditional result: {}",
        unscramble_and_solve(&with_do_and_dont(&code))?
    );
    Ok(())
}
