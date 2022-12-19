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
// Constants.
const MAX_TIME: usize = 24;

fn evolve(bp: data::Blueprint) -> data::Size {
    let mut best = 0;

    // Perform 1000 optimisations per blueprint.
    for _ in 0..1_000 {
        let mut state = data::State::start();

        for _time in 0..MAX_TIME {
            state.next(&bp);
        }
        if state.geode > best {
            best = state.geode;
        }
    }

    best
}

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let blueprints = io::parse_chunks_to_data::<data::Blueprint>(
        io::read_lines_from_file(file, 1)?,
        "blueprint",
        None,
        None,
    )?;

    // Part 1.
    println!("{:?}", blueprints);
    let mut best = vec![0; blueprints.len()];

    loop {
        for (idx, bp) in blueprints.iter().enumerate() {
            let possible_best = evolve(bp.clone());
            if possible_best > best[idx] {
                best[idx] = possible_best;
                println!("found better: {} {}", idx, possible_best);
                let quality_level: data::Size = best
                    .iter()
                    .zip(blueprints.iter())
                    .map(|(val, bp)| bp.id * val)
                    .sum();
                println!("the new overall quality level is: {}", quality_level);
            }
        }
    }
}

fn main() -> Result<()> {
    // solve(SAMPLE1)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
