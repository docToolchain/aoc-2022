#![feature(let_chains)]
// Expected input file names.
const SAMPLE1: &str = "sample.dat";
const SAMPLE2: &str = "sample2.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Error, Result};
use std::collections::HashSet;
// Constants.
// None yet.

// Define some helper functions that allow easy conversions from an Option<data::Vec> to a
// Result<data::Vec> because the latter lets us use the question mark operator for unobstrusive
// error forwarding.
fn tail(rope: &Vec<data::Vec>) -> Result<data::Vec> {
    rope.last()
        .map(|el| el.clone())
        .ok_or(Error::msg("cannot get tail"))
}

// Yeah, playing with lifetimes.
fn get_mut<'a>(rope: &'a mut Vec<data::Vec>, idx: usize) -> Result<&'a mut data::Vec> {
    rope.get_mut(idx)
        .ok_or(Error::msg("cannot get element mutably"))
}

fn get<'a>(rope: &'a Vec<data::Vec>, idx: usize) -> Result<&'a data::Vec> {
    rope.get(idx).ok_or(Error::msg("cannot get element"))
}

fn solve(file: &str, length: usize) -> Result<()> {
    eprintln!("PROCESSING {} WITH LENGTH {}", file, length);

    // Read file and convert into data.
    let updates = io::parse_lines_to_data::<data::Vec>(file, "vec", None, None)?;

    // All knots start at the very same position.
    let mut rope = vec![data::NULL_VEC; length + 2];
    let mut visited_by_tail = HashSet::<data::Vec>::new();

    visited_by_tail.insert(tail(&rope)?);

    // Part 1.
    // The `iter` method for a vector provides an iterator over a set of unit-size steps that, if
    // followed, will ensure that we have traveled the entire distance described by the vector.
    for update in updates.iter().map(|el| el.iter()).flatten() {
        // The mv method will make sure we never move the head farther than one space. This is just
        // a safeguard for errors in the code.
        get_mut(&mut rope, 0)?.mv(&update)?;
        // Remember only the position of the reference vector, which is the head so far. That
        // involves a clone.
        let mut ref_vec = *get(&rope, 0)?;

        // Update all others with reference to their previous entry.
        for knot in rope.iter_mut() {
            // Move the knot with respect to the reference knot, which is always the previous one.
            *knot = knot.add(&ref_vec.get_tail_update(&knot));
            // Update the reference knot's position. Because we can only ever borrow one element as
            // mutable in rust and once we did so, we cannot borrow anything else, we clone it
            // here. That's inefficient but I couldn't find an easy way around it without resorting
            // to `unsafe`, which I want to avoid. Sadly, it seems as if `split_at_mut` involves
            // `unsafe`.
            ref_vec = *knot;
        }
        visited_by_tail.insert(tail(&rope)?);
    }

    println!(
        "the tail visited {} unique spots for a rope of length {}",
        visited_by_tail.len(),
        rope.len(),
    );

    Ok(())
}

fn main() -> Result<()> {
    // Part 1.
    solve(SAMPLE1, 0)?;
    solve(SAMPLE2, 0)?;
    solve(REAL, 0)?;

    // Part 2.
    solve(SAMPLE1, 8)?;
    solve(SAMPLE2, 8)?;
    solve(REAL, 8)?;

    Ok(())
}
// end::main[]
