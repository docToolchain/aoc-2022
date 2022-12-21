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
use std::cmp::Ordering;
use std::collections::HashMap;
// Constants.

fn do_op(op: char, val1: &isize, val2: &isize) -> (isize, isize) {
    match op {
        '+' => (val1 + val2, 0),
        '-' => (val1 - val2, 0),
        '*' => (val1 * val2, 0),
        '/' => (val1 / val2, val1 % val2),
        _ => panic!("unknown op"),
    }
}

fn do_op_str(op: char, str1: &str, str2: &str) -> String {
    match op {
        '*' => format!("{}*{}", str1, str2),
        '+' => format!("({}+{})", str1, str2),
        '-' => format!("({}-{})", str1, str2),
        '/' => format!("({}/{})", str1, str2),
        _ => panic!("unknown op"),
    }
}

fn monkeys_again(
    monkeys: &Vec<data::Monkey>,
    human: isize,
    part1: bool,
) -> Result<(isize, isize, char, isize, bool)> {
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
    if !part1 {
        known.insert("humn".to_string(), human);
    }

    // We use this to determine which side of part 2's root equation is independent of the human
    // value.
    let mut known_str = monkeys
        .iter()
        .filter_map(|el| {
            if let data::Action::Shout(num) = el.action {
                Some((el.name.clone(), format!("{}", num)))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    known_str.insert("humn".to_string(), "humn".to_string());

    // A map from monkey names to their ops for those monkeys that don't yet know their numbers.
    let mut unknown = monkeys
        .iter()
        .filter_map(|el| {
            if let data::Action::Op(mon1, mon2, op) = &el.action {
                if !known.contains_key(&el.name) {
                    Some((el.name.clone(), (mon1, mon2, op)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let root_name = "root".to_string();
    let mut largest_remainder = 0;

    while unknown.len() > 0 {
        let mut moved = vec![];
        for (name, (mon1, mon2, &op)) in unknown.iter() {
            if let Some(val1) = known.get(mon1.as_str()) {
                if let Some(val2) = known.get(mon2.as_str()) {
                    if name == &root_name {
                        // We've reached the end.
                        if !part1 {
                            return Ok((*val1, *val2, op, largest_remainder, false));
                        } else {
                            let str1 = known_str.get(mon1.as_str()).expect("mon1 not found");
                            let str2 = known_str.get(mon2.as_str()).expect("mon2 not found");

                            let left_has_human = if !str1.contains("humn") && str2.contains("humn")
                            {
                                false
                            } else if str1.contains("humn") && !str2.contains("humn") {
                                true
                            } else {
                                return Err(Error::msg("neither side depends on the human"));
                            };
                            return Ok((*val1, *val2, op, largest_remainder, left_has_human));
                        }
                    } else {
                        let result = do_op(op, val1, val2);
                        if result.1.abs() > largest_remainder {
                            largest_remainder = result.1.abs();
                        }
                        known.insert(name.clone(), result.0);
                        // Handle string representation.
                        if part1 {
                            let str1 = known_str.get(mon1.as_str()).expect("mon1 not found");
                            let str2 = known_str.get(mon2.as_str()).expect("mon2 not found");
                            known_str.insert(name.clone(), do_op_str(op, str1, str2));
                        }
                        // We will remove the ones in thos vector from the map of unknown values.
                        moved.push(name.clone());
                    }
                }
            }
        }
        for now_known in moved {
            unknown.remove(&now_known);
        }
    }

    Err(Error::msg("we didn't find the root monkey"))
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

    let (left_root_val, right_root_val, root_op, largest_remainder, left_has_human) =
        monkeys_again(&monkeys, 0, true)?;

    assert_eq!(largest_remainder, 0, "remainder is zero");
    assert!(
        left_has_human,
        "right value is constant, swap left and right values if it isn't"
    );
    let (root_val, _) = do_op(root_op, &left_root_val, &right_root_val);

    println!("root's value is {}", root_val);

    let start = 0;
    let end = std::isize::MAX;
    let mut step = 1_000_000_000_000;

    // This is hacky but it works. We assume a certain ordering for the left and right values at
    // zero. If the ordering is not what we expect, we simply multiply both sides by -1, which
    // "fixes" the ordering.
    let (zero_left_root_val, zero_right_root_val, _, _, _) = monkeys_again(&monkeys, 0, false)?;
    let inv = if zero_left_root_val > zero_right_root_val {
        1
    } else {
        -1
    };

    // Part 2.
    // We assume the number we need to shout is positive.
    let mut check = start;
    let mut found = false;
    while !found && end - step > check {
        let (left_root_val, right_root_val, _, largest_remainder, _) =
            monkeys_again(&monkeys, check, false)?;

        // println!("{} == {}", left_root_val, right_root_val);
        match (inv * left_root_val).cmp(&(inv * right_root_val)) {
            Ordering::Equal => {
                if largest_remainder == 0 {
                    found = true;
                    println!("we need to shout {}\n", check);
                } else {
                    panic!("we found a non-zero remainder");
                }
            }
            Ordering::Less => {
                check -= step;
                step /= 10;
                if step == 0 {
                    step = 1;
                }
            }
            Ordering::Greater => {
                check += step;
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE1)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]
