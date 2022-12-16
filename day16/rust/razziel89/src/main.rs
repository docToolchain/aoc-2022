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

fn pairwise_distances(
    valve_map: &HashMap<String, &data::Valve>,
) -> HashMap<(String, String), usize> {
    let mut result = HashMap::<(String, String), usize>::new();

    for (name, valve) in valve_map {
        let mut curr_dist = 1 as usize;
        // All neighbours have a distance of one.
        let mut distances = valve
            .neighbours
            .iter()
            .map(|el| (el.clone(), curr_dist))
            .collect::<HashMap<_, _>>();
        // We always remeber new points we added. Those points have a distance of one more than the
        // previous max.
        let mut new_valves = distances
            .iter()
            .map(|el| el.0.clone())
            .collect::<HashSet<_>>();
        // We include the point itself for ease of use later on.
        distances.insert(name.clone(), 0);

        // Repeat until no more neighbours have been added.
        while new_valves.len() != 0 {
            curr_dist += 1;
            // Determine those valve names that are at the next distance.
            new_valves = new_valves
                .iter()
                // Extract the actual valve data.
                .filter_map(|el| valve_map.get(el))
                // Get all neighbours of those valves we just extracted in one big iterator.
                .map(|el| el.neighbours.iter())
                .flatten()
                // Get a unique set of the names of those valves that may have been added in this
                // iteration.
                .collect::<HashSet<_>>()
                .into_iter()
                // Keep only those to which we haven't yet computed distances.
                .filter_map(|el| {
                    if let Some(_) = distances.get(el) {
                        None
                    } else {
                        Some(el.clone())
                    }
                })
                .collect::<HashSet<_>>();
            // Add those distances.
            distances.extend(new_valves.iter().map(|el| (el.clone(), curr_dist)));
        }

        result.extend(
            distances
                .into_iter()
                .map(|(new_valve, dist)| ((name.clone(), new_valve), dist)),
        );
    }

    result
}

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let valves = io::parse_chunks_to_data::<data::Valve>(
        io::read_lines_from_file(file, 1)?,
        "valves",
        None,
        None,
    )?;
    let valve_map = valves
        .iter()
        .map(|el| (el.name.clone(), el))
        .collect::<HashMap<_, _>>();

    for v in &valves {
        println!("{:?}", v);
    }

    // Compute pairwise distances.
    let distances = pairwise_distances(&valve_map);
    println!("{:?}", distances);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    // solve(REAL)?;
    Ok(())
}
// end::main[]
