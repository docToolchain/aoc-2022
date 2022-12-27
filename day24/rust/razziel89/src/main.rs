// Expected input file names.
#![feature(let_chains, variant_count)]
const SAMPLE1: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Context, Error, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
// Constants.

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    // Also obtain max coords. Min coords are implicitly 0.
    let (occ_map, max_x, max_y) = io::parse_chars_to_data::<data::Tile>(file, "tile", None, None)?;
    let max = data::Point::new(max_x, max_y, 0);

    println!("{:?}", occ_map);
    println!("{:?}", max);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    // solve(REAL)?;

    Ok(())
}
// end::main[]
