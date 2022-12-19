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

fn is_env(var: &str, val: &str, def: &str) -> bool {
    std::env::var(var).unwrap_or(def.to_string()) == val
}

fn exhaustive_search(
    state: data::State,
    bp: &data::Blueprint,
    actions: &Vec<data::WhatToBuild>,
    lru: &mut HashMap<data::State, data::Size>,
) -> data::Size {
    if state.time == 0 {
        return state.geode;
    }
    if let Some(lru_val) = lru.get(&state) {
        return *lru_val;
    }

    let mut best = state.geode;

    for act in actions.iter() {
        if let Some(next) = state.next(bp, act) {
            let possible_best = exhaustive_search(next, bp, actions, lru);
            if possible_best > best {
                best = possible_best;
            }
        }
    }

    // Remember the value we found, but only for early ones.
    if state.time >= 4 {
        lru.insert(state, best);
    }

    return best;
}

fn solve(file: &str, part1: bool) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let blueprints = io::parse_chunks_to_data::<data::Blueprint>(
        io::read_lines_from_file(file, 1)?,
        "blueprint",
        None,
        None,
    )?;

    let actions = vec![
        data::WhatToBuild::GeodeR,
        data::WhatToBuild::ObsidianR,
        data::WhatToBuild::ClayR,
        data::WhatToBuild::OreR,
        data::WhatToBuild::Nothing,
    ];

    if part1 {
        let mut best_vals = vec![];

        for (idx, bp) in blueprints.iter().enumerate() {
            let mut lru = HashMap::<data::State, data::Size>::new();
            let state = data::State::start(24);
            let best = exhaustive_search(state, bp, &actions, &mut lru);
            println!("best for {} is {}", idx + 1, best);
            best_vals.push(best);
        }

        let quality_level = best_vals
            .into_iter()
            .zip(blueprints.iter())
            .map(|(val, bp)| bp.id as usize * val as usize)
            .sum::<usize>();

        println!("the overall quality level is: {}", quality_level);
    } else {
        let mut best_vals = vec![];

        for (idx, bp) in blueprints.iter().take(3).enumerate() {
            // Sadly, we cannot reuse the LRU cache for other blueprints.
            let mut lru = HashMap::<data::State, data::Size>::new();
            if idx == 0 {
                // We've already managed to compute this one for our input.
                lru.insert(data::State::start(32), 30);
            }
            let state = data::State::start(32);
            let best = exhaustive_search(state, bp, &actions, &mut lru);
            println!("best for {} is {}", idx + 1, best);
            best_vals.push(best);
        }

        let quality_level = best_vals
            .into_iter()
            .map(|el| el as usize)
            .product::<usize>();

        println!("the overall quality level is: {}", quality_level);
    }

    Ok(())
}

fn main() -> Result<()> {
    // Run none by default.
    if is_env("RUN", "0", "") {
        solve(SAMPLE1, true)?;
    }
    if is_env("RUN", "1", "") {
        solve(REAL, true)?;
    }

    if is_env("RUN", "2", "") {
        solve(SAMPLE1, false)?;
    }
    if is_env("RUN", "3", "") {
        solve(REAL, false)?;
    }

    Ok(())
}
// end::main[]
