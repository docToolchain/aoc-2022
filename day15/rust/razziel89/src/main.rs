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

    // Keep those that aren't contained in any other diamond, which means their indics don't
    // appear on any left side.
    let mut zones = input
        .iter()
        .enumerate()
        .filter_map(|(idx, el)| {
            if !overlaps.iter().any(|el| el.0 == &idx) {
                Some(el.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    zones.sort_by(|el1, el2| el1.size().cmp(&el2.size()));

    zones
}

// This function assumes that the ranges it gets are sorted by their left coordinates first and by
// their right coordinates second.
fn find_missing_x(ranges: &Vec<data::Range>, outer_min: isize, outer_max: isize) -> Option<isize> {
    let mut max = ranges[0].right;
    if ranges[0].left != outer_min {
        // Wouldn't that be nice.
        Some(outer_min)
    } else {
        for range in ranges[1..].into_iter() {
            // We can never find a left coordinate that is smaller than what we already have.
            if range.left > max {
                // Yeah, found it!
                return Some(max);
            } else if range.right > max {
                max = range.right;
            } else if range.right <= max {
                // Don't do anything here.
            } else {
                unreachable!("there are no more conditions");
            }
        }

        None
    }
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

    // Remove all diamonds that are completely enclosed in at least one other diamond.
    // let unique_zones = gather_unique_zones(&exclusion_zones);
    // println!("number of unique diamonds is {}", unique_zones.len());
    let unique_zones = exclusion_zones
        .iter()
        .map(|el| el.clone())
        .collect::<Vec<_>>();

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
    let count = unique_zones
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
    let mut found_y: Option<isize> = None;
    let mut found_x: Option<isize> = None;
    for y in 0..max + 1 {
        let mut ranges = unique_zones
            .iter()
            .map(|el| el.xrange_at_y(&y).clamp(0, max + 1))
            .filter(|el| el != &data::NULL_RANGE)
            .collect::<Vec<_>>();
        // We sort by left coordinate first and by right coordinate second.
        ranges.sort_by(|range1, range2| {
            let left_cmp = range1.left.cmp(&range2.left);
            if left_cmp == std::cmp::Ordering::Equal {
                range1.right.cmp(&range2.right)
            } else {
                left_cmp
            }
        });
        if let Some(x) = find_missing_x(&ranges, 0, max + 1) {
            found_y = Some(y);
            found_x = Some(x);
            break;
        }
    }
    if let (Some(x), Some(y)) = (found_x, found_y) {
        println!("tuning frequency is {}", x * max + y);
        Ok(())
    } else {
        Err(Error::msg("there is no distress beacon"))
    }
}

fn main() -> Result<()> {
    solve(SAMPLE1, 10, 20)?;
    solve(REAL, 2_000_000, 4_000_000)?;
    Ok(())
}
// end::main[]
