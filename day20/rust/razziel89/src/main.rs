// Expected input file names.
#![feature(let_chains, variant_count)]
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

fn solve(file: &str, mixes: usize, decryption_key: data::Size) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let nums = io::parse_chunks_to_data::<data::Num>(
        io::read_lines_from_file(file, 1)?,
        "blueprint",
        None,
        None,
    )?;

    let mut file = nums
        .iter()
        // Multiply by the decyption key.
        // .map(|el| {
        //     let possible = (el.num() * decryption_key) % (nums.len() as data::Size - 1);
        //     if el.num() != 0 && possible == 0 {
        //         nums.len() as data::Size
        //     } else {
        //         possible
        //     }
        // })
        .map(|el| el.num() * decryption_key)
        .enumerate()
        .collect::<Vec<_>>();
    let len = file.len();
    let mod_me = file.len();

    // println!("{:?}", file.iter().map(|el| el.1).collect::<Vec<_>>());

    for mix in 0..mixes {
        for org_idx in 0..len {
            let move_me_data = file
                .iter()
                .enumerate()
                .find_map(|(idx, el)| {
                    if el.0 == org_idx {
                        Some((idx, el.1))
                    } else {
                        None
                    }
                })
                .ok_or(Error::msg("cannot find element"))?;

            // Zero doesn't move.
            if file[move_me_data.0].1 == 0 {
                continue;
            }

            let move_me = file.remove(move_me_data.0);
            println!("{} {} {} {}", move_me.1, move_me_data.0, org_idx, mix);
            if move_me.1 < 0 {
                // Move to the left.
                let prev_elem = file
                    .iter()
                    // Reverse the iterator's direction here.
                    .rev()
                    .cycle()
                    .skip(file.len() - (move_me_data.0 % mod_me))
                    // Skip as often as we need to according to the value of the number. The -1 is here
                    // because we have basically already taken one step.
                    .skip((move_me.1.abs() as usize) % file.len())
                    .next()
                    .ok_or(Error::msg("this should be infinite"))?;

                let prev_elem_pos = file
                    .iter()
                    .enumerate()
                    .find_map(|(idx, el)| if el == prev_elem { Some(idx) } else { None })
                    .ok_or(Error::msg("cannot find previous element"))?;

                file.insert((prev_elem_pos + 1).rem_euclid(file.len()), move_me);
            } else if move_me.1 > 0 {
                // Move to the right.
                let next_elem = file
                    .iter()
                    .cycle()
                    // Skip until we are at the element that was located behind the one we removed.
                    .skip(move_me_data.0 % mod_me)
                    // Skip as often as we need to according to the value of the number. The -1 is here
                    // because we have basically already taken one step.
                    .skip((move_me.1.abs() as usize) % file.len())
                    .next()
                    .ok_or(Error::msg("this should be infinite"))?;

                let next_elem_pos = file
                    .iter()
                    .enumerate()
                    .find_map(|(idx, el)| if el == next_elem { Some(idx) } else { None })
                    .ok_or(Error::msg("cannot find next element"))?;

                file.insert(next_elem_pos, move_me);
            } else {
                unreachable!("there are no more numbers");
            }
        }
    }

    // Extract the desired sum in a lazy way. Simply iterate 1, 2 and 3 thousand times over an ever
    // repeating instance of our iterator.
    let start_idx = file
        .iter()
        .enumerate()
        .find_map(|(idx, (_, num))| if num == &0 { Some(idx) } else { None })
        .ok_or(Error::msg("cannot find zero"))?;
    let grove_coords = file
        .into_iter()
        .map(|(idx, _el)| idx)
        .map(|el| nums[el].num() * decryption_key)
        // .map(|el| nums[el].num() * 1)
        .collect::<Vec<_>>()
        .into_iter()
        .cycle()
        .skip(start_idx)
        .enumerate()
        .filter_map(|(idx, el)| {
            if idx % 1000 == 0 {
                println!("{}", el);
                Some(el)
            } else {
                None
            }
        })
        .take(4)
        .skip(1)
        .sum::<data::Size>();

    println!("solution is: {}\n", grove_coords);

    Ok(())
}

fn main() -> Result<()> {
    // solve(SAMPLE1, 1, 1)?;
    // solve(REAL, 1, 1)?;

    solve(SAMPLE1, 10, 811_589_153)?;
    solve(REAL, 10, 811_589_153)?;

    Ok(())
}
// end::main[]
