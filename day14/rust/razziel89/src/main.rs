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
const SOURCE: data::Point = data::Point { x: 500, y: 0 };

fn is_blocked(p: &data::Point, rocks: &Vec<data::Rocks>, sands: &HashSet<data::Point>) -> bool {
    if let Some(_) = sands.get(p) {
        true
    } else {
        rocks.iter().any(|el| el.contains(*p))
    }
}

fn render(rocks: &Vec<data::Rocks>, sands: &HashSet<data::Point>) -> String {
    let mut image = String::new();
    let empty = HashSet::<data::Point>::new();

    for y in 0..10 {
        for x in 494..504 {
            let p = data::Point { x, y };
            // This point is a rock.
            let char = if is_blocked(&p, rocks, &empty) {
                '#'
            // This point is sand.
            } else if is_blocked(&p, rocks, sands) {
                'o'
            // This point is free.
            } else {
                '.'
            };
            image.push(char);
        }
        image.push('\n');
    }
    image
}

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let rocks = io::parse_chunks_to_data::<data::Rocks>(
        io::read_lines_from_file(file, 1)?,
        "rocks",
        None,
        None,
    )?;

    let max_y = rocks
        .iter()
        .map(|el| el.edges.iter())
        .flatten()
        .map(|el| el.y)
        .max()
        .ok_or(Error::msg("cannot find highest point"))?;
    let mut highest_y = SOURCE.y;

    let mut sands = HashSet::<data::Point>::new();

    // If this condition is no longer fulfilled, a piece of sand has exceeded our world and will
    // fall to infinity.
    while highest_y <= max_y {
        // Spawn new sand.
        let mut sand = SOURCE;
        let mut has_settled = false;
        while !has_settled && sand.y <= max_y {
            // Find the highest point that contains either sand or is a rock.
            let mut next = sand.down();
            while !is_blocked(&next, &rocks, &sands) && sand.y <= max_y {
                sand = next;
                next = sand.down()
            }
            // If we reach here, that piece of sand has hit an occupied tile on its way down.
            // We don't check again whether we exceeded our world because that will be checked the
            // nxt time we reach the top of the while loop.
            //
            // Check down to the left first.
            next = sand.left_down();
            if !is_blocked(&next, &rocks, &sands) {
                sand = next;
                continue;
            }
            // Check down to the right next.
            next = sand.right_down();
            if !is_blocked(&next, &rocks, &sands) {
                sand = next;
                continue;
            }
            has_settled = true;
            sands.insert(sand);
        }
        if sand.y > highest_y {
            highest_y = sand.y;
        }
    }

    let mut sorted = Vec::<data::Point>::from_iter(sands.into_iter());
    sorted.sort_by(|el1, el2| {
        let x_cmp = el1.x.cmp(&el2.x);
        if x_cmp == std::cmp::Ordering::Equal {
            el1.y.cmp(&el2.y)
        } else {
            x_cmp
        }
    });

    println!("amount of sand is {:?}", sorted.len());

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    // solve(REAL)?;
    Ok(())
}
// end::main[]
