// Expected input file names.
#![feature(let_chains, variant_count)]
const SAMPLE1: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::Result;

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let snafus = io::parse_chunks_to_data::<data::Snafu>(
        io::read_lines_from_file(file, 1)?,
        "snafu",
        None,
        None,
    )?;

    for snafu in &snafus {
        println!("{} -> {}", snafu.dec(), snafu);
    }

    let sum = snafus
        .into_iter()
        .fold(data::Snafu::new(0), |acc, val| acc.add(&val));
    println!("sum is {} -> {}", sum.dec(), sum);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
