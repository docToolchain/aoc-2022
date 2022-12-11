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
// Constants.

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let mut monkeys = io::parse_chunks_to_data::<data::Monkey>(
        io::read_lines_from_file(file, 7)?,
        "monkey",
        None,
        None,
    )?;

    for _round in 0..20 {
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
    solve(SAMPLE1)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
