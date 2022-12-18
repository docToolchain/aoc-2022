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
use std::collections::{HashMap, HashSet};
use std::iter::Cycle;
use std::vec::IntoIter;
// Constants.

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let lava_lines = io::parse_chunks_to_data::<data::Pos>(
        io::read_lines_from_file(file, 1)?,
        "pos",
        None,
        None,
    )?;

    // Part 1.

    let lava = HashSet::<data::Pos>::from_iter(lava_lines.into_iter());

    let surface_area_part1 = lava
        .iter()
        .map(|el| el.env().into_iter())
        .flatten()
        .filter(|el| !lava.contains(el))
        .count();

    println!("for part 1, the surface area is {}", surface_area_part1);

    // Part 2.
    let min_x = lava
        .iter()
        .map(|el| el.x)
        .min()
        .ok_or(Error::msg("min x"))?;
    let max_x = lava
        .iter()
        .map(|el| el.x)
        .max()
        .ok_or(Error::msg("max x"))?;
    let min_y = lava
        .iter()
        .map(|el| el.y)
        .min()
        .ok_or(Error::msg("min y"))?;
    let max_y = lava
        .iter()
        .map(|el| el.y)
        .max()
        .ok_or(Error::msg("max y"))?;
    let min_z = lava
        .iter()
        .map(|el| el.z)
        .min()
        .ok_or(Error::msg("min z"))?;
    let max_z = lava
        .iter()
        .map(|el| el.z)
        .max()
        .ok_or(Error::msg("max z"))?;

    println!(
        "{} => {} {} | {} {} | {} {}",
        lava.len(),
        min_x,
        max_x,
        min_y,
        max_y,
        min_z,
        max_z
    );

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
