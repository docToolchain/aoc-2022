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
use std::collections::{HashMap, HashSet};
// Constants.
const THRESHOLD: isize = 83_397_964_201_949;

fn do_op(op: char, val1: &isize, val2: &isize) -> isize {
    match op {
        '+' => val1 + val2,
        '-' => val1 - val2,
        '*' => val1 * val2,
        '/' => val1 / val2,
        '=' => {
            // if val2 != &83397964201949 {
            //     println!("{}\n== {}", val1, val2);
            // }
            if val1 == val2 {
                1
            } else {
                0
            }
        }
        _ => panic!("unknown op"),
    }
}

fn monkeys_again(monkeys: &Vec<data::Monkey>, human: Option<isize>, verbose: bool) -> isize {
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

    if let Some(val) = human {
        known.insert("humn".to_string(), val);
        let old_root = unknown
            .remove(&root_name)
            .expect("why do we have that already");
        unknown.insert(root_name.clone(), (old_root.0, old_root.1, &'='));
    }

    let known_at_start = known.iter().map(|el| el.0.clone()).collect::<HashSet<_>>();

    while unknown.contains_key(&root_name) {
        let mut moved = vec![];
        for (name, (mon1, mon2, &op)) in unknown.iter() {
            if let Some(val1) = known.get(mon1.as_str()) {
                if let Some(val2) = known.get(mon2.as_str()) {
                    known.insert(name.clone(), do_op(op, val1, val2));
                    moved.push(name.clone());
                    // Handle string representation.
                    let str1 = known_str.get(mon1.as_str()).expect("mon1 not found");
                    let str2 = known_str.get(mon2.as_str()).expect("mon2 not found");
                    let this_op = if name == "root" { '=' } else { op };
                    let this_str = match this_op {
                        '*' => format!("{}*{}", str1, str2),
                        '+' => format!("({}+{})", str1, str2),
                        '-' => format!("({}-{})", str1, str2),
                        '/' => format!("({}/{})", str1, str2),
                        '=' => format!("{}\n========\n{}", str1, str2),
                        _ => panic!("unknown op"),
                    };
                    known_str.insert(name.clone(), this_str);
                }
            }
        }
        for now_known in moved {
            unknown.remove(&now_known);
        }
    }

    if verbose {
        let mut result = known_str.iter().collect::<Vec<_>>();
        result.sort_by(|el1, el2| el1.1.len().cmp(&el2.1.len()));
        for monkey in result {
            if !known_at_start.contains(monkey.0) {
                println!("{}: {}\n", monkey.0, monkey.1);
            }
        }
    }

    *known.get(&root_name).expect("I though we found it")
}

fn fast_solve(humn: isize) -> Ordering {
    #[rustfmt::skip]
    let left = (((4*2*(2*((((((17+2*(15+((9+(2+8))+3*3)))-3*2*4)+(20-3))*5*5*5/4)+((2*11+2*3)+1)*5*5)+(((5*((3*(7*4+15)+10*3)+(((((2*13+1)+(3*11-3))+2*(3*16+(3*3+14)))-14*2)*3+1))+(((((5*7+1)+8)*2+1)*3+2*(2*((7*3+16)+4)/2)*2)-4*(16+((4*2+3)-2))))-2*(3*3*((9+2)+4*2*2)+(11*5*3+((2*(5+18)*2+2*3)+15))))/2)*2)*2+((2*(2*(3*(6+5)+(20*2/5))-14)+((2+13*3)*4/4))/3)*3*3*17)-((2*((10+(2*(8+7)/5))+3*5)+(3*(5+1)+5*5)*2)+2*(6*(2+4)+17))*7*3*13)+((2*2*((4+(5*5+8))+6)+(2*5+((5+1)+1))*(8-1))+(((9*3+5)/4)-1)*(3+4)*2)*2*(((12+1)*3+12*7)+8))*((17*((2*13*2+(2*13+(7+4)))-5*5)*((((20+5)+4)*3*5-(4*2+3*3*7)*2)*2+((4*4*(3+8)*2+3*(5*2+(5+4*2)))+((17*(2*3+1)+(((3*16+2*(5*3+(2*5*16/8)))/2)+(14+3)*3))+13*(3+10))))-(3*(5*2*2*(1+16)+(5*11+((3+10)*2+(5+2*7*3))*2))*2*(14+((10+11)+(6*4+8)))+(2*(1+(3+3))*8+15*(3+4*2))*3*(2+3*3)))+2*((7*(3*4*2*5*5-(1+2*((2*3+(14+2))/2)))*(2*(10+7)*2-3*7)+(3*((5*5+11)+5)*2+(2*13/2)*17)*2)-5*(14*(3*(3*2+1)+((1+2*5)*16+2*3*(4+8)))+(14+(2+3*((2*4+11)-2)))*(2*3+7)))*5*(1+6))*2-(((((2*(3*2+1)*(6+5*(2+5))+(((((17*3*5+2*11*(12+1))+(8+5)*5*3)+(8*3/2)*(((3*(((4*13*((2*(((2+3*(6+1))+(3*2+1)*2)+1)/2)/2)+(3*3*(18+2)+((2*(3*(4*(2+5)+1)*6-(11+3*3*2*8))+(((((((((3+4)*7*7+2*(5*(((11+(14-1))+12)-5)*4+((((((((((3*((4*((12+3*5)-(3*2+2))-((((1+(7+4))*2+(18+5))+3)/2))*2+(((20*2+7*5*3)*2/2)/5))+((((3*3+2)+4*3)*(1+2*3*3)+((((((((((2*((3+2*(1+18))+20)+((((humn-(2*7*(12*(3+2*5)/3)+((3+4)*3*2+4)))*4+(2*4*13+(13+11*2)*2))/(1+2*3))-((((((14+3)*2+1)*2/2)*2*(4+((2+6)+5)*3)/(1+(5+1)))+8*4*17)/2))*6*3*2)/5)+(3+(1+9*2)*2)*11)+3*(18*3+(1+(11+5))*5))/2)-((((4*16*2+(13+10))+(3*5+17*5))*2*2/4)+(((((8+14)*4+((5*((9+2*5)*7/7)/5)*3+4)*5)+(2*5*4+(3*3*2*3+(8+3)*7)))-(2*(3*7+(2*(3*4+(9+2*(5+6)))/2)*2)/2))+18*3)))*2*4-20*((2*19/2)+(13+3*5)))/4)+(10*10-(14+(17*(2*4*3/4)/(2+4))))*6*2)*2-((4*2+9)*2*(2+5)-(3*3+4*7)))*4)+(((2*((2*3+2*4)*5+1)+7*(7+2*4*3))*4/2)-12*19)))/2)-(((3*(2+(7+4))+7)+(3*(3+(((4+19)+17*2)/3)*2)+(((5*((((3*(3+4)*2+3*3*3*3)+((2+5)+3))/(14/2))-6)-19)-((12+8)-5))+2*(10+1))*2))+5*(20+9)))/2)+(((14+((2*5+(2*(1+((4*11/2)/2)*2)-3))+4*5))+16)*3+7*((5*7/5)*4/4)))*4-2*(15+14*7)*2)/4)+(3*(9*3+(3*2+5*2))+10)*3)*4-4*((7+3*2)+8*3*2)*2)/4)))*2-(17*3+13*4)*2)/(3+(6+2)))+6*(18+(9+2)))*9-7*3)/3)-((((2*(7*7/(6+1))/2)*2+5)*2+3*11*8)+8*11))/(3+4)))*2*3-(((((3*(5*(11*3+6)/5)+(3*19+7*4))+4*6)+(5+5*11*2))+(9*(((2*((2*11/2)+2)+3)-3)+7*6*4)-3*(11*2+(7*3+(2*11+(8+1)*2)))))/2))))/3)+((5*3*2+1)*5*2+4*11))-((19*2/2)+3*3*2)*3)/3)+(5*(5+((((3*12+11)+(5+2)*2)*3*4/2)/3))+(4+2))))*2-(16*8/4)*2*8)/8))*2-2*5*7)/4)+(12+(13*14+5*(2*20+(4+9))*3)))/3))*(2*(2+3*3)+1);

    println!("{}", left);
    if left < THRESHOLD {
        Ordering::Less
    } else if left > THRESHOLD {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
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

    let root_val = monkeys_again(&monkeys, None, true);

    println!("root's value is {}\n", root_val);

    // When starting at 0, we start at a value greater than the threshold.
    let start = if let Ok(val) = std::env::var("START") {
        val.parse()?
    } else {
        // 2_285_500
        0
    };

    assert_eq!(fast_solve(start), Ordering::Greater);

    // let end = std::isize::MAX;
    // let end = start + 1000;

    let mut step = 10_000_000;

    // Part 2.
    let mut check = start;
    let mut found = false;
    while !found {
        println!("check {}", check);
        match fast_solve(check) {
            Ordering::Equal => {
                found = true;
                println!("we need to shout +{}\n", check);
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
        // if fast_solve(-check) {
        //     println!("we need to shout -{}\n", check);
        //     break;
        // }
        // if check % 5_000_000 == 0 {
        //     println!("checked {}", check);
        // }
    }

    Ok(())
}

fn main() -> Result<()> {
    // solve(SAMPLE1)?;
    solve(REAL)?;

    // solve(SAMPLE1, 10, 811_589_153)?;
    // solve(REAL, 10, 811_589_153)?;

    Ok(())
}
// end::main[]
