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

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let updates = io::parse_lines_to_data::<data::Vec>(file, "vec", None, None)?;

    let mut head = data::NULL_VEC.clone();
    let mut tail = data::NULL_VEC.clone();
    let mut visited = HashSet::<data::Vec>::new();

    visited.insert(tail.clone());

    // Part 1.
    for update in updates.iter().map(|el| el.iter()).flatten() {
        // The mv method will make sure we never move the head farther than one space.
        head.mv(&update)?;
        let tail_update = head.get_tail_update(&tail);
        tail = tail.add(&tail_update);
        visited.insert(tail.clone());
        println!("{:?} -> {:?}", head, tail);
    }

    println!("{:?} => {}", visited, visited.len());

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
