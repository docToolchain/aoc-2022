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

fn lines_to_stacks(lines: &Vec<data::StackLine>) -> Result<Vec<data::Stack>> {
    if let Some(num_stacks) = lines.iter().map(|el| el.stacks.len()).max() {
        let mut stacks = vec![];

        for stack_idx in 0..num_stacks {
            let mut stack = data::Stack { data: vec![] };

            for line in lines.iter() {
                if let Some(Some(elem)) = line.stacks.get(stack_idx) {
                    stack.push(*elem);
                }
            }

            stacks.push(stack);
        }

        Ok(stacks)
    } else {
        Err(Error::msg("no stack lines obtaibed"))
    }
}

fn apply_move_part1(stacks: &mut Vec<data::Stack>, mov: &data::Move) -> Result<()> {
    for _ in 0..mov.num {
        if let Some(moved_elem) = &stacks[mov.src].pop() {
            stacks[mov.dest].push(*moved_elem);
        } else {
            return Err(Error::msg(format!("cannot apply move {:?}", mov)));
        }
    }
    Ok(())
}

fn apply_move_part2(stacks: &mut Vec<data::Stack>, mov: &data::Move) -> Result<()> {
    // We are being lazy and are using a temporary stack to stash the crates away. That way, we
    // keep the order intact.
    let mut temp_stack = data::Stack { data: vec![] };

    for _ in 0..mov.num {
        if let Some(moved_elem) = &stacks[mov.src].pop() {
            temp_stack.push(*moved_elem);
        } else {
            return Err(Error::msg(format!(
                "cannot apply 1st half of move {:?}",
                mov
            )));
        }
    }

    for _ in 0..mov.num {
        if let Some(moved_elem) = &temp_stack.pop() {
            stacks[mov.dest].push(*moved_elem);
        } else {
            return Err(Error::msg(format!(
                "cannot apply 2nd half of move {:?}",
                mov
            )));
        }
    }
    Ok(())
}

fn solve(
    file: &str,
    apply_move: fn(&mut Vec<data::Stack>, &data::Move) -> Result<()>,
) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let moves =
        io::parse_lines_to_data::<data::Move>(file, "move", Some(|el| el.contains("move")), None)?;

    let stack_lines = io::parse_lines_to_data::<data::StackLine>(
        file,
        "stack line",
        Some(|el| el.contains("[")),
        None,
    )?
    .into_iter()
    .rev()
    .collect::<Vec<_>>();

    let mut stacks = lines_to_stacks(&stack_lines)?;

    for stack in &stacks {
        println!("{:?}", stack);
    }

    for mov in &moves {
        apply_move(&mut stacks, mov)?;
    }

    println!(
        "the top elements are: {}",
        stacks
            .iter()
            .map(|el| el
                .data
                .last()
                .expect("none of the stacks should be empty")
                .to_string())
            .collect::<Vec<_>>()
            .join("")
    );

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE, apply_move_part1)?;
    solve(REAL, apply_move_part1)?;

    solve(SAMPLE, apply_move_part2)?;
    solve(REAL, apply_move_part2)?;

    Ok(())
}
// end::main[]
