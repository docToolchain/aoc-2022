#![feature(let_chains)]
// Expected input file names.
const SAMPLE1: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Error, Result};
use std::collections::HashSet;
// Constants.

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let valves = io::parse_chunks_to_data::<data::Valve>(
        io::read_lines_from_file(file, 1)?,
        "valves",
        None,
        None,
    )?;

    for v in valves {
        println!("{:?}", v);
    }

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;
    Ok(())
}
// end::main[]
