// Expected input file names.
#![feature(let_chains, variant_count)]
const SAMPLE1: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Context, Error, Result};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::iter::Cycle;
use std::vec::IntoIter;
// Constants.

// Play one round.
fn game_of_elves(
    elves: &mut HashSet<data::Point>,
    proposals: &mut Cycle<IntoIter<data::Point>>,
) -> bool {
    // First half. Find out which elves might actually move by filtering out those that have no
    // other elf anywhere around them.
    let maybe_moving_elves = elves
        .iter()
        .filter(|el| el.env().into_iter().any(|env| elves.contains(&env)))
        .map(|el| el.clone())
        .collect::<HashSet<data::Point>>();
    let mut props = HashMap::<data::Point, data::Point>::new();
    // Each one of those elves proposes a spot. We propose up to 4 directions.
    for _ in 0..4 {
        let prop = proposals.next().expect("initnite proposals");
        props.extend(
            maybe_moving_elves
                .iter()
                // Only check those that haven't yet proposed something.
                .filter(|el| !props.contains_key(el))
                // Keep only those for whom this direction is clear.
                .filter(|el| {
                    el.dir_env(&prop)
                        .into_iter()
                        .all(|env| !elves.contains(&env))
                })
                .map(|el| (el.clone(), el.add(&prop)))
                .collect::<HashMap<_, _>>(),
        );
    }
    let mut prop_count = HashMap::<data::Point, usize>::new();
    for prop in props.values() {
        if let Some(old) = prop_count.get_mut(&prop) {
            *old += 1;
        } else {
            prop_count.insert(prop.clone(), 1);
        }
    }

    let mut was_moved = false;

    // Second half. Only update to those positions that have been suggested exactly once.
    for (elf, prop) in props {
        if prop_count.get(&prop).unwrap() == &1 {
            elves.remove(&elf);
            elves.insert(prop);
            was_moved = true;
        }
    }

    was_moved
}

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    // Also obtain max coords. Min coords are implicitly 0.
    let (occ_map, _, _) = io::parse_chars_to_data::<data::Input>(file, "input", None, None)?;

    let mut elves = occ_map
        .into_iter()
        .filter_map(|(pos, tile)| {
            if tile == data::Input::Elf {
                Some(pos)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    // This is our infinite stream of proposals.
    let mut proposals = vec![data::N, data::S, data::W, data::E].into_iter().cycle();

    // Consider 10 rounds for part 1.
    for _ in 0..10 {
        game_of_elves(&mut elves, &mut proposals);
        // Discard one element.
        proposals.next().expect("inf it");
    }

    let min_x = elves.iter().map(|el| el.x).min().expect("no min x");
    let max_x = elves.iter().map(|el| el.x).max().expect("no max x");
    let min_y = elves.iter().map(|el| el.y).min().expect("no min y");
    let max_y = elves.iter().map(|el| el.y).max().expect("no max y");

    let mut count = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !elves.contains(&data::Point::new(x, y)) {
                count += 1;
            }
        }
    }

    println!("free field count is {}", count);

    let mut round_count = 10;
    while game_of_elves(&mut elves, &mut proposals) {
        proposals.next().expect("inf it");
        round_count += 1;
    }
    println!("final round count is {}", round_count + 1);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
