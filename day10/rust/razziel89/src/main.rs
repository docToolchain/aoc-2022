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

fn render(crt: &Vec<bool>, width: usize, fill: char) -> String {
    crt.iter()
        .enumerate()
        .map(|(idx, el)| {
            let ch = if *el { '#' } else { fill };
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

    // Used to avoid drawing past the edges. It turns out the register value is nice and those
    // checks would not have been needed.
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

// This function likely performs a lot of allocations that are not needed but it makes the rest of
// the problem so much easier to solve.
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

fn solve(file: &str, part1: bool, fill: char) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let ops = io::parse_lines_to_data::<data::Op>(file, "op", None, None)?;

    // We avoid that one-cycle-two-cycle weridness by replacing each addx operation by a noop and
    // an addx operation that is assumed to take only one cycle.
    let extended_ops = extend(ops);

    let mut reg = 1;
    let reg_vals = extended_ops.into_iter().map(move |el| {
        reg = match el {
            data::Op::None => reg,
            data::Op::Some(val) => {
                reg += val;
                reg
            }
        };
        reg
    });

    if part1 {
        // There is a lot of potential for off-by-one errors in this one.
        //
        // We skip the first 18 entries here because we want to start with the value during cycle
        // 20, which is the value after cycle 19, and this solution ignores what happens during the
        // first cycle, because that is trivial.
        let skip = 18;
        let mut skipper: isize = -1;
        let interesting = reg_vals
            .skip(skip)
            .enumerate()
            .filter_map(move |(step, reg)| {
                // Now convert from our weird way of counting to that of the puszzle.
                // We wanted to skip the first 19 cycles (add skip + 1) and cycle counting starts
                // at one (add 1).
                let current_cycle = step + 1 + skip + 1;
                skipper += 1;
                if skipper % 40 == 0 {
                    Some(reg * current_cycle as isize)
                } else {
                    None
                }
            })
            .sum::<isize>();

        println!("{:?}", interesting);
    } else {
        let mut crt = vec![false; WIDTH * 6];

        // We need to handle the first cycle separately because the cycle number used here is
        // always that of the cycle we are in. During the first cycle, the register has a value of
        // 1.
        maybe_draw(&mut crt, 1, 1, WIDTH);

        // Here, current_cycle is the cycle we are currently in. Thus, the number is one larger
        // than what the example shows because the example usually taks about the value after a
        // cycle but the value during a cycle is important. This also erroneously assumes the
        // existence 241'th cycle, but that's not really a problem because the value after the
        // 240'th cycle is the value during the 241th cycle, so it's consistent.
        for (current_cycle, reg) in reg_vals.enumerate().map(|(step, el)| (step + 2, el)) {
            maybe_draw(&mut crt, reg, current_cycle, WIDTH);
        }
        println!("\n{}\n", render(&crt, WIDTH, fill));
    }

    Ok(())
}

fn main() -> Result<()> {
    // Part 1.
    // The last argument is not important here.
    solve(SAMPLE1, true, '.')?;
    solve(SAMPLE2, true, '.')?;
    solve(REAL, true, '.')?;

    // Part 2.
    solve(SAMPLE2, false, '.')?;
    // Use a space as filler for the real solution to make the letters easier to read.
    solve(REAL, false, ' ')?;

    Ok(())
}
// end::main[]
