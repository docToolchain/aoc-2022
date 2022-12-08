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
use std::collections::{HashMap, HashSet};
// Constants.
const MAX_HEIGHT: u8 = 10;

fn count_visible(
    forest: &HashMap<(i64, i64), data::Tree>,
    start_pos: data::Vec,
    count_disp: data::Vec,
    outer_disp: data::Vec,
) -> HashSet<(i64, i64)> {
    let mut count = HashSet::<(i64, i64)>::new();

    let mut outer_start = start_pos.clone();

    while !outer_disp.empty() && let Some(_) = forest.get(&outer_start.pos()) {
        let mut largest: u8 = 0;
        let mut pos = outer_start.clone();

        while largest < MAX_HEIGHT && let Some(tree) = forest.get(&pos.pos()) {
            let size = tree.size();
            if size > largest {
                count.insert(pos.pos());
                largest = size;
                eprintln!("visible ({},{}) = {}", pos.pos().0, pos.pos().1, tree.size()-1)
            }

            pos = pos.add(&count_disp);
        }

        outer_start = outer_start.add(&outer_disp);
    }

    count
}

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let forest = io::parse_chars_to_data::<data::Tree>(file, "tree")?;

    // Part 1.
    let max_x = forest
        .keys()
        .map(|el| el.0)
        .max()
        .ok_or(Error::msg("cannot find max x index"))?;
    let max_y = forest
        .keys()
        .map(|el| el.1)
        .max()
        .ok_or(Error::msg("cannot find max x index"))?;

    let mut count = HashSet::<(i64, i64)>::new();
    // Top border rightwards.
    count = &count
        | &count_visible(
            &forest,
            data::Vec::new(0, 0),
            data::Vec::new(0, 1),
            data::Vec::new(1, 0),
        );
    // Left border downwards.
    count = &count
        | &count_visible(
            &forest,
            data::Vec::new(0, 0),
            data::Vec::new(1, 0),
            data::Vec::new(0, 1),
        );
    // Bottom border rightwards.
    count = &count
        | &count_visible(
            &forest,
            data::Vec::new(0, max_y),
            data::Vec::new(0, -1),
            data::Vec::new(1, 0),
        );
    // Right border downwards.
    count = &count
        | &count_visible(
            &forest,
            data::Vec::new(max_x, 0),
            data::Vec::new(-1, 0),
            data::Vec::new(0, 1),
        );

    println!("visible: {:?}", count);
    println!("visible are {} trees", count.len());

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
