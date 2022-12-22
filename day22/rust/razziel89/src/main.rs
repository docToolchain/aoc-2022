// Expected input file names.
#![feature(let_chains, variant_count)]
const SAMPLE1: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Error, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
// Constants.

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let org_field = io::parse_chars_to_data::<data::Tile>(
        file,
        "tile",
        Some(|el: &String| el.contains(".") || el.contains("#")),
        None,
    )?;
    let actions = io::parse_chunks_to_data::<data::Action>(
        io::read_lines_from_file(file, 1)?,
        "action",
        Some(|el: &String| el.len() != 0 && !el.contains(".") && !el.contains("#")),
        None,
    )?;

    println!("{:?}", org_field);
    println!("{:?}", actions);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    // solve(REAL)?;

    Ok(())
}
// end::main[]
