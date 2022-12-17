// Expected input file names.
#![feature(let_chains, variant_count)]
const SAMPLE1: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Error, Result};
use std::collections::HashSet;
use std::iter::Cycle;
use std::vec::IntoIter;
// Constants.

fn render(field: &HashSet<data::Pos>, rock: &data::Rock, pos: &data::Pos) {
    let max_x = 8;
    let max_y = field.iter().map(|el| el.y).max().unwrap_or(10) + 10;
    let min_x = 0;
    let min_y = 0;

    let rock_fields = rock.occupied_fields(pos).collect::<HashSet<_>>();

    for y in (min_y..max_y).rev() {
        for x in min_x..=max_x {
            let pos = data::Pos { x, y };
            if field.contains(&pos) && rock_fields.contains(&pos) {
                // This a conflict field.
                print!("X")
            } else if field.contains(&pos) {
                print!("#")
            } else if rock_fields.contains(&pos) {
                print!("@")
            } else if y == 0 {
                print!("-")
            } else if x == 0 || x == 8 {
                print!("|")
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    std::io::stdin().lines().next();
}

fn is_blocked(field: &HashSet<data::Pos>, check: &data::Pos, width: isize) -> data::Blocked {
    if check.x <= 0 || check.x >= width {
        data::Blocked::Wall
    } else if field.contains(check) {
        data::Blocked::Rock
    } else if check.y == 0 {
        // This is hacky but we simulate a floor of rocks so that a downward operation will have
        // the rock settle there.
        data::Blocked::Rock
    } else {
        data::Blocked::None
    }
}

// Yeah, we are playing tetris today.
fn play_tetris(
    mut stream: Cycle<IntoIter<data::Push>>,
    mut rocks: Cycle<IntoIter<data::Rock>>,
    max_num_rocks: usize,
    rep_rounds: usize,
) -> usize {
    // The field is 7 spots wide. Thus, if one wall is at 0, the other is at 8.
    let mut field = HashSet::<data::Pos>::new();
    // At the beginning, there is no rock yet. Thus, the top position is the floor.
    let mut top_rock = 0;
    let mut round = 0;
    while round < max_num_rocks {
        // the zip iterator interrupts as soon as the first one runs out.
        for _ in (0..rep_rounds).zip(round..max_num_rocks) {
            round += 1;
            let rock = rocks.next().expect("weren't there infinitely many rocks");
            // println!("{}/{} {}", round, max_num_rocks, top_rock);
            // Spwan a rock.
            let mut pos = data::Pos {
                // There also have to be 2 free spaces in x direction. Thus, it spawns at x
                // coordinate 3.
                x: 3,
                // There have to be 3 free spaces in y direction. Thus, it spwans 4 units
                // further above.
                y: top_rock + 4,
            };
            // render(&field, &rock, &pos);
            let mut has_settled = false;
            while !has_settled {
                // Get the next stream element. This is an infinite iterator.
                let push = stream.next().expect("wasn't this supposed to be infinite?");

                // Apply the movement operation to the side.
                let next_pushed = push.apply(&pos);
                // render(&field, &rock, &next_pushed);
                // Check whether there is any collision.
                let push_blocked_check = rock
                    .occupied_fields(&next_pushed)
                    .map(|el| is_blocked(&field, &el, 8))
                    .find(|el| el != &data::Blocked::None);
                // Accept the movement only if we haven't been blocked.
                if let None = push_blocked_check {
                    pos = next_pushed;
                }

                // Move downwards and check again.
                let next_dropped = pos.drop();
                // render(&field, &rock, &next_dropped);
                // Check whether there is any collision.
                let drop_blocked_check = rock
                    .occupied_fields(&next_dropped)
                    .map(|el| is_blocked(&field, &el, 8))
                    .find(|el| el == &data::Blocked::Rock);
                // Accept the movement only if we haven't been blocked.
                if let Some(_) = drop_blocked_check {
                    // The rock has settled!
                    has_settled = true;
                    // Find the topmost position and occupy all fields of the rock.
                    field.extend(rock.occupied_fields(&pos));
                    let possible_top_rock = rock
                        .occupied_fields(&pos)
                        .map(|el| el.y)
                        .max()
                        .expect("cannot find top rock");
                    if possible_top_rock > top_rock {
                        top_rock = possible_top_rock;
                    }
                    // // If any rock in the sequence settles in such a way that we arrive at a state that
                    // // all the next rocks will fall as they did before, we have found a stopping point.
                    // // That point is definitely reached if the last rock in the sequence causes there
                    // // to be a fully flat surface.
                    // if (1..=7)
                    //     .into_iter()
                    //     .all(|el| field.contains(&data::Pos { x: el, y: top_rock }))
                    // {
                    //     return top_rock as usize;
                    // }
                } else {
                    // The rock hasn't settled yet. Accept the update.
                    pos = next_dropped;
                }
            }
        }
    }

    top_rock as usize
}

fn solve(file: &str, max_num_rocks: usize) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let stream = io::parse_chunks_to_data::<data::Stream>(
        io::read_lines_from_file(file, 1)?,
        "stream",
        None,
        None,
    )?
    .into_iter()
    .nth(0)
    .ok_or(Error::msg("found no stream"))?;

    let num_pushes = stream.flow.len();
    println!("push sequence has {} elements", num_pushes);
    let num_rocks = std::mem::variant_count::<data::Rock>();
    println!("rock sequence has {} elements", num_rocks);
    let rep_rounds = num_pushes * num_rocks;
    println!("possible repetition after {} rounds", rep_rounds);

    let tallness = play_tetris(
        stream.infinite(),
        data::Rock::infinite_stream(),
        max_num_rocks,
        rep_rounds,
    );
    println!("{}", tallness);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1, 2022)?;
    solve(REAL, 2022)?;

    // solve(SAMPLE1, 1_000_000_000_000)?;
    // solve(REAL, 1_000_000_000_000)?;

    Ok(())
}
// end::main[]
