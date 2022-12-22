// Expected input file names.
#![feature(let_chains, variant_count)]
const SAMPLE1: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
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
                        if tile == &data::Tile::Free {
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
                        if tile == &data::Tile::Free {
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
                        if tile == &data::Tile::Free {
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
                        if tile == &data::Tile::Free {
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
fn make_neigh_map(
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

fn solve(file: &str) -> Result<()> {
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
        None,
    )?;

    let (start, _) =
        find(&occ_map, data::Direction::Right, (None, Some(0)), &max).context("finding start")?;

    let neigh_map = make_neigh_map(&occ_map, &max);

    // println!("{:?}", occupation_map);
    // println!("{:?}", actions);
    println!("{:?}", start);
    println!("{:?}", neigh_map);
    println!("{:?}", neigh_map.get(&data::Point::new(0, 4)));

    for y in 0..=max.y {
        for x in 0..=max.x {
            let char = match occ_map.get(&data::Point::new(x, y)) {
                None => ' ',
                Some(data::Tile::None) => ' ',
                Some(data::Tile::Free) => '.',
                Some(data::Tile::Wall) => '#',
            };
            // if x == 0 && y == 4 {
            //     print!("X");
            // } else {
            print!("{}", char);
            // }
        }
        println!("");
    }

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    // solve(REAL)?;

    Ok(())
}
// end::main[]
