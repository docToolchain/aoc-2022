#![feature(let_chains)]
// Expected input file names.
const SAMPLE: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Error, Result};
use std::collections::HashSet;
// Constants.

fn solve(file: &str, win: usize) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file.
    let lines = io::read_lines_from_file(file)?;

    for line in lines {
        let first_match = line
            .chars()
            .collect::<Vec<_>>()
            .as_slice()
            .windows(win)
            .enumerate()
            .filter_map(|(idx, el)| {
                // If the size of a set is equal to the window size, then we have only unique
                // entries. There is no other way.
                if HashSet::<char>::from_iter(el.to_vec().into_iter()).len() == win {
                    Some(idx)
                } else {
                    None
                }
            })
            .take(1)
            .next()
            .ok_or(Error::msg("cannot find matching entry"))?;

        println!("first line that fits: {}", first_match + win)
    }

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE, 4)?;
    solve(REAL, 4)?;

    solve(SAMPLE, 14)?;
    solve(REAL, 14)?;

    Ok(())
}
// end::main[]
