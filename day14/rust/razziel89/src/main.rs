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

fn is_blocked(
    p: &data::Point,
    rocks: &Vec<data::Rocks>,
    sands: &HashSet<data::Point>,
    max_y: Option<isize>,
) -> bool {
    if let Some(_) = sands.get(p) {
        true
    } else if let Some(bottom) = max_y && p.y >= bottom {
        true
    } else {
        rocks.iter().any(|el| el.contains(*p))
    }
}

fn render(
    rocks: &Vec<data::Rocks>,
    sands: &HashSet<data::Point>,
    fov: (isize, isize, isize, isize),
) -> String {
    let dist = 10;
    let mut image = String::new();
    let empty = HashSet::<data::Point>::new();
    let min_y = if fov.1 - 4 <= -2 { fov.1 - 2 } else { -2 };

    println!("{:?}", fov);

    for y in min_y..fov.3 + 4 {
        for x in fov.0 - dist..fov.2 + dist {
            let p = data::Point { x, y };
            // This point is a rock.
            let char = if p == SOURCE {
                'S'
            } else if p.y == fov.3 + 2 {
                '#'
            } else if is_blocked(&p, rocks, &empty, None) {
                '#'
            // This point is sand.
            } else if is_blocked(&p, rocks, sands, None) {
                '.'
            // This point is free.
            } else {
                ' '
            };
            image.push(char);
        }
        image.push('\n');
    }
    image
}

fn solve(file: &str, part1: bool) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let rocks = io::parse_chunks_to_data::<data::Rocks>(
        io::read_lines_from_file(file, 1)?,
        "rocks",
        None,
        None,
    )?;

    // Coordinates for rendering.
    let min_x_rocks = rocks
        .iter()
        .map(|el| el.edges.iter())
        .flatten()
        .map(|el| el.x)
        .min()
        .ok_or(Error::msg("cannot find xmin"))?;
    let min_y_rocks = rocks
        .iter()
        .map(|el| el.edges.iter())
        .flatten()
        .map(|el| el.y)
        .min()
        .ok_or(Error::msg("cannot find ymin"))?;
    let max_x_rocks = rocks
        .iter()
        .map(|el| el.edges.iter())
        .flatten()
        .map(|el| el.x)
        .max()
        .ok_or(Error::msg("cannot find xmax"))?;
    let max_y_rocks = rocks
        .iter()
        .map(|el| el.edges.iter())
        .flatten()
        .map(|el| el.y)
        .max()
        .ok_or(Error::msg("cannot find ymax"))?;
    let render_coords = (min_x_rocks, min_y_rocks, max_x_rocks, max_y_rocks);
    let mut highest_y = SOURCE.y;

    let max_y = if part1 { max_y_rocks } else { max_y_rocks + 2 };

    let mut sands = HashSet::<data::Point>::new();

    let blocked_y = if part1 { None } else { Some(max_y) };

    let do_render = std::env::var("RENDER").unwrap_or("0".to_string()) == "1";

    // If this condition is no longer fulfilled, a piece of sand has exceeded our world and will
    // fall to infinity.
    loop {
        // Spawn new sand.
        if do_render {
            println!("{}", render(&rocks, &sands, render_coords));
        }
        let mut sand = SOURCE;
        let mut has_settled = false;
        while !has_settled && sand.y <= max_y {
            // Find the highest point that contains either sand or is a rock.
            let mut next = sand.down();
            while !is_blocked(&next, &rocks, &sands, blocked_y) && sand.y <= max_y {
                sand = next;
                next = sand.down()
            }
            // If we reach here, that piece of sand has hit an occupied tile on its way down.
            // We don't check again whether we exceeded our world because that will be checked the
            // nxt time we reach the top of the while loop.
            //
            // Check down to the left first.
            next = sand.left_down();
            if !is_blocked(&next, &rocks, &sands, blocked_y) {
                sand = next;
                continue;
            }
            // Check down to the right next.
            next = sand.right_down();
            if !is_blocked(&next, &rocks, &sands, blocked_y) {
                sand = next;
                continue;
            }
            has_settled = true;
            sands.insert(sand);
        }
        if sand.y > highest_y {
            highest_y = sand.y;
        }
        if part1 {
            if highest_y >= max_y {
                break;
            }
        } else {
            // Break if the source has been blocked.
            if let Some(_) = sands.get(&SOURCE) {
                break;
            }
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
    solve(SAMPLE1, true)?;
    solve(REAL, true)?;

    solve(SAMPLE1, false)?;
    solve(REAL, false)?;

    Ok(())
}
// end::main[]
