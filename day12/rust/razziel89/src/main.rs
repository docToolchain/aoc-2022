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
use std::collections::{HashMap, HashSet};
// Constants.

fn build_graph(map: &HashMap<data::Point, data::Height>) -> HashSet<data::Node> {
    map.iter()
        .map(|(&p, &h)| {
            let neighbours = p
                .env()
                .into_iter()
                .filter_map(|el| match map.get(&el) {
                    Some(other_h) => {
                        if other_h.height() <= h.height() + 1 {
                            Some(p)
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect::<HashSet<_>>();
            data::Node::new(p, h, neighbours)
        })
        .collect::<HashSet<data::Node>>()
}

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let height_map = io::parse_chars_to_data::<data::Height>(file, "vec")?;

    // println!("{:?}", height_map);

    let graph = build_graph(&height_map);

    println!("{:?}", graph);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    // solve(REAL)?;

    // solve(SAMPLE1)?;
    // solve(REAL)?;
    Ok(())
}
// end::main[]
