use std::collections::HashMap;

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData<'static>, isize, isize, usize, usize> {
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
            exp: None,
        }),
    }
}

// tag::input[]
pub mod input {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum Yell<'a> {
        Operation(&'a str, &'a str, &'a str),
        Number(isize),
    }

    pub fn parse_yell<'a>(line: &'a str) -> (&'a str, Yell<'a>) {
        let mut words = line.split_ascii_whitespace();
        let name = words.next().unwrap().trim_end_matches(':');
        let a = words.next().unwrap();
        let b = a.as_bytes()[0];
        let yell = if (b as char).is_ascii_digit() {
            Yell::Number(a.parse().unwrap())
        } else {
            Yell::Operation(a, words.next().unwrap(), words.next().unwrap())
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
                _ => panic!(),
            }
        }
        Some(Yell::Number(v)) => *v,
        _ => panic!(),
    }
}

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> isize {
    get_result(&data.monkeys, "root")
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(_data: &PuzzleData) -> usize {
    0
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"Hello World!
Advent of Code 2022
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
