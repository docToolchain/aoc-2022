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
use std::collections::HashMap;
// Constants.

fn do_op(op: char, val1: &isize, val2: &isize) -> isize {
    match op {
        '+' => val1 + val2,
        '-' => val1 - val2,
        '*' => val1 * val2,
        '/' => val1 / val2,
        '=' => {
            if val1 == val2 {
                1
            } else {
                0
            }
        }
        _ => panic!("unknown op"),
    }
}

fn monkeys_again(monkeys: &Vec<data::Monkey>, human: Option<isize>) -> isize {
    // A map from monkey names to their numbers for those monkeys that already know them.
    let mut known = monkeys
        .iter()
        .filter_map(|el| {
            if let data::Action::Shout(num) = el.action {
                Some((el.name.clone(), num))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    // A map from monkey names to their ops for those monkeys that don't yet know their numbers.
    let mut unknown = monkeys
        .iter()
        .filter_map(|el| {
            if let data::Action::Op(mon1, mon2, op) = &el.action {
                Some((el.name.clone(), (mon1, mon2, op)))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let root_name = "root".to_string();

    if let Some(val) = human {
        known.insert("humn".to_string(), val);
        let old_root = unknown
            .remove(&root_name)
            .expect("why do we have that already");
        unknown.insert(root_name.clone(), (old_root.0, old_root.1, &'='));
    }

    while unknown.contains_key(&root_name) {
        let mut moved = vec![];
        for (name, (mon1, mon2, &op)) in unknown.iter() {
            if let Some(val1) = known.get(mon1.as_str()) {
                if let Some(val2) = known.get(mon2.as_str()) {
                    known.insert(name.clone(), do_op(op, val1, val2));
                    moved.push(name.clone());
                }
            }
        }
        for now_known in moved {
            unknown.remove(&now_known);
        }
    }

    *known.get(&root_name).expect("I though we found it")
}

fn solve(file: &str) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into data. We use a custom struct here just so we can continue using
    // our parser function.
    let monkeys = io::parse_chunks_to_data::<data::Monkey>(
        io::read_lines_from_file(file, 1)?,
        "monkey",
        None,
        None,
    )?;

    let root_val = monkeys_again(&monkeys, None);

    println!("root's value is {}\n", root_val);

    // Part 2.
    for check in 0..std::isize::MAX {
        if check % 500 == 0 {
            println!("checking {}", check);
        }
        if monkeys_again(&monkeys, Some(check)) == 1 {
            println!("we need to shout +{}\n", check);
            break;
        }
        if monkeys_again(&monkeys, Some(-check)) == 1 {
            println!("we need to shout -{}\n", check);
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;

    // solve(SAMPLE1, 10, 811_589_153)?;
    // solve(REAL, 10, 811_589_153)?;

    Ok(())
}
// end::main[]
