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
use std::collections::{HashMap, HashSet};
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
}

fn is_blocked(field: &HashSet<data::Pos>, check: &data::Pos, width: isize) -> data::Blocked {
    if check.x <= 0 || check.x >= width {
        data::Blocked::Wall
    } else if field.contains(check) {
        data::Blocked::Rock
    } else if check.y == 0 {
        // This is hacky but we simulate a floor of rocks so that a downward operation will have
        // the rock to settle on.
        data::Blocked::Rock
    } else {
        data::Blocked::None
    }
}

// Nice means no rock can muddle its way through. There is a direct connection from the left wall
// to the right wall.
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

// We try to find the ground the last rock settled on. We basically go from top to the bottom until
// we have found at least one rock at each of the possible 7 x positions. For piece of mind, we
// only consider nice grounds (see is_nice_ground for a definition).
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

// Rep stands for repetition.
#[derive(Debug)]
struct Rep {
    round: usize,
    top_rock: isize,
    ground: HashSet<data::Pos>,
}

// Yeah, we are playing tetris today.
// This function looked OK until after part 1 but now it's horrible, but I don't want to take any
// time to refactor.
fn play_tetris(
    mut stream: Cycle<IntoIter<data::Push>>,
    mut rocks: Cycle<IntoIter<data::Rock>>,
    max_num_rocks: usize,
    num_steam_steps: usize,
    num_rock_steps: usize,
    // trigger for part 2.
    do_repeat: bool,
) -> usize {
    let mut possible_rep_store = HashMap::<(usize, usize), Vec<Rep>>::new();
    // We will track the positions of all rocks with this.
    let mut field = HashSet::<data::Pos>::new();
    // At the beginning, there is no rock yet. Thus, the top position is the floor.
    let mut top_rock = 0;
    // Round counts rocks.
    let mut round = 0;
    // Step counts gusts of air.
    let mut step = 0;
    while round < max_num_rocks {
        round += 1;
        let rock = rocks.next().expect("weren't there infinitely many rocks");
        // Spwan a rock.
        let mut pos = data::Pos {
            // There also have to be 2 free spaces in x direction. Thus, it spawns at x
            // coordinate 3.
            x: 3,
            // There have to be 3 free spaces in y direction. Thus, it spwans 4 units
            // further above.
            y: top_rock + 4,
        };
        let mut has_settled = false;
        while !has_settled {
            // Get the next stream element. This is an infinite iterator.
            let push = stream.next().expect("wasn't this supposed to be infinite");
            step += 1;

            // Apply the movement operation to the side.
            let next_pushed = push.apply(&pos);
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
                // Check whether the rock that just settled reaches higher than before.
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
        // This entire block is horrible and takes care of part 2. Ignore it for part 1.
        if do_repeat && round % num_rock_steps == 0 {
            // If we're in here, a square block has settled.
            // Determine our position in the repeating rock and steam streams.
            let rep_key = (round % num_rock_steps, step % num_steam_steps);
            if let Some(ground) = get_ground(&field, top_rock) {
                // If we're in here, then the rock has settled on a patch of ground that is nice
                // (i.e. where no rock can muddle its way through).
                if let Some(possible_rep) = possible_rep_store.get_mut(&rep_key) {
                    // If we're in here, we have already found this type of rock settling at this
                    // position in the stream/gust of air sequence.
                    // Check whether the ground repeated, too.
                    if let Some(rep) = possible_rep.iter().find_map(|el| {
                        if el.ground.len() == ground.len() && (&el.ground ^ &ground).len() == 0 {
                            Some(el)
                        } else {
                            None
                        }
                    }) {
                        // If we're in here, we found the same type of rock that settled at the
                        // same position in the steam stream AND that rock settled on an identical
                        // patch of ground as a previous rock. That means everything will repeat
                        // from here on out.
                        //
                        // Clear the field to save memory. Then, fast forward time and use the most
                        // recently found ground to reinitialise the field.
                        field.clear();
                        let rounds_in_loop = round - rep.round;
                        let increase_per_loop = (top_rock - rep.top_rock) as usize;
                        let loops_remaining = (max_num_rocks - round) / rounds_in_loop;
                        round += loops_remaining * rounds_in_loop;
                        top_rock += (loops_remaining * increase_per_loop) as isize;
                        // Displace the ground to where it belongs. It will not form the top of the
                        // field. Because the ground is nice, no rock can fall through it.
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
                    } else {
                        // Remember this ground.
                        possible_rep.push(Rep {
                            round,
                            top_rock,
                            ground,
                        });
                    }
                } else {
                    // Remember the ground for this combination of rock and position in the steam
                    // stream.
                    let rep_val = Rep {
                        round,
                        top_rock,
                        ground,
                    };
                    possible_rep_store.insert(rep_key, vec![rep_val]);
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

    let tallness = play_tetris(
        stream.infinite(),
        data::Rock::infinite_stream(),
        max_num_rocks,
        num_pushes,
        num_rocks,
        // Works but hacky trigger for part 2 ^^.
        max_num_rocks > 10_000,
    );
    println!(
        "tower will be {} tall after {} rocks",
        tallness, max_num_rocks
    );

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
