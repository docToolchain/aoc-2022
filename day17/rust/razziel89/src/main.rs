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
    print!("\n");
    // std::io::stdin().lines().next();
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

fn is_nice_ground(ground: &HashSet<data::Pos>, field: &HashSet<data::Pos>) -> bool {
    let mut x = 1;
    let mut candidates = ground
        .iter()
        .filter_map(|el| if el.x == x { Some(el.clone()) } else { None })
        .collect::<HashSet<data::Pos>>();
    candidates = &candidates & field;

    while candidates.len() != 0 {
        x += 1;
        candidates = candidates
            .into_iter()
            .map(|el| el.right_env())
            .flatten()
            .collect::<HashSet<data::Pos>>();
        candidates = &candidates & field;
    }

    x == 8
}

fn get_ground(field: &HashSet<data::Pos>, top_rock: isize) -> Option<HashSet<data::Pos>> {
    let mut found = vec![false; 7];
    let mut bottom = HashSet::<data::Pos>::new();
    let mut min_y = top_rock;

    for y in (1..=top_rock).rev() {
        min_y = y;
        for x in 1..=7 {
            let check = data::Pos { x, y };
            if field.contains(&check) {
                found[(x - 1) as usize] = true;
                bottom.insert(check);
            }
        }
        if found.iter().all(|el| *el) {
            break;
        }
    }

    if is_nice_ground(&bottom, field) {
        let disp = data::Pos { x: 0, y: -min_y };
        Some(bottom.into_iter().map(|el| el.add(&disp)).collect())
    } else {
        None
    }
}

// Yeah, we are playing tetris today.
fn play_tetris(
    mut stream: Cycle<IntoIter<data::Push>>,
    mut rocks: Cycle<IntoIter<data::Rock>>,
    max_num_rocks: usize,
    rep_rounds: usize,
    do_repeat: bool,
) -> usize {
    let mut nice_repeating_grounds = Vec::<(usize, usize, HashSet<data::Pos>)>::new();
    // The field is 7 spots wide. Thus, if one wall is at 0, the other is at 8.
    let mut field = HashSet::<data::Pos>::new();
    // At the beginning, there is no rock yet. Thus, the top position is the floor.
    let mut top_rock = 0;
    let mut round = 0;
    let mut found_rep = false;
    let mut potential_ground = HashSet::<data::Pos>::new();
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
                } else {
                    // The rock hasn't settled yet. Accept the update.
                    pos = next_dropped;
                }
            }
        }
        // println!("{}", round);
        // Extract the shape of the bottom and render it. If this shape ever repeats, we're golden.
        if do_repeat && !found_rep {
            if let Some(ground) = get_ground(&field, top_rock) {
                // If we reach here, we found a nice ground, which is a ground that we know has no
                // holes.
                // Only remember the possibly repeating ground if we hadn't yet seen it.
                if let Some((last_round, last_top_rock, last_ground)) =
                    nice_repeating_grounds.iter().find_map(|el| {
                        if el.2.len() == ground.len() && (&el.2 ^ &ground).len() == 0 {
                            Some(el)
                        } else {
                            None
                        }
                    })
                {
                    // found_rep = true;
                    // We have found a repeating ground, yeah!
                    render(&ground, &data::Rock::Minus, &data::Pos { x: -10, y: -10 });
                    render(
                        &last_ground,
                        &data::Rock::Minus,
                        &data::Pos { x: -10, y: -10 },
                    );
                    println!("{} {}", last_round, last_top_rock);
                    println!("{} {}", round, top_rock);
                    if potential_ground.len() == 0 {
                        potential_ground = ground;
                    } else {
                        // Now let's first clear the entire field to save RAM and extrapolate from
                        // here.
                        field.clear();
                        let rounds_in_loop = round - last_round;
                        let increase_in_loop = top_rock as usize - last_top_rock;
                        let loops_remaining = (max_num_rocks - round) / rounds_in_loop;
                        round += loops_remaining * rounds_in_loop;
                        println!(
                            "{} {} {} {}",
                            rounds_in_loop,
                            increase_in_loop,
                            loops_remaining,
                            max_num_rocks - round,
                        );
                        top_rock += (loops_remaining * increase_in_loop) as isize;
                        // Displace the ground to where it belongs.
                        let top_rock_in_ground = ground
                            .iter()
                            .map(|el| el.y)
                            .max()
                            .expect("there is no ground");
                        let disp = data::Pos {
                            x: 0,
                            y: top_rock - top_rock_in_ground,
                        };
                        field = ground.into_iter().map(|el| el.add(&disp)).collect();
                    }
                } else {
                    nice_repeating_grounds.push((round, top_rock as usize, ground));
                }
            }
        }
    }
    println!("{} {}", round, max_num_rocks);

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
        max_num_rocks > 10_000,
    );
    println!("{}", tallness);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1, 2022)?;
    solve(REAL, 2022)?;

    solve(SAMPLE1, 1_000_000_000_000)?;
    solve(REAL, 1_000_000_000_000)?;

    Ok(())
}
// end::main[]
