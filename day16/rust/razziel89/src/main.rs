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
const START: &'static str = "AA";

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
    relevant_valves: &Vec<Option<String>>,
    current_spot: &String,
    current_time: usize,
    max_time: usize,
    current_best: usize,
    visited: &mut HashSet<String>,
    allow_elephant: bool,
    elephant_depth: usize,
) -> Result<usize> {
    // Check that None is the first entry in the list of relevant_valves.
    if allow_elephant && !relevant_valves[0].as_ref().is_none() {
        return Err(Error::msg("the first relevant valve has to be None"));
    }
    let mut next_best = current_best;

    for maybe_next_spot in relevant_valves.iter() {
        let possible_best = if let Some(next_spot) = maybe_next_spot {
            // Let the human do their thing.
            // Skip spots we've already seen.
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
                // If not, we found the best we can using this route. Skip this spot, but make sure
                // to let the elephant do its thing.
                continue;
            } else {
                // We will be able to open that valve.
                // Remember that we visited it.
                visited.insert(next_spot.to_string());

                // Check by how much the next valve can increase our release value.
                let next_valve_rate = valve_name_map
                    .get(next_spot)
                    .ok_or(Error::msg("cannot retrieve valve"))?
                    .rate;
                let benefit = (max_time - time_of_next_valve_opening) * next_valve_rate;

                backtrack(
                    valve_name_map,
                    distances,
                    relevant_valves,
                    &next_spot,
                    time_of_next_valve_opening,
                    max_time,
                    current_best + benefit,
                    visited,
                    allow_elephant,
                    elephant_depth,
                )?
            }
        } else if allow_elephant && visited.len() == elephant_depth {
            // Let the elephant also do its thing, allowing it only to visit those points that we
            // haven't yet visited. Actually, this has to be the first step we do for the algorithm
            // to work. Otherwise, better paths will be missed.
            backtrack(
                valve_name_map,
                distances,
                relevant_valves,
                &START.to_string(),
                0,
                max_time,
                next_best,
                visited,
                // Don't allow the elephant to explore again.
                false,
                elephant_depth,
            )?
        } else {
            // This block allows us to still run part 1.
            0
        };
        if possible_best > next_best {
            next_best = possible_best;
        }

        // Forget that we visited the point but only if this isn't the elephant's turn.
        if let Some(next_spot) = maybe_next_spot {
            visited.remove(next_spot);
        }
    }
    // Return the bext one we found.
    Ok(next_best)
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
        .map(|el| Some(el))
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
    let start_release = 0;
    let mut visited = HashSet::<String>::with_capacity(relevant_valves.len());

    let max_part1 = backtrack(
        &valve_name_map,
        &distances,
        &relevant_valves,
        &START.to_string(),
        start_time,
        max_time,
        start_release,
        &mut visited,
        false,
        0,
    )?;
    println!("part 1: {}", max_part1);

    // Part 2.
    // Backtrack the solution.

    let max_time_part2 = 26;
    // A value of None means that the human should stop what they are doing and let the elephant do
    // its thing. It has to be the first entry in relevant_valves. Otherwise, better paths are
    // lost, somehow.
    let mut relevant_with_elephant = vec![None];
    relevant_with_elephant.extend_from_slice(&relevant_valves);

    // Quick mode uses the below assumptions and we use it by default.
    let quick_mode = std::env::var("QUICK").unwrap_or("1".to_string()) == "1";

    let mut best = 0;
    // This is a bit hacky but it works. We check what happens if the human is guaranteed a certain
    // number of valves because the elephant can only open those that the human didn't approach.
    // Then, we assume that, the fewer valves the human opens, the higher the overal pressure
    // release gets because the elephant can open some. At some point, since this is at least a
    // certain number of steps, there will no longer be an increase followed by a decrease, which
    // is our stopping point. This is an assumption, which tunrs out to hold, but it might not for
    // all possible inputs. If it doesn't simply remove the breaking condition and use the overall
    // max, it'll just take longer.
    for num_human_valves in (0..relevant_valves.len()).rev() {
        // Reset some mutable data structures.
        visited = HashSet::<String>::with_capacity(relevant_valves.len());

        let max = backtrack(
            &valve_name_map,
            &distances,
            &relevant_with_elephant,
            &START.to_string(),
            start_time,
            max_time_part2,
            start_release,
            &mut visited,
            true,
            num_human_valves,
        )?;
        println!(
            "part 2: with {} guaranteed human valves: {}",
            num_human_valves, max
        );
        if quick_mode {
            if max < best {
                break;
            } else {
                best = max;
            }
        } else {
            if max > best {
                best = max;
            }
        }
    }
    println!("part 2: {}", best);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;
    Ok(())
}
// end::main[]
