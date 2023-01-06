// Expected input file names.
#![feature(let_chains, variant_count)]
const SAMPLE1: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod cube;
mod data;
mod io;

// tag::main[]
use anyhow::{Context, Error, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
// Constants.

// Find the first wall or free tile in a certain direction. This function is ugly but works.
fn find(
    occ_map: &HashMap<data::Point, data::Tile>,
    dir: data::Direction,
    (x_in, y_in): (Option<isize>, Option<isize>),
    max: &data::Point,
) -> Result<(data::Point, data::Tile)> {
    if x_in.is_some() && y_in.is_some() {
        return Err(Error::msg("cannot use both x and y"));
    }

    if let Some(x) = x_in {
        let neigh = match dir {
            data::Direction::Left | data::Direction::Right => {
                return Err(Error::msg("wrong vertical direction"));
            }
            data::Direction::Down => (0..=max.y)
                .find_map(|y| {
                    let pos = data::Point::new(x, y);
                    if let Some(tile) = occ_map.get(&pos) {
                        if tile != &data::Tile::None {
                            Some((pos, tile.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .ok_or(Error::msg("cannot find neigh down")),
            data::Direction::Up => (0..=max.y)
                .rev()
                .find_map(|y| {
                    let pos = data::Point::new(x, y);
                    if let Some(tile) = occ_map.get(&pos) {
                        if tile != &data::Tile::None {
                            Some((pos, tile.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .ok_or(Error::msg("cannot find neigh up")),
        };
        return neigh;
    }

    if let Some(y) = y_in {
        let neigh = match dir {
            data::Direction::Down | data::Direction::Up => {
                return Err(Error::msg("wrong vertical direction"));
            }
            data::Direction::Left => (0..=max.x)
                .rev()
                .find_map(|x| {
                    let pos = data::Point::new(x, y);
                    if let Some(tile) = occ_map.get(&pos) {
                        if tile != &data::Tile::None {
                            Some((pos, tile.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .ok_or(Error::msg("cannot find neigh left")),
            data::Direction::Right => (0..=max.x)
                .find_map(|x| {
                    let pos = data::Point::new(x, y);
                    if let Some(tile) = occ_map.get(&pos) {
                        if tile != &data::Tile::None {
                            Some((pos, tile.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .ok_or(Error::msg("cannot find neigh right")),
        };
        return neigh;
    }

    // If anything, we should find the point itself.
    Err(Error::msg("should never reach here"))
}

// This function is ugly, too but works.
fn make_neigh_map_part1(
    occ_map: &HashMap<data::Point, data::Tile>,
    max: &data::Point,
) -> HashMap<data::Point, data::Neighbours> {
    occ_map
        .iter()
        .filter(|(_point, tile)| tile != &&data::Tile::None)
        .map(|(point, _tile)| {
            // Left.
            let left = if let Some(poss) = occ_map.get(&point.add(&data::LEFT)) {
                if poss != &data::Tile::None {
                    // Point is on the map and free or a wall.
                    point.add(&data::LEFT)
                } else {
                    find(occ_map, data::Direction::Left, (None, Some(point.y)), max)
                        .expect("left")
                        .0
                }
            } else {
                find(occ_map, data::Direction::Left, (None, Some(point.y)), max)
                    .expect("left")
                    .0
            }
            .clone();
            // Right.
            let right = if let Some(poss) = occ_map.get(&point.add(&data::RIGHT)) {
                if poss != &data::Tile::None {
                    // Point is on the map and free or a wall.
                    point.add(&data::RIGHT)
                } else {
                    find(occ_map, data::Direction::Right, (None, Some(point.y)), max)
                        .expect("right")
                        .0
                }
            } else {
                find(occ_map, data::Direction::Right, (None, Some(point.y)), max)
                    .expect("right")
                    .0
            }
            .clone();
            // Up.
            let up = if let Some(poss) = occ_map.get(&point.add(&data::UP)) {
                if poss != &data::Tile::None {
                    // Point is on the map and free or a wall.
                    point.add(&data::UP)
                } else {
                    find(occ_map, data::Direction::Up, (Some(point.x), None), max)
                        .expect("up")
                        .0
                }
            } else {
                find(occ_map, data::Direction::Up, (Some(point.x), None), max)
                    .expect("up")
                    .0
            }
            .clone();
            // Down.
            let down = if let Some(poss) = occ_map.get(&point.add(&data::DOWN)) {
                if poss != &data::Tile::None {
                    // Point is on the map and free or a wall.
                    point.add(&data::DOWN)
                } else {
                    find(occ_map, data::Direction::Down, (Some(point.x), None), max)
                        .expect("down")
                        .0
                }
            } else {
                find(occ_map, data::Direction::Down, (Some(point.x), None), max)
                    .expect("down")
                    .0
            }
            .clone();

            (
                point.clone(),
                data::Neighbours {
                    left,
                    right,
                    up,
                    down,
                },
            )
        })
        .collect::<HashMap<data::Point, data::Neighbours>>()
}

fn render(occ_map: &HashMap<data::Point, data::Tile>, max: &data::Point, actor: &data::Actor) {
    for y in 0..=max.y {
        for x in 0..=max.x {
            let pos = data::Point::new(x, y);
            let char = if pos == actor.pos {
                match actor.dir {
                    data::Direction::Left => '<',
                    data::Direction::Right => '>',
                    data::Direction::Up => '^',
                    data::Direction::Down => 'v',
                }
            } else {
                match occ_map.get(&pos) {
                    None => ' ',
                    Some(data::Tile::None) => ' ',
                    Some(data::Tile::Free) => '.',
                    Some(data::Tile::Wall) => '#',
                }
            };
            // if x == 0 && y == 4 {
            //     print!("X");
            // } else {
            print!("{}", char);
            // }
        }
        println!("");
    }
    println!("{:?}\n", actor);
    std::io::stdin().lines().next();
}

fn play_game(
    actor: &mut data::Actor,
    actions: &Vec<data::Action>,
    neigh_map: &HashMap<data::Point, data::Neighbours>,
    occ_map: &HashMap<data::Point, data::Tile>,
) -> Result<()> {
    // render(&occ_map, &max, &actor);
    for action in actions {
        match action {
            data::Action::Left => {
                actor.left();
                // render(&occ_map, &max, &actor);
            }
            data::Action::Right => {
                actor.right();
                // render(&occ_map, &max, &actor);
            }
            data::Action::Move(val) => {
                for _ in 0..*val {
                    let neigh = neigh_map
                        .get(&actor.pos)
                        .expect("we should not move off the board");
                    let next_tile = occ_map
                        .get(&actor.peek(neigh))
                        .expect("all neighbours are on the map");
                    let next_neigh = neigh_map
                        .get(&actor.peek(neigh))
                        .expect("all neighbours should be on the board");
                    if next_tile == &data::Tile::Free {
                        actor.mv(neigh, next_neigh)?;
                        // render(&occ_map, &max, &actor);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    // render(&occ_map, &max, &actor);

    Ok(())
}

fn solve(file: &str, points_per_edge: usize) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    // Also obtain max coords. Min coords are implicitly 0.
    let (occ_map, max_x, max_y) = io::parse_chars_to_data::<data::Tile>(
        file,
        "tile",
        Some(|el: &String| el.contains(".") || el.contains("#")),
        None,
    )?;
    let max = data::Point::new(max_x, max_y);
    let actions = io::parse_chunks_to_data::<data::Action>(
        io::read_lines_from_file(file, 1)?,
        "action",
        Some(|el: &String| el.len() != 0 && !el.contains(".") && !el.contains("#")),
        Some(|el: String| {
            el.replace("R", "\nR\n")
                .replace("L", "\nL\n")
                .split("\n")
                .map(|el| el.to_string())
                .collect::<Vec<_>>()
        }),
    )?;

    let (start, _) =
        find(&occ_map, data::Direction::Right, (None, Some(0)), &max).context("finding start")?;

    let neigh_map_part1 = make_neigh_map_part1(&occ_map, &max);

    // println!("{:?}", occupation_map);
    // println!("{:?}", actions);
    // println!("{:?}", start);
    // println!("{:?}", neigh_map);
    // println!("{:?}", neigh_map.get(&data::Point::new(3, 7)));

    // Play the game.
    let mut actor = data::Actor {
        pos: start,
        dir: data::Direction::Right,
    };
    play_game(&mut actor, &actions, &neigh_map_part1, &occ_map)?;
    println!("{:?}", actor);
    println!(
        "{}",
        1000 * (actor.pos.y + 1) + 4 * (actor.pos.x + 1) + actor.num()
    );

    // Part 2.
    // Part 2 is identical apart from the really rather annoying construction of the neighbour map.
    let cube_map = cube::build(&occ_map, &max, points_per_edge)?;
    println!("{:?}", cube_map);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1, 4)?;
    // solve(REAL, 50)?;

    Ok(())
}
// end::main[]
