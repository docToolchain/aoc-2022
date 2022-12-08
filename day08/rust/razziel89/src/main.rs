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
// None yet.

// Count trees visible in a direction count_disp from all positions that can be reached from
// start_pos + n * outer_disp for all n that still yield a tree. If outer_disp is None, use only
// n==0. The search stops at the latest if no more trees can be found in that direction, assuming a
// dense forest.
//
// Whether a tree still counts as visible is defined by size_cmp. For part 1, it compares the
// current tree's size with the size of the largest tree found so far. For part 2, it always
// returns true because max_height has been set to that of the tree in consideration.
//
// A search in one direction stops early after a tree of max_height has been found because no trees
// behind it can be visible. The value for max_height differs between parts 1 and 2. For part 1,
// it's the global maximum and for part 2 it's the size of the tree in consideration.
fn count_visible(
    forest: &HashMap<(i64, i64), data::Tree>,
    start_pos: &data::Vec,
    count_disp: &data::Vec,
    outer_disp: Option<&data::Vec>,
    max_height: &u8,
    size_cmp: fn(&u8, &u8) -> bool,
) -> HashSet<(i64, i64)> {
    let mut visible_forest = HashSet::<(i64, i64)>::new();

    let mut outer_start = start_pos.clone();

    // This will automatically stop if we cannot retrieve any more trees. Assuming a dense forest,
    // that means once we reached the edge.
    while let Some(_) = forest.get(&outer_start.pos()) {
        let mut largest: u8 = 0;
        let mut pos = outer_start.clone();

        while &largest < max_height && let Some(tree) = forest.get(&pos.pos()) {
            let size = tree.size();
            // Remember the positions of trees that pass the size condition.
            if size_cmp(&size, &largest) {
                visible_forest.insert(pos.pos());
                largest = size;
            }
            pos = pos.add(&count_disp);
        }

        // If we want to search in the same direction from different starting positions, update the
        // starting position and go on searching. If not, end the outer loop early.
        if let Some(disp) = outer_disp {
            outer_start = outer_start.add(disp);
        } else {
            break;
        }
    }

    visible_forest
}

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let forest = io::parse_chars_to_data::<data::Tree>(file, "tree")?;

    // Part 1.
    // Get dimensions of forest in all three directions. That could have been avoided by using some
    // matrix structure but I wanted to use a HashMap here, so this is necessary.
    let max_x = forest
        .keys()
        .map(|el| el.0)
        .max()
        .ok_or(Error::msg("cannot find max x index"))?;
    let max_y = forest
        .keys()
        .map(|el| el.1)
        .max()
        .ok_or(Error::msg("cannot find max y index"))?;
    let max_height = forest
        .values()
        .map(|val| val.size())
        .max()
        .ok_or(Error::msg("cannot find max height"))?;

    // Compute union of all visible forests (or rather, tree positions).
    let mut count = HashSet::<(i64, i64)>::new();
    // Top border rightwards.
    count = &count
        | &count_visible(
            &forest,
            &data::Vec::new(0, 0),
            &data::Vec::new(0, 1),
            Some(&data::Vec::new(1, 0)),
            &max_height,
            |size, largest| size > largest,
        );
    // Left border downwards.
    count = &count
        | &count_visible(
            &forest,
            &data::Vec::new(0, 0),
            &data::Vec::new(1, 0),
            Some(&data::Vec::new(0, 1)),
            &max_height,
            |size, largest| size > largest,
        );
    // Bottom border rightwards.
    count = &count
        | &count_visible(
            &forest,
            &data::Vec::new(0, max_y),
            &data::Vec::new(0, -1),
            Some(&data::Vec::new(1, 0)),
            &max_height,
            |size, largest| size > largest,
        );
    // Right border downwards.
    count = &count
        | &count_visible(
            &forest,
            &data::Vec::new(max_x, 0),
            &data::Vec::new(-1, 0),
            Some(&data::Vec::new(0, 1)),
            &max_height,
            |size, largest| size > largest,
        );

    println!("visible are {} trees", count.len());

    // Part 2.
    let disps = vec![
        data::Vec::new(0, -1),
        data::Vec::new(-1, 0),
        data::Vec::new(0, 1),
        data::Vec::new(1, 0),
    ];

    let best_view = forest
        .iter()
        .map(|(pos, height)| {
            disps
                .iter()
                .map(|disp| {
                    let tree_pos = data::Vec::new(pos.0, pos.1);
                    count_visible(
                        &forest,
                        // Start searching at the first tree in the search direction from the
                        // starting position.
                        &tree_pos.add(&disp),
                        disp,
                        // Don't search along multiple parallel lines.
                        None,
                        &height.size(),
                        |_size, _largest| true,
                    )
                    .len()
                })
                // The scenic score for a tree is the product of the number of trees it can see in
                // every direction.
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
