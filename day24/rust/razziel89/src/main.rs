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
use std::collections::{HashMap, HashSet};
// Constants.

fn find_path<'a>(
    start: &'a data::Node,
    end: &'a data::Node,
    graph: &'a HashSet<data::Node>,
    estimator_fn: fn(&data::Node, &data::Point) -> usize,
) -> Result<HashMap<data::Node, (Option<data::Point>, usize)>> {
    let ref_point = end.pos();
    let estimator = move |node: &data::Node| estimator_fn(node, &ref_point);
    let get_node = move |node: &data::Point| {
        graph
            .get(&node.as_node(Some(0)))
            .ok_or(Error::msg("node not found"))
            .map(|el| el.shift(node.t))
    };

    let mut connections = HashMap::<data::Node, (Option<data::Point>, usize)>::new();
    let mut checkable = HashMap::<data::Node, (data::Point, usize)>::new();

    // Add starting point to resulting path.
    connections.insert(start.clone(), (None, 0));
    // Add neighbours of starting point to list of checkable values. Ignore neighbouring points
    // that are not part of the graph.
    for neigh in start.neighbours() {
        let neigh_node = get_node(neigh)?;
        // Estimated costs are the most direct possible connection plus 1, since every step costs
        // one.
        let estimate = estimator(&neigh_node);
        checkable.insert(neigh_node, (start.pos(), estimate));
        // connections.insert(neigh_node, (Some(start.pos()), 1));
    }

    // Search until we added the final node to the path or until there is nothing more to check.
    while !connections.contains_key(end) && checkable.len() > 0 {
        // Get node with minimum _estimated_ cost.
        let next_best_node = checkable
            .iter_mut()
            // Get node with minimum estimated cost.
            .min_by(|(_node1, (_pre1, cost1)), (_node2, (_pre2, cost2))| cost1.cmp(&cost2))
            .ok_or(Error::msg("cannot find next node"))?
            .0
            .clone();
        let (_, (predecessor, _old_estimate)) = checkable
            .remove_entry(&next_best_node)
            .ok_or(Error::msg("cannot find predecessor"))?;

        let cost_of_predecessor = connections
            .get(&predecessor.as_node(None))
            .ok_or(Error::msg("predecessor has not been visited"))?
            .1;

        // Add point to resulting path unless we've found the end/
        connections.insert(
            next_best_node.clone(),
            (Some(predecessor), cost_of_predecessor + 1),
        );
        // We've found the end. Add it in a hacky way.
        if estimator(&next_best_node) == 0 {
            connections.insert(
                next_best_node.shift(end.p.t),
                (Some(predecessor), cost_of_predecessor + 1),
            );
        }

        // Add neighbours of point to list of checkable values.
        for neigh in next_best_node.neighbours() {
            let neigh_node = get_node(neigh)?;
            if !connections.contains_key(&neigh_node) {
                let estimate = cost_of_predecessor + estimator(&neigh_node);
                let previous_best = checkable
                    .get(&neigh_node)
                    .unwrap_or(&(*neigh, std::usize::MAX))
                    .1;
                if previous_best > estimate {
                    checkable.insert(neigh_node, (next_best_node.pos(), estimate));
                }
                // connections.insert(neigh_node, Some(start.pos()));
            }
        }
    }

    Ok(connections)
}

fn render<'a>(
    time: u16,
    graph: &'a HashSet<data::Node>,
    blizzards: &'a Vec<data::Blizzard>,
    min: &'a data::Point,
    max: &'a data::Point,
) {
    let blizz = blizzards
        .iter()
        // This is a hacky way of moving the blizzard to a location but still retrieving its
        // location in a way that can easily be queried against the graph.
        .map(|el| el.at_time(time).as_node(Some(0)).p)
        .collect::<Vec<_>>();

    let mut blizz_count = HashMap::<data::Point, u8>::with_capacity(blizz.len());
    let mut blizz_last = HashMap::<data::Point, &data::Blizzard>::with_capacity(blizz.len());
    for (p, b) in blizz.iter().zip(blizzards.iter()) {
        let curr_count = blizz_count.get(p).unwrap_or(&0);
        blizz_count.insert(p.clone(), curr_count + 1);
        blizz_last.insert(p.clone(), b);
    }

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let p = data::Point::new(x as isize, y as isize, 0);
            if let Some(_) = graph.get(&p.as_node(None)) {
                // This is a potential free spot. Check whether it's a blizzard or not.
                if let Some(c) = blizz_count.get(&p) {
                    if c == &1 {
                        // Only one blizzard, print its direction.
                        print!("{}", blizz_last.get(&p).unwrap().as_char());
                    } else {
                        // Otherwise, print the count.
                        print!("{}", c);
                    }
                } else {
                    // This is a free spot.
                    print!(".");
                }
            } else {
                // This is a wall spot.
                print!("#");
            }
        }
        println!("");
    }

    // Blobk until receiving return.
    std::io::stdin().lines().next();
}

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    // Also obtain max coords. Min coords are implicitly (1,1) for free spaces..
    let (occ_map, max_x, max_y) = io::parse_chars_to_data::<data::Tile>(file, "tile", None, None)?;
    let max = data::Point::new(max_x, max_y, 0);
    let min = data::Point::new(0, 0, 0);
    let min_free = data::Point::new(1, 1, 0);

    let start = occ_map
        .iter()
        .find(|(point, tile)| point.y == 0 && tile == &&data::Tile::Free)
        .ok_or(Error::msg("cannot find start"))?
        .0
        .clone()
        .as_node(None);

    let end = occ_map
        .iter()
        .find(|(point, tile)| point.y == max.y && tile == &&data::Tile::Free)
        .ok_or(Error::msg("cannot find end"))?
        .0
        .clone()
        .as_node(None);

    // This isn't really a graph but we call it that because that's the name used in the other days
    // where the same path finding algorithm has been used.
    let graph = occ_map
        .iter()
        .filter_map(|(point, tile)| {
            if tile != &data::Tile::Wall {
                Some(point.as_node(None))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    let blizzards = occ_map
        .iter()
        .filter_map(|(point, tile)| {
            if let data::Tile::Blizzard(dir) = tile {
                Some(data::Blizzard::new(
                    point.clone(),
                    dir.clone(),
                    min_free,
                    max,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for time in 0..10 {
        render(time, &graph, &blizzards, &min, &max);
    }

    let estimator = |node: &data::Node, point: &data::Point| node.infinity_dist(&point);

    let path = find_path(&start, &end, &graph, estimator);

    println!("{:?}", path);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    // solve(REAL)?;

    Ok(())
}
// end::main[]
