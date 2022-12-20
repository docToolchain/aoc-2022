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
// Constants.

fn solve(file: &str, mixes: usize, decryption_key: data::Size) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data. We use a custom struct here just so we can continue using
    // our parser function.
    let nums = io::parse_chunks_to_data::<data::Num>(
        io::read_lines_from_file(file, 1)?,
        "blueprint",
        None,
        None,
    )?;

    // Convert custom struct into vector of primitive types.
    let mut file = nums
        .iter()
        // For part 1, the decryption_key is 1.
        .map(|el| el.num() * decryption_key)
        // Remember the index in the original list. This is used to find the data point later on
        // based on its index in the original input.
        .enumerate()
        .collect::<Vec<_>>();

    // Three convenience values that will be used further down.
    // So that we always know how many values there were originally.
    let len = file.len();
    // After removing an element from the vector, this is gonna be its length. Thus, this is the
    // value with respect to which we need to take the modulus when deciding how many cycles to
    // skip.
    let mod_me = file.len() - 1;

    // We will be mixing `mixes` times.
    for _ in 0..mixes {
        // We will keep mixing in the originalorder.
        for org_idx in 0..len {
            // Find the element that was at `org_idx` in the initial input and remember its value
            // and current index.
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

            // Zero doesn't move so we skip it.
            if file[move_me_data.0].1 == 0 {
                continue;
            }

            // Remove the element from the vector. When imagining the cyclic list, it becomes clear
            // that a number will never encounter itself. Thus, it is the same as if we were
            // moving through an infinite vector that doesn't contain the number.
            let move_me = file.remove(move_me_data.0);
            // Below, we use a nice feature of Rust's iterators over vectors, namely that they can
            // be cyclic. Thus, we don't actually care whether our resulting vector looks as it
            // does in the example because it is cyclic anyway.
            if move_me.1 < 0 {
                // Move to the left. Here, we iterate through the vector backwards.
                let prev_elem = file
                    .iter()
                    // Reverse the iterator's direction here.
                    .rev()
                    // Turn it into an inifinte iterator. Thus, we won't have to care about
                    // wrap-arounds.
                    .cycle()
                    // Skip as many elements until we are at the location of the element we just
                    // removed. Doing so when moving backwards is a bit of a hassle but it works.
                    // Make this efficient by taking the modulus with respect to the current length
                    // of the vector.
                    .skip(file.len() - (move_me_data.0 % mod_me))
                    // Skip as often as we need to according to the value of the number. Take the
                    // mod again.
                    .skip((move_me.1.abs() as usize) % mod_me)
                    // Get the next element in the iterator, which is the element that is currently
                    // just before the position we want our removed element to occupy.
                    .next()
                    .ok_or(Error::msg("this should be infinite"))?;

                // Find the actual position of that element.
                let prev_elem_pos = file
                    .iter()
                    .enumerate()
                    .find_map(|(idx, el)| if el == prev_elem { Some(idx) } else { None })
                    .ok_or(Error::msg("cannot find previous element"))?;

                // Insert after the element we just found. Thanks, Rust, that `insert` also works
                // if the position where we want to insert is one past the end.
                file.insert((prev_elem_pos + 1).rem_euclid(file.len()), move_me);
            } else if move_me.1 > 0 {
                // Move to the right. Here, we iterate in forward direction.
                let next_elem = file
                    .iter()
                    // Turn it into an inifinte iterator. Thus, we won't have to care about
                    // wrap-arounds.
                    .cycle()
                    // Skip as many elements until we are at the location of the element we just
                    // removed.
                    // Make this efficient by taking the modulus with respect to the current length
                    // of the vector.
                    .skip(move_me_data.0 % mod_me)
                    // Skip as often as we need to according to the value of the number. Take the
                    // mod again.
                    .skip((move_me.1.abs() as usize) % mod_me)
                    // Get the next element, which is the element that is currently at the position
                    // we want our removed element to occupy.
                    .next()
                    .ok_or(Error::msg("this should be infinite"))?;

                // Find the actual position of that element.
                let next_elem_pos = file
                    .iter()
                    .enumerate()
                    .find_map(|(idx, el)| if el == next_elem { Some(idx) } else { None })
                    .ok_or(Error::msg("cannot find next element"))?;

                // Insert at the position of the element we just found.
                file.insert(next_elem_pos, move_me);
            } else {
                unreachable!("there are no more numbers");
            }
        }
    }

    // Extract the desired sum in a lazy way. Simply iterate 1, 2 and 3 thousand times over an ever
    // repeating instance of our iterator.
    //
    // Find the index of 0 first.
    let start_idx = file
        .iter()
        .enumerate()
        .find_map(|(idx, (_, num))| if num == &0 { Some(idx) } else { None })
        .ok_or(Error::msg("cannot find zero"))?;
    // Then take the sum.
    let grove_coords = file
        .into_iter()
        // Take apart so that we only keep the data. We no longer care about original indices.
        .map(|(_idx, el)| el)
        .collect::<Vec<_>>()
        .into_iter()
        // Infinite iterators again.
        .cycle()
        // Skip until we are at the location of 0.
        .skip(start_idx)
        // Find current indices.
        .enumerate()
        // Take oly every 1000'th element after zero. Note that 0 also passes here.
        .filter_map(|(idx, el)| if idx % 1000 == 0 { Some(el) } else { None })
        // We take 4 here because the first one fulfilling the condition will be zero.
        .take(4)
        // Skip zero.
        .skip(1)
        .sum::<data::Size>();

    println!(
        "solution for {} mix(es) and a key of {} is {}",
        mixes, decryption_key, grove_coords
    );

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1, 1, 1)?;
    solve(REAL, 1, 1)?;

    solve(SAMPLE1, 10, 811_589_153)?;
    solve(REAL, 10, 811_589_153)?;

    Ok(())
}
// end::main[]
