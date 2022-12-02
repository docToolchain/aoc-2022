// Expected input file names.
const SAMPLE: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Error, Result};
use std::str::FromStr;
// Constants.
// None yet.

fn solve<T>(file: &str) -> Result<()>
where
    T: FromStr<Err = Error>,
    T: data::Round,
{
    eprintln!("PROCESSING {}", file);

    let mut scores = (0, 0);

    // Read file and convert into data.
    let rounds = io::parse_lines_to_data::<T>(file, "rounds")?;

    for round in rounds {
        let round_scores = round.score();
        scores = (scores.0 + round_scores.0, scores.1 + round_scores.1);
    }

    println!("scores are opponent: {}, you: {}\n", scores.0, scores.1);

    Ok(())
}

fn main() -> Result<()> {
    // Funny that the example for part 1 would end in a draw, but that's not mentioned anywhere.
    solve::<data::RoundPart1>(SAMPLE)?;
    solve::<data::RoundPart1>(REAL)?;

    solve::<data::RoundPart2>(SAMPLE)?;
    solve::<data::RoundPart2>(REAL)?;

    Ok(())
}
// end::main[]
