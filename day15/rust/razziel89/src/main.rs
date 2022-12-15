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
use std::collections::HashSet;
// Constants.

fn find_missing_x(
    diamonds: &Vec<data::Diamond>,
    outer_min: isize,
    outer_max: isize,
    y: isize,
) -> Option<(isize, isize)> {
    // Extract all ranges at the given y-coordinate first.
    let mut ranges = diamonds
        .iter()
        .map(|el| el.xrange_at_y(&y).clamp(outer_min, outer_max))
        .filter(|el| el != &data::NULL_RANGE)
        .collect::<Vec<_>>();

    // Then, we sort by left coordinate first and by right coordinate second. That makes finding
    // the missing x spot trivial.
    ranges.sort_by(|range1, range2| {
        let left_cmp = range1.left.cmp(&range2.left);
        if left_cmp == std::cmp::Ordering::Equal {
            range1.right.cmp(&range2.right)
        } else {
            left_cmp
        }
    });

    let mut max = ranges[0].right;
    if ranges[0].left != outer_min {
        // Wouldn't that be nice.
        Some((outer_min, y))
    } else {
        for range in ranges[1..].into_iter() {
            // We can never find a left coordinate that is smaller than what we already have.
            if range.left > max {
                // Yeah, found it!
                return Some((max, y));
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

fn solve(file: &str, y: isize, (min, max): (isize, isize)) -> Result<()> {
    println!("PROCESSING {} WITH Y {}", file, y);

    // Read file and convert into data.
    let exclusion_zones = io::parse_chunks_to_data::<data::Diamond>(
        io::read_lines_from_file(file, 1)?,
        "diamonds",
        None,
        None,
    )?;
    println!("number of diamons is {}", exclusion_zones.len());

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
    // There is guaranteed to be exactly one point.
    let missing = (min..max + 1).find_map(|y| find_missing_x(&exclusion_zones, min, max + 1, y));

    if let Some((x, y)) = missing {
        println!("tuning frequency is {}", x * max + y);
        Ok(())
    } else {
        Err(Error::msg("there is no distress beacon"))
    }
}

fn main() -> Result<()> {
    solve(SAMPLE1, 10, (0, 20))?;
    solve(REAL, 2_000_000, (0, 4_000_000))?;
    Ok(())
}
// end::main[]
