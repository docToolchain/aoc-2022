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

fn extend(input: Vec<data::Op>) -> Vec<data::Op> {
    input
        .into_iter()
        .map(|el| match el {
            data::Op::None => vec![data::Op::None].into_iter(),
            data::Op::Some(_) => vec![data::Op::None, el].into_iter(),
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let ops = io::parse_lines_to_data::<data::Op>(file, "op", None, None)?;

    println!("{:?}", ops);

    let extended_ops = extend(ops);

    println!("{:?}", extended_ops);

    let mut step = 0;
    let mut reg = 1;
    let reg_vals = extended_ops.into_iter().map(move |el| {
        reg = match el {
            data::Op::None => reg,
            data::Op::Some(val) => {
                reg += val;
                reg
            }
        };
        step += 1;
        // eprintln!("all {} {}", reg, step);
        reg
    });

    let skip: isize = 18;
    let mut step: isize = 0;
    let mut skipper: isize = -1;
    let interesting = reg_vals
        .skip(skip as usize)
        .filter_map(move |el| {
            skipper += 1;
            step += 1;
            if skipper % 40 == 0 {
                eprintln!("int {} {}", el, step + skip);
                Some(el * (step + skip + 1))
            } else {
                None
            }
        })
        .sum::<isize>();

    println!("{:?}", interesting);

    Ok(())
}

fn main() -> Result<()> {
    // Part 1.
    solve(SAMPLE1)?;
    solve(SAMPLE2)?;
    solve(REAL)?;

    // Part 2.
    // solve(SAMPLE1)?;
    // solve(SAMPLE2)?;
    // solve(REAL)?;

    Ok(())
}
// end::main[]
