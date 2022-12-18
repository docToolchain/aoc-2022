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
// Constants.

fn find_path<'a>(
    start: &'a data::Node,
    end: &'a data::Node,
    graph: &'a HashSet<data::Node>,
    estimator_fn: fn(&data::Node, &data::Point) -> usize,
) -> Result<HashMap<&'a data::Node, (Option<data::Point>, usize)>> {
    let ref_point = end.pos();
    let estimator = move |node: &data::Node| estimator_fn(node, &ref_point);
    let get_node = move |node: &data::Point| {
        graph
            .get(&node.as_node())
            .ok_or(Error::msg("node not found"))
    };

    let mut connections = HashMap::<&'a data::Node, (Option<data::Point>, usize)>::new();
    let mut checkable = HashMap::<&data::Node, (data::Point, usize)>::new();

    // Add starting point to resulting path.
    connections.insert(&start, (None, 0));
    // Add neighbours of starting point to list of checkable values. Ignore neighbouring points
    // that are not part of the graph.
    for neigh in start.neighbours() {
        let neigh_node = get_node(neigh)?;
        // Estimated costs are the most direct possible connection plus 1, since every step costs
        // one.
        checkable.insert(neigh_node, (start.pos(), estimator(neigh_node)));
        // connections.insert(neigh_node, (Some(start.pos()), 1));
    }

    // Search until we added the final node to the path or until there is nothing more to check.
    while !connections.contains_key(&end) && checkable.len() > 0 {
        // Get node with minimum _estimated_ cost.
        let next_best_node = checkable
            .iter_mut()
            // Get node with minimum estimated cost.
            .min_by(|(_node1, (_pre1, cost1)), (_node2, (_pre2, cost2))| cost1.cmp(&cost2))
            .ok_or(Error::msg("cannot find next node"))?
            .0
            .clone();
        let (_, (predecessor, _old_estimate)) = checkable
            .remove_entry(next_best_node)
            .ok_or(Error::msg("cannot find predecessor"))?;

        let cost_of_predecessor = connections
            .get(&predecessor.as_node())
            .ok_or(Error::msg("predecessor has not been visited"))?
            .1;

        // Add point to resulting path.
        connections.insert(next_best_node, (Some(predecessor), cost_of_predecessor + 1));

        // Add neighbours of point to list of checkable values.
        for neigh in next_best_node.neighbours() {
            let neigh_node = get_node(neigh)?;
            if !connections.contains_key(neigh_node) {
                let estimate = cost_of_predecessor + estimator(neigh_node);
                let previous_best = checkable
                    .get(neigh_node)
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

fn build_graph(
    lava: &HashSet<data::Point>,
    min: &data::Point,
    max: &data::Point,
) -> HashSet<data::Node> {
    let mut graph = HashSet::<data::Node>::new();

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let point = data::Point { x, y, z };
                // Ignore points that are themselves lava.
                if let Some(_) = lava.get(&point) {
                    continue;
                }
                // Find all neihhbours, which are points that are still within the boundaries and
                // not lava.
                let neighbours = point
                    .env()
                    .into_iter()
                    .filter_map(|el| {
                        // The point does not contain lava.
                        if let None = lava.get(&el) {
                            // The point is still within the cube boundaries.
                            if el.x >= min.x
                                && el.x <= max.x
                                && el.y >= min.y
                                && el.y <= max.y
                                && el.z >= min.z
                                && el.z <= max.z
                            {
                                Some(el)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                let node = data::Node {
                    p: point,
                    neighbours,
                };
                graph.insert(node);
            }
        }
    }

    graph
}

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data.
    let lava_lines = io::parse_chunks_to_data::<data::Point>(
        io::read_lines_from_file(file, 1)?,
        "pos",
        None,
        None,
    )?;

    // Part 1.

    let lava = HashSet::<data::Point>::from_iter(lava_lines.into_iter());

    let surface_area_part1 = lava
        .iter()
        .map(|el| el.env().into_iter())
        .flatten()
        .filter(|el| !lava.contains(el))
        .count();

    println!("for part 1, the surface area is {}", surface_area_part1);

    // Part 2.
    let min_x = lava
        .iter()
        .map(|el| el.x)
        .min()
        .ok_or(Error::msg("min x"))?;
    let max_x = lava
        .iter()
        .map(|el| el.x)
        .max()
        .ok_or(Error::msg("max x"))?;
    let min_y = lava
        .iter()
        .map(|el| el.y)
        .min()
        .ok_or(Error::msg("min y"))?;
    let max_y = lava
        .iter()
        .map(|el| el.y)
        .max()
        .ok_or(Error::msg("max y"))?;
    let min_z = lava
        .iter()
        .map(|el| el.z)
        .min()
        .ok_or(Error::msg("min z"))?;
    let max_z = lava
        .iter()
        .map(|el| el.z)
        .max()
        .ok_or(Error::msg("max z"))?;

    let min = data::Point {
        x: min_x - 1,
        y: min_y - 1,
        z: min_z - 1,
    };
    // Max is also the target for the pahtfinding algorithm.
    let max = data::Point {
        x: max_x + 1,
        y: max_y + 1,
        z: max_z + 1,
    };

    // Construct graph that does not contain diagonal connections.
    let graph = build_graph(&lava, &min, &max);
    // This is the heuristic needed for A*.
    let estimator = |node: &data::Node, ref_point: &data::Point| node.infinity_dist(&ref_point);
    // Define the end point of the paht finding.
    let end = graph
        .get(&max.as_node())
        .ok_or(Error::msg("cannot find end node in graph"))?;

    // println!("{:?}", graph);

    let mut air_pockets = HashSet::<data::Point>::new();

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let start_point = data::Point { x, y, z };
                if let Some(_) = lava.get(&start_point) {
                    // Do not  try to find paths that start at lava.
                    continue;
                }
                if let Some(_) = air_pockets.get(&start_point) {
                    // Do not  try to find paths that start at lava.
                    continue;
                }
                let start = graph
                    .get(&start_point.as_node())
                    .ok_or(Error::msg("cannot find start node in graph"))?;
                // try to find the path to the max node.
                let path = find_path(&start, &end, &graph, estimator)?;
                // If the end node is not in the found path, we discovered an air pocket.
                if let None = path.get(&end) {
                    air_pockets = &air_pockets | &path.into_iter().map(|el| el.0.p).collect();
                }
            }
        }
    }

    let lava_with_filled_pockets = &lava | &air_pockets;

    // println!("{} => {:?} {:?}", lava.len(), min, max,);

    let surface_area_part2 = lava_with_filled_pockets
        .iter()
        .map(|el| el.env().into_iter())
        .flatten()
        .filter(|el| !lava_with_filled_pockets.contains(el))
        .count();

    println!("for part 2, the surface area is {}", surface_area_part2);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
