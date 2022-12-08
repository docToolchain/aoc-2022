#![feature(let_chains)]
// Expected input file names.
const SAMPLE: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Error, Result};
use std::collections::{HashMap, HashSet};
// Constants.
const MAX_HEIGHT: u8 = 10;

fn count_visible(
    forest: &HashMap<(i64, i64), data::Tree>,
    start_pos: &data::Vec,
    count_disp: &data::Vec,
    outer_disp: Option<&data::Vec>,
    max_height: &u8,
    verbose: bool,
    size_cmp_fn: Option<fn(&u8, &u8) -> bool>,
) -> HashSet<(i64, i64)> {
    let size_cmp = size_cmp_fn.unwrap_or(|size, largest| size > largest);
    let mut count = HashSet::<(i64, i64)>::new();

    let mut outer_start = start_pos.clone();

    while let Some(_) = forest.get(&outer_start.pos()) {
        let mut largest: u8 = 0;
        let mut pos = outer_start.clone();

        while &largest < max_height && let Some(tree) = forest.get(&pos.pos()) {
            let size = tree.size();
            if size_cmp(&size, &largest) {
                count.insert(pos.pos());
                largest = size;
                if verbose {
                    eprintln!("visible ({},{}) = {} <= {}", pos.pos().0, pos.pos().1, tree.size()-1, max_height-1);
                }
            }

            pos = pos.add(&count_disp);
        }

        if let Some(disp) = outer_disp {
            outer_start = outer_start.add(disp);
        } else {
            break;
        }
    }

    if verbose {
        eprintln!("{:?}", count);
    }

    count
}

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let forest = io::parse_chars_to_data::<data::Tree>(file, "tree")?;

    // Part 1.
    let max_x = forest
        .keys()
        .map(|el| el.0)
        .max()
        .ok_or(Error::msg("cannot find max x index"))?;
    let max_y = forest
        .keys()
        .map(|el| el.1)
        .max()
        .ok_or(Error::msg("cannot find max x index"))?;

    let mut count = HashSet::<(i64, i64)>::new();
    // Top border rightwards.
    count = &count
        | &count_visible(
            &forest,
            &data::Vec::new(0, 0),
            &data::Vec::new(0, 1),
            Some(&data::Vec::new(1, 0)),
            &MAX_HEIGHT,
            false,
            None,
        );
    // Left border downwards.
    count = &count
        | &count_visible(
            &forest,
            &data::Vec::new(0, 0),
            &data::Vec::new(1, 0),
            Some(&data::Vec::new(0, 1)),
            &MAX_HEIGHT,
            false,
            None,
        );
    // Bottom border rightwards.
    count = &count
        | &count_visible(
            &forest,
            &data::Vec::new(0, max_y),
            &data::Vec::new(0, -1),
            Some(&data::Vec::new(1, 0)),
            &MAX_HEIGHT,
            false,
            None,
        );
    // Right border downwards.
    count = &count
        | &count_visible(
            &forest,
            &data::Vec::new(max_x, 0),
            &data::Vec::new(-1, 0),
            Some(&data::Vec::new(0, 1)),
            &MAX_HEIGHT,
            false,
            None,
        );

    println!("visible: {:?}", count);
    println!("visible are {} trees", count.len());

    // Part 2.
    let disps = vec![
        data::Vec::new(0, -1),
        data::Vec::new(-1, 0),
        data::Vec::new(0, 1),
        data::Vec::new(1, 0),
    ];
    // let check_pos = data::Vec::new(2, 3);

    let best_view = forest
        .iter()
        .map(|(pos, height)| {
            disps
                .iter()
                .map(|disp| {
                    let tree_pos = data::Vec::new(pos.0, pos.1);
                    count_visible(
                        &forest,
                        &tree_pos.add(&disp),
                        disp,
                        None,
                        &height.size(),
                        // tree_pos == check_pos,
                        false,
                        Some(|_size, _largest| true),
                    )
                    .len()
                })
                .product::<usize>()
        })
        .max()
        .ok_or(Error::msg("cannot find best view"))?;

    println!("best view is {}", best_view);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
