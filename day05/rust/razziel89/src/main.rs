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
            let mut stack: data::Stack = vec![];

            // We reverse the iterator because we obtained the lines from top to bottom but we need
            // to build the stacks from the ground up. Thus, we iterate from the ground to the
            // bottom.
            for line in lines.iter().rev() {
                // We cannot be sure that every stack line contains the same number of entries.
                // Thus, we use the ".get" method to be able to catch the case where one line ends
                // before another. It turns out that every line has the same number of entries,
                // making this safeguard unnecessary...
                if let Some(Some(elem)) = line.stacks.get(stack_idx) {
                    stack.push(*elem);
                }
            }

            stacks.push(stack);
        }

        Ok(stacks)
    } else {
        Err(Error::msg("only empty stacks obtained"))
    }
}

fn check_bounds(idx: usize, len: usize, name: &str) -> Result<()> {
    if idx > len {
        return Err(Error::msg(format!(
            "{} stack {} is out of bounds",
            name, idx
        )));
    } else {
        Ok(())
    }
}

fn apply_move_part1(stacks: &mut Vec<data::Stack>, mov: &data::Move) -> Result<()> {
    // Thanks to these two bounds checks, we know that the index operations below will never panic.
    check_bounds(mov.src, stacks.len(), "source")?;
    check_bounds(mov.dest, stacks.len(), "dest")?;

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
    // Thanks to these two bounds checks, we know that the index operations below will never panic.
    check_bounds(mov.src, stacks.len(), "source")?;
    check_bounds(mov.dest, stacks.len(), "dest")?;

    // We are being lazy and are using a temporary stack to stash the crates away. That way, we
    // keep the order intact when putting them back from the temporary stash to the final stash.
    let mut temp_stack: data::Stack = vec![];

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
    )?;

    let mut stacks = lines_to_stacks(&stack_lines)?;

    for mov in &moves {
        apply_move(&mut stacks, mov)?;
    }

    let mut errs = vec![];

    println!(
        "the top elements are: {}",
        stacks
            .iter()
            .enumerate()
            .map(|(idx, el)| el
                .last()
                .map(|el| el.to_string())
                .ok_or(Error::msg(format!("stack {} is empty", idx))))
            .filter_map(|el| io::filter_and_remember_errs(el, &mut errs))
            .collect::<Vec<_>>()
            .join("")
    );

    io::process_remembered_errs(errs)
}

fn main() -> Result<()> {
    solve(SAMPLE, apply_move_part1)?;
    solve(REAL, apply_move_part1)?;

    solve(SAMPLE, apply_move_part2)?;
    solve(REAL, apply_move_part2)?;

    Ok(())
}
// end::main[]
