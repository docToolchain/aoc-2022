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
use std::collections::{HashMap, HashSet};
// Constants.
const MULT: isize = 1_000_000;

fn gather_unique_zones(input: &Vec<data::Diamond>) -> Vec<data::Diamond> {
    // Find those diamonds that are fully enclosed in another one.
    let overlaps = (0..input.len())
        .map(|small_idx| {
            (0..input.len()).filter_map(move |big_idx| {
                if big_idx != small_idx && input[big_idx].encompasses(&input[small_idx]) {
                    Some((small_idx, big_idx))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<HashMap<_, _>>();
    eprintln!("overlaps {:?} #{}", overlaps, overlaps.len());

    // Keep those that aren't contained in any other diamond, which means their indics don't
    // appear on any left side.
    input
        .iter()
        .enumerate()
        .filter_map(|(idx, el)| {
            if !overlaps.iter().any(|el| el.0 == &idx) {
                Some(el.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
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
    println!("number of diamons is {}", exclusion_zones.len());

    // Copy the diamonds.
    let fewer_zones = gather_unique_zones(&exclusion_zones);
    println!("number of unique diamonds is {}", fewer_zones.len());

    let beacons = exclusion_zones
        .iter()
        .map(|el| (el.bx, el.by))
        .collect::<HashSet<_>>();
    let sensors = exclusion_zones
        .iter()
        .map(|el| (el.x, el.y))
        .collect::<HashSet<_>>();
    let objects = &beacons | &sensors;

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

    println!("number of points along {} is {}", y, count);

    // Part 2.
    // Nothing yet.

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1, 10, 20)?;
    solve(REAL, 2_000_000, 4_000_000)?;
    Ok(())
}
// end::main[]
