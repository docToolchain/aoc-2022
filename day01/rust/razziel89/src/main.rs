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
const NUM_ELVES: usize = 3;

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let baggage = io::parse_lines_to_data::<data::Baggage>(file, "baggage")?;

    let mut elves = data::baggages_to_elves(baggage);

    // Elf carrying the most calories will be first in line. It is inefficient to calculate total
    // calories for every comparison, but it's not really important for this exercise.
    elves.sort_by(|el1, el2| el2.total_calories().cmp(&el1.total_calories()));

    match elves.len() {
        // Even though we could solve part 1 if we had 1..=2 elves, we ignore that case here.
        0..=2 => Err(Error::msg("somehow, we found too few elves :(")),
        _ => {
            // Part 1.
            println!(
                "elf carrying the most is num {} who carries {} calories",
                elves[0].get_idx(),
                elves[0].total_calories()
            );

            // Part 2.
            let total_calories: u64 = elves
                .iter()
                .take(NUM_ELVES)
                .map(|el| el.total_calories())
                .sum();

            println!(
                "the {} elves carrying the most carry {} in total\n",
                NUM_ELVES, total_calories
            );

            Ok(())
        }
    }
}
// end::main[]

fn main() -> Result<()> {
    solve(SAMPLE)?;
    solve(REAL)?;

    Ok(())
}
