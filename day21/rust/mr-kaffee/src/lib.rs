use std::collections::HashMap;

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData<'static>, isize, isize, isize, isize> {
    Puzzle {
        year: 2022,
        day: 21,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(43_699_799_094_202),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(3_375_719_472_770),
        }),
    }
}

// tag::input[]
pub mod input {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub enum Yell<'a> {
        Operation(&'a str, &'a str, &'a str),
        Number(isize),
        Unknown, // required for part 2
    }

    pub fn parse_yell<'a>(line: &'a str) -> (&'a str, Yell<'a>) {
        let mut words = line.split_ascii_whitespace();
        let name = words.next().unwrap().trim_end_matches(':');
        let word = words.next().unwrap();
        let yell = if (word.as_bytes()[0] as char).is_ascii_digit() {
            Yell::Number(word.parse().unwrap())
        } else {
            Yell::Operation(word, words.next().unwrap(), words.next().unwrap())
        };
        (name, yell)
    }

    #[derive(Debug)]
    pub struct PuzzleData<'a> {
        pub monkeys: HashMap<&'a str, Yell<'a>>,
    }

    impl<'a> From<&'a str> for PuzzleData<'a> {
        /// parse the puzzle input
        fn from(s: &'a str) -> Self {
            Self {
                monkeys: s.lines().map(parse_yell).collect(),
            }
        }
    }
}
// end::input[]

pub fn get_result(monkeys: &HashMap<&str, Yell<'_>>, monkey: &str) -> isize {
    match monkeys.get(monkey) {
        Some(Yell::Operation(lhs, op, rhs)) => {
            let lhs = get_result(monkeys, lhs);
            let rhs = get_result(monkeys, rhs);
            match *op {
                "+" => lhs + rhs,
                "-" => lhs - rhs,
                "*" => lhs * rhs,
                "/" => lhs / rhs,
                _ => panic!("Unknown operation: {op}"),
            }
        }
        Some(Yell::Number(v)) => *v,
        yell => panic!("Can't get result for monkey {monkey} => {yell:?}"),
    }
}

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> isize {
    get_result(&data.monkeys, "root")
}
// end::star_1[]

// tag::star_2[]
#[derive(Debug)]
pub enum YellRec<'a> {
    Operation(Box<(YellRec<'a>, &'a str, YellRec<'a>)>),
    Number(isize),
    Unknown,
}

pub fn reduce<'a>(monkeys: &HashMap<&str, Yell<'a>>, monkey: &str) -> YellRec<'a> {
    match monkeys.get(monkey) {
        Some(Yell::Operation(lhs, op, rhs)) => {
            let lhs = reduce(monkeys, lhs);
            let rhs = reduce(monkeys, rhs);
            match (lhs, rhs) {
                (YellRec::Number(lhs), YellRec::Number(rhs)) => match *op {
                    "+" => YellRec::Number(lhs + rhs),
                    "-" => YellRec::Number(lhs - rhs),
                    "*" => YellRec::Number(lhs * rhs),
                    "/" => YellRec::Number(lhs / rhs),
                    _ => panic!("Unknown operation: {op}"),
                },
                (lhs, rhs) => YellRec::Operation((lhs, *op, rhs).into()),
            }
        }
        Some(Yell::Number(v)) => YellRec::Number(*v),
        Some(Yell::Unknown) => YellRec::Unknown,
        yell => panic!("Can't get result for monkey {monkey} => {yell:?}"),
    }
}

pub fn solve(yell: &YellRec, tar: isize) -> isize {
    match yell {
        YellRec::Operation(b) => match b.as_ref() {
            (lhs, op, YellRec::Number(rhs)) => match *op {
                "+" => solve(lhs, tar - *rhs), // lhs + rhs = tar
                "-" => solve(lhs, tar + *rhs), // lhs - rhs = tar
                "*" => solve(lhs, tar / *rhs), // lhs * rhs = tar
                "/" => solve(lhs, tar * *rhs), // lhs / rhs = tar
                _ => panic!("Unknown operation: {op}"),
            },
            (YellRec::Number(lhs), op, rhs) => match *op {
                "+" => solve(rhs, tar - *lhs), // lhs + rhs = tar
                "-" => solve(rhs, *lhs - tar), // lhs - rhs = tar
                "*" => solve(rhs, tar / *lhs), // lhs * rhs = tar
                "/" => solve(rhs, *lhs / tar), // lhs / rhs = tar
                _ => panic!("Unknown operation: {op}"),
            },
            _ => panic!("solve expects either rhs or lhs to be a number"),
        },
        YellRec::Unknown => tar,
        YellRec::Number(_) => panic!("Can't solve a number"),
    }
}

pub fn star_2(data: &PuzzleData) -> isize {
    let mut monkeys = data.monkeys.clone();

    let Some(Yell::Operation(lhs, _, rhs)) = monkeys.get("root") else {panic!()};
    monkeys.insert("root", Yell::Operation(lhs, "-", rhs));
    monkeys.insert("humn", Yell::Unknown);

    solve(&reduce(&monkeys, "root"), 0)
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(152, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(301, star_2(&data));
    }
}
// end::tests[]
