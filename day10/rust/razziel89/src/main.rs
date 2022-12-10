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
use anyhow::Result;
// Constants.
// None yet.

fn render(crt: &Vec<bool>, width: usize) -> String {
    crt.iter()
        .enumerate()
        .map(|(idx, el)| {
            let ch = if *el { '#' } else { '.' };
            if (idx + 1) % width == 0 {
                format!("{}\n", ch)
            } else {
                ch.to_string()
            }
        })
        .collect::<String>()
}

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

    let mut crt = vec![false; 40 * 6];
    println!("{}", render(&crt, 40));

    // Read file and convert into data.
    let ops = io::parse_lines_to_data::<data::Op>(file, "op", None, None)?;

    // println!("{:?}", ops);

    let extended_ops = extend(ops);

    // println!("{:?}", extended_ops);

    let mut reg = 1;
    let reg_vals = extended_ops
        .into_iter()
        .enumerate()
        .map(move |(_step, el)| {
            reg = match el {
                data::Op::None => reg,
                data::Op::Some(val) => {
                    reg += val;
                    reg
                }
            };
            // eprintln!("all {} {}", reg, step + 1);
            reg
        });

    let skip = 18;
    let mut skipper: isize = -1;
    let interesting = reg_vals
        .skip(skip)
        .enumerate()
        .filter_map(move |(step, el)| {
            let current_cycle = step + 1 + skip + 1;
            skipper += 1;
            if skipper % 40 == 0 {
                eprintln!("int {} {}", el, current_cycle);
                Some(el * current_cycle as isize)
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
