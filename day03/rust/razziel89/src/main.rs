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
// Constants.
// None yet.

fn ord(c: char) -> Result<usize> {
    match c {
        'a'..='z' => Ok((c as usize - 'a' as usize) + 1),
        'A'..='Z' => Ok((c as usize - 'A' as usize) + 27),
        _ => Err(Error::msg(format!(
            "invalid character {} for ord conversion",
            c,
        ))),
    }
}

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let rucksacks = io::parse_lines_to_data::<data::Rucksack>(file, "rucksack")?;

    // Part 1.
    let common = rucksacks
        .iter()
        .map(|el| (&el.left & &el.right).into_iter().collect::<Vec<_>>())
        .enumerate()
        .map(|(idx, el)| {
            if el.len() == 1 {
                ord(el[0])
            } else {
                Err(Error::msg(format!(
                    "entry {} has wrong length {}",
                    idx,
                    el.len()
                )))
            }
        })
        .collect::<Vec<_>>();

    {
        let mut has_err = false;
        for c in common.as_slice() {
            if let Err(err) = c {
                eprintln!("{:?}", err);
                has_err = true;
            }
        }
        if has_err {
            return Err(Error::msg("encountered at least one error"));
        }
    }

    println!(
        "part 1, total value is {}",
        common.iter().flatten().sum::<usize>()
    );

    // Part 2.
    let badges = rucksacks
        .iter()
        .map(|el| el.everything())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .into_iter()
        .map(|sets| {
            if let [set1, set2, set3] = sets {
                (&(set1 & set2) & set3).into_iter().collect::<Vec<_>>()
            } else {
                panic!("this will never happen due to the use of exact_chunk")
            }
        })
        .enumerate()
        .map(|(idx, el)| {
            if el.len() == 1 {
                ord(el[0])
            } else {
                Err(Error::msg(format!(
                    "entry {} has wrong length {}",
                    idx,
                    el.len()
                )))
            }
        })
        .collect::<Vec<_>>();

    println!(
        "part 2, total value is {}",
        badges.iter().flatten().sum::<usize>()
    );

    Ok(())
}

fn main() -> Result<()> {
    // Funny that the example for part 1 would end in a draw, but that's not mentioned anywhere.
    solve(SAMPLE)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
