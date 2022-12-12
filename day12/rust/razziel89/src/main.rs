#![feature(let_chains)]
// Expected input file names.
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

fn build_graph(map: &HashMap<data::Point, data::Height>) -> HashSet<data::Node> {
    map.iter()
        .map(|(&p, &h)| {
            let neighbours = p
                .env()
                .into_iter()
                .filter_map(|el| match map.get(&el) {
                    Some(other_h) => {
                        if other_h.height() <= h.height() + 1 {
                            Some(el)
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect::<HashSet<_>>();
            data::Node::new(p, h, neighbours)
        })
        .collect::<HashSet<data::Node>>()
}

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
            .ok_or(Error::msg(format!("cannot get node {:?}", node)))
    };

    let mut connections = HashMap::<&'a data::Node, (Option<data::Point>, usize)>::new();
    let mut checkable = HashMap::<&data::Node, (data::Point, usize)>::new();

    // Add starting point to resulting path.
    connections.insert(&start, (None, 0));
    // Add neighbours of starting point to list of checkable values.
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
        let (_, (predecessor, old_estimate)) = checkable
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

fn extract_shortest_path<'a>(
    end: &'a data::Node,
    points: HashMap<&'a data::Node, (Option<data::Point>, usize)>,
    graph: &'a HashSet<data::Node>,
) -> Result<Vec<&'a data::Node>> {
    let mut path = vec![end];

    let mut check_node = end;
    while let Some((Some(pre), _)) = points.get(check_node) {
        let node = graph
            .get(&pre.as_node())
            .ok_or(Error::msg("cannot find node in graph"))?;
        path.push(node);
        check_node = node;
    }

    Ok(path)
}

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let height_map = io::parse_chars_to_data::<data::Height>(file, "vec")?;

    // Create graph. To avoid self-referencing data types, we identify each node only by its
    // position.
    let mut graph = build_graph(&height_map);

    // Find start and end nodes. Also adjust heights to use actual values. This is a bit of a pain
    // in rust...
    let mut start_node = graph
        .iter()
        .find(|&el| el.get_height() == data::Height::Start)
        .ok_or(Error::msg("cannot find start node"))?
        .clone();
    start_node.set_height(data::Height::Normal(0));
    graph
        .replace(start_node.clone())
        .ok_or(Error::msg("cannot replace end node"))?;

    let mut end_node = graph
        .iter()
        .find(|&el| el.get_height() == data::Height::End)
        .ok_or(Error::msg("cannot find end node"))?
        .clone();
    end_node.set_height(data::Height::Normal(25));
    graph
        .replace(end_node.clone())
        .ok_or(Error::msg("cannot replace end node"))?;

    let estimator = |node: &data::Node, ref_point: &data::Point| node.infinity_dist(&ref_point);

    let path = find_path(&start_node, &end_node, &graph, estimator)
        .map(|el| extract_shortest_path(&end_node, el, &graph))??;

    println!("part 1: {}", path.len() - 1);

    // Part 2.
    // Add an additional node at a position that hadn't yet been part of the graph and connect it
    // to all nodes of zero elevation.
    let possible_starts = graph
        .iter()
        .filter(|node| node.get_height().height() == 0)
        .map(|node| node.pos())
        .collect::<HashSet<_>>();

    let far_away_fake_node = data::Node::new(
        data::Point { x: -10, y: -10 },
        data::Height::Normal(0),
        possible_starts,
    );

    if !graph.insert(far_away_fake_node) {
        return Err(Error::msg(
            "refusing to overwrite existing node for fake nod",
        ));
    }

    let path2 = find_path(&start_node, &end_node, &graph, estimator)
        .map(|el| extract_shortest_path(&end_node, el, &graph))??;

    // We have to ignore the first two steps here because of the real and fake start nodes.
    println!("part 2: {}", path2.len() - 2);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;
    Ok(())
}
// end::main[]
