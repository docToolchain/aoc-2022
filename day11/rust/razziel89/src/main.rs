#![feature(let_chains)]
// Expected input file names.
const SAMPLE1: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::Result;
use std::collections::HashSet;
// Constants.

fn solve(file: &str, part1: bool) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let mut monkeys = io::parse_chunks_to_data::<data::Monkey>(
        io::read_lines_from_file(file, 7)?,
        "monkey",
        None,
        None,
    )?;

    let prod_of_div_vals = monkeys
        .iter()
        .map(|el| el.get_div())
        .collect::<HashSet<_>>()
        .into_iter()
        .product::<isize>();

    if !part1 {
        for monkey in &mut monkeys {
            monkey.set_all_divs(prod_of_div_vals);
        }
    }

    let rounds = if part1 { 20 } else { 10_000 };

    for _round in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            // println!("check: {:?}", monkeys[monkey_idx]);
            for (target, item) in monkeys[monkey_idx].inspect_and_toss().into_iter() {
                monkeys[target].catch(item);
                // println!("toss:  {:?} <- {}", monkeys[target], item);
            }
            // println!("\n")
        }
    }

    monkeys.sort_by(|monkey1, monkey2| monkey1.how_active().cmp(&monkey2.how_active()));
    monkeys.reverse();

    println!("{}", monkeys[0].how_active() * monkeys[1].how_active(),);

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
