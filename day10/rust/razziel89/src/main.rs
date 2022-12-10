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
// Constants.
const WIDTH: usize = 40;

fn render(crt: &Vec<bool>, width: usize) -> String {
    crt.iter()
        .enumerate()
        .map(|(idx, el)| {
            let ch = if *el { '#' } else { ' ' };
            if (idx + 1) % width == 0 {
                format!("{}\n", ch)
            } else {
                ch.to_string()
            }
        })
        .collect::<String>()
}

fn maybe_draw(crt: &mut Vec<bool>, reg: isize, cycle: usize, width: usize) {
    let pixel_idx = cycle - 1;
    let horizontal_pos = (pixel_idx % width) as isize;

    let sprite_at_left_edge = reg == 0;
    let sprite_at_right_edge = reg == width as isize - 1;

    if reg == horizontal_pos {
        crt[pixel_idx] = true;
    } else if reg + 1 == horizontal_pos && !sprite_at_right_edge {
        crt[pixel_idx] = true;
    } else if reg - 1 == horizontal_pos && !sprite_at_left_edge {
        crt[pixel_idx] = true;
    }
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

fn solve(file: &str, part1: bool) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let ops = io::parse_lines_to_data::<data::Op>(file, "op", None, None)?;

    let extended_ops = extend(ops);

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

    if part1 {
        let skip = 18;
        let mut skipper: isize = -1;
        let interesting = reg_vals
            .skip(skip)
            .enumerate()
            .filter_map(move |(step, reg)| {
                let current_cycle = step + 1 + skip + 1;
                skipper += 1;
                if skipper % 40 == 0 {
                    eprintln!("int {} {}", reg, current_cycle);
                    Some(reg * current_cycle as isize)
                } else {
                    None
                }
            })
            .sum::<isize>();

        println!("{:?}", interesting);
    } else {
        let mut crt = vec![false; WIDTH * 6];
        println!("{}", render(&crt, WIDTH));

        // We need to handle the first cycle separately. During htat cycle, the register has a
        // value of 1.
        maybe_draw(&mut crt, 1, 1, WIDTH);
        println!("{}", render(&crt, WIDTH));

        // Here, current_cycle is the cycle we are currently in. Thus, the number is one larger
        // than what the example shows. This also erroneously assumes a 241'th cycle, but that's
        // not really a problem.
        for (current_cycle, reg) in reg_vals.enumerate().map(|(step, el)| (step + 2, el)) {
            eprintln!("int {} {}", reg, current_cycle);
            maybe_draw(&mut crt, reg, current_cycle, WIDTH);
            println!("{}", render(&crt, WIDTH));
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    // Part 1.
    solve(SAMPLE1, true)?;
    solve(SAMPLE2, true)?;
    solve(REAL, true)?;

    // Part 2.
    solve(SAMPLE2, false)?;
    solve(REAL, false)?;

    Ok(())
}
// end::main[]
