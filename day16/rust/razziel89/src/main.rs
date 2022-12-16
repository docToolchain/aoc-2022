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

fn backtrack(
    valve_name_map: &HashMap<String, &data::Valve>,
    distances: &HashMap<(String, String), usize>,
    relevant_valves: &Vec<String>,
    current_spot: &String,
    current_time: usize,
    max_time: usize,
    current_best: usize,
    visited: &mut HashSet<String>,
    stack: &mut Vec<(String, usize)>,
) -> Result<(usize, Vec<(String, usize)>)> {
    let mut next_best = current_best;
    let mut best_stack = stack.iter().map(|el| el.clone()).collect();

    for next_spot in relevant_valves.iter() {
        if visited.contains(next_spot) {
            continue;
        }

        let time_spent_traveling = distances
            .get(&(current_spot.to_string(), next_spot.to_string()))
            .ok_or(Error::msg("cannot retrieve distance"))?;

        // We add 1 because of the time it takes to open the valve.
        let time_of_next_valve_opening = current_time + time_spent_traveling + 1;

        // Check whether we have any time left to open another valve.
        if time_of_next_valve_opening >= max_time {
            // If not, we found the best we can using this route. Skip this spot.
            // return Ok((current_best, best_stack));
            continue;
        } else {
            // We will be able to open that valve.
            // Remember that we visited it.
            visited.insert(next_spot.to_string());
            stack.push((next_spot.to_string(), time_of_next_valve_opening));

            // Check by how much the next valve can increase our release value.
            let next_valve_rate = valve_name_map
                .get(next_spot)
                .ok_or(Error::msg("cannot retrieve valve"))?
                .rate;
            let benefit = (max_time - time_of_next_valve_opening) * next_valve_rate;

            let (possible_best, possible_stack) = backtrack(
                valve_name_map,
                distances,
                relevant_valves,
                next_spot,
                time_of_next_valve_opening,
                max_time,
                current_best + benefit,
                visited,
                stack,
            )?;

            if possible_best > next_best {
                next_best = possible_best;
                best_stack = possible_stack;
            }

            // Forget that we visited the point. That's the backtracking bit.
            visited.remove(next_spot);
            stack.pop();
        }
    }

    // In case we've already visited all the possible valves, return the best we found so far.
    Ok((next_best, best_stack))
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
    // We only want to look at valves that have non-zero rates for part 1. I suspect it's gonna be
    // different for part 2.
    let relevant_valves = valves
        .iter()
        .filter_map(|el| {
            if el.rate > 0 {
                Some(el.name.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Compute pairwise distances.
    let valve_name_map = valves
        .iter()
        .map(|el| (el.name.clone(), el))
        .collect::<HashMap<_, _>>();
    let distances = pairwise_distances(&valve_name_map);

    // Backtrack the solution.
    let start_time = 0;
    let max_time = 30;
    let start = "AA".to_string();
    let start_release = 0;
    let mut visited = HashSet::<String>::with_capacity(relevant_valves.len());
    let mut stack = vec![];

    let (max, best_stack) = backtrack(
        &valve_name_map,
        &distances,
        &relevant_valves,
        &start,
        start_time,
        max_time,
        start_release,
        &mut visited,
        &mut stack,
    )?;
    println!("{:?} {}", best_stack, max);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;
    Ok(())
}
// end::main[]
