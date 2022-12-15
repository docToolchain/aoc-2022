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
const MULT: isize = 1_000_000;

fn env(var: &str, def: &str) -> String {
    std::env::var(var).unwrap_or(def.to_string())
}

fn solve(file: &str, y: isize, max: isize) -> Result<()> {
    println!("PROCESSING {} WITH Y {}", file, y);

    // Read file and convert into data.
    let exclusion_zones = io::parse_chunks_to_data::<data::Diamond>(
        io::read_lines_from_file(file, 1)?,
        "diamonds",
        None,
        None,
    )?;

    let beacons = exclusion_zones
        .iter()
        .map(|el| (el.bx, el.by))
        .collect::<HashSet<_>>();
    let sensors = exclusion_zones
        .iter()
        .map(|el| (el.x, el.y))
        .collect::<HashSet<_>>();
    let objects = &beacons | &sensors;
    println!("{:?}", objects);

    // Part 1.
    let count = exclusion_zones
        .iter()
        .map(|el| el.xs_at_y(&y))
        .flatten()
        // This is a lazy filter to detect clashes with existing objects but I didn't want to put
        // too much effort into part 1.
        .filter(|el| !objects.contains(&(*el, y)))
        .collect::<HashSet<_>>()
        .len();

    println!("{:?} {}", exclusion_zones, count);

    // Part 2. This is a brute force solution.
    // Default to including everything.
    let min_x = env("MIN", "-1").parse::<isize>()? * MULT;
    let max_x = env("MAX", "5").parse::<isize>()? * MULT;

    for x in 0..max + 1 {
        if x < min_x || x > max_x {
            continue;
        }
        for y in 0..max + 1 {
            if !exclusion_zones.iter().any(|el| el.contains(&x, &y)) {
                // We found it!
                println!("distress beacon found at {} {}", x, y);
                return Ok(());
            }
        }
        if x % 100 == 0 {
            println!("{}%", x as f64 / max as f64 * 100.0);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1, 10, 20)?;
    solve(REAL, 2_000_000, 4_000_000)?;
    Ok(())
}
// end::main[]
