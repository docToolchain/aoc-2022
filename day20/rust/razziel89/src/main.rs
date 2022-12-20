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

fn solve(file: &str, part1: bool) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let nums = io::parse_chunks_to_data::<data::Num>(
        io::read_lines_from_file(file, 1)?,
        "blueprint",
        None,
        None,
    )?;

    // // We use something indexed by tye type isize to avoid back and forth type casting.
    // let mut file = nums
    //     .iter()
    //     .enumerate()
    //     .map(|(idx, el)| (idx as isize, el.num()))
    //     .collect::<HashMap<isize, data::Size>>();
    // let num_nums = file.len() as isize;
    // let mut org_curr_map = file
    //     .iter()
    //     .map(|(idx, _num)| (idx.clone(), idx.clone()))
    //     .collect::<HashMap<isize, isize>>();
    // let mut curr_org_map = org_curr_map
    //     .iter()
    //     .map(|el| (el.0.clone(), el.1.clone()))
    //     .collect::<HashMap<_, _>>();
    // println!(
    //     "{:?}",
    //     (0..num_nums).map(|el| file[&el]).collect::<Vec<_>>()
    // );
    //
    // for org_idx in 0..file.len() {
    //     let move_num = file[&org_curr_map[&(org_idx as isize)]];
    //     let disp: isize = if move_num < 0 { -1 } else { 1 };
    //
    //     for _ in 0..move_num.abs() {
    //         // println!(
    //         //     "IN {:?}",
    //         //     (0..num_nums).map(|el| file[&el]).collect::<Vec<_>>()
    //         // );
    //         let move_idx = org_curr_map[&(org_idx as isize)];
    //         let target_curr_idx = (move_idx + disp).rem_euclid(num_nums);
    //         if target_curr_idx > 0 && target_curr_idx < num_nums - 1 {
    //             // Normal move inside the file.
    //             let target_org_idx = curr_org_map[&target_curr_idx];
    //
    //             let num_at_target = file[&target_curr_idx];
    //             // println!("{} {} {}", target_curr_idx, target_org_idx, num_at_target);
    //             // println!("{} {} {}", move_idx, org_idx, move_num);
    //             // Swap the numbers.
    //             file.insert(target_curr_idx, move_num);
    //             file.insert(move_idx, num_at_target);
    //             // Update the index maps.
    //             // Target number.
    //             org_curr_map.insert(target_org_idx, move_idx);
    //             curr_org_map.insert(move_idx, target_org_idx);
    //             // Moved number.
    //             org_curr_map.insert(org_idx as isize, target_curr_idx);
    //             curr_org_map.insert(target_curr_idx, org_idx as isize);
    //         } else if target_curr_idx == 0 {
    //             // Wrap around from right to left.
    //         } else if target_curr_idx == num_nums - 1 {
    //             // Wrap around from left to right.
    //         } else {
    //             unreachable!("there are no other cases");
    //         }
    //     }
    //     println!(
    //         "{:?}",
    //         (0..num_nums).map(|el| file[&el]).collect::<Vec<_>>()
    //     );
    // }

    let mut file = nums
        .iter()
        .map(|el| el.num())
        .enumerate()
        .collect::<Vec<_>>();
    let len = file.len();

    println!("{:?}", file.iter().map(|el| el.1).collect::<Vec<_>>());

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
        if move_me.1 < 0 {
            // Move to the left.
            let prev_elem = file
                .iter()
                // Reverse the iterator's direction here.
                .rev()
                .cycle()
                .skip(file.len() - move_me_data.0)
                // Skip as often as we need to according to the value of the number. The -1 is here
                // because we have basically already taken one step.
                .skip(move_me.1.abs() as usize)
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
                .skip(move_me_data.0)
                // Skip as often as we need to according to the value of the number. The -1 is here
                // because we have basically already taken one step.
                .skip(move_me.1.abs() as usize)
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

        println!("{}", move_me.1);
        if file.len() < 100 {
            println!("{:?}", file.iter().map(|el| el.1).collect::<Vec<_>>());
        }
    }

    // println!(
    //     "final: {:?}",
    //     file.iter().map(|el| el.1).collect::<Vec<_>>()
    // );

    // // Extract the desired sum in a lazy way. Simply iterate 1, 2 and 3 thousand times over an ever
    // // repeating instance of our iterator.
    // let grove_coords = file
    //     .into_iter()
    //     .map(|(_idx, el)| el.clone())
    //     .collect::<Vec<_>>()
    //     .into_iter()
    //     .cycle()
    //     .enumerate()
    //     .filter_map(|(idx, el)| {
    //         if (idx + 1) % 1000 == 0 {
    //             Some(el)
    //         } else {
    //             None
    //         }
    //     })
    //     .take(3)
    // //     .sum::<data::Size>();
    //
    // println!("{}", grove_coords);

    // Extract the desired sum in a lazy way. Simply iterate 1, 2 and 3 thousand times over an ever
    // repeating instance of our iterator.
    let start_idx = file
        .iter()
        .enumerate()
        .find_map(|(idx, (_, num))| if num == &0 { Some(idx) } else { None })
        .ok_or(Error::msg("cannot find zero"))?;
    let grove_coords = file
        .into_iter()
        .map(|(_idx, el)| el.clone())
        .collect::<Vec<_>>()
        .into_iter()
        .cycle()
        .skip(start_idx)
        .enumerate()
        .filter_map(|(idx, el)| if idx % 1000 == 0 { Some(el) } else { None })
        .take(4)
        .skip(1)
        .sum::<data::Size>();

    println!("\n{}\n", grove_coords);

    Ok(())
}

fn main() -> Result<()> {
    solve(REAL, true)?;
    solve(SAMPLE1, true)?;

    Ok(())
}
// end::main[]
