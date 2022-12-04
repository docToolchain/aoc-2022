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
// Constants.
// None yet.

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let pairs = io::parse_lines_to_data::<data::Pair>(file, "pair")?;

    let full_overlaps = pairs
        .iter()
        .filter_map(|el| if el.full_overlap() { Some(el) } else { None })
        .count();

    println!("there are {} full overlaps", full_overlaps);

    let partial_overlaps = pairs
        .iter()
        .filter_map(|el| if el.partial_overlap() { Some(el) } else { None })
        .count();

    println!("there are {} partial overlaps", partial_overlaps);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
