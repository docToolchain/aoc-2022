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
        let first_match_vec = line
            .chars()
            .collect::<Vec<_>>()
            .as_slice()
            .windows(win)
            .enumerate()
            .filter_map(|(idx, el)| {
                let char_vec = el.to_vec();
                let string_iter = char_vec.iter().map(|el| String::from(el.clone()));
                let set: HashSet<String> = HashSet::from_iter(string_iter);
                if set.len() == win {
                    Some(idx)
                } else {
                    None
                }
            })
            .take(1)
            .collect::<Vec<_>>();

        println!(
            "first line that fits: {}",
            first_match_vec
                .get(0)
                .ok_or(Error::msg("cannot find matching entry"))?
                .checked_add(win)
                .ok_or(Error::msg("cannot increase by window size"))?
        )
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
