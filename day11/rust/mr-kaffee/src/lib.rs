use input::*;
use mr_kaffee_aoc::{Puzzle, Star};
use std::collections::VecDeque;

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 11,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &(|data| solve(data, 3, 20)),
            exp: Some(61_005),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &(|data| solve(data, 1, 10_000)),
            exp: Some(20_567_144_694),
        }),
    }
}

// tag::input[]
pub mod input {
    use std::collections::VecDeque;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Operation {
        Plus(usize),
        Times(usize),
        Square,
        Double,
    }

    impl Operation {
        pub fn apply(&self, item: usize) -> usize {
            match self {
                Operation::Plus(v) => item + v,
                Operation::Times(v) => item * v,
                Operation::Square => item * item,
                Operation::Double => item + item,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Monkey {
        pub id: usize,
        pub items: VecDeque<usize>,
        pub operation: Operation,
        pub test: usize,
        pub if_true: usize,
        pub if_false: usize,
    }

    fn advance_by<'a, T>(iter: &mut T, n: usize)
    where
        T: Iterator<Item = &'a str>,
    {
        for _ in 0..n {
            iter.next();
        }
    }

    impl From<&'static str> for Monkey {
        fn from(monkey: &'static str) -> Self {
            let mut parts = monkey.split_ascii_whitespace();

            advance_by(&mut parts, 1); // Monkey

            let id = parts.next().unwrap().trim_end_matches(':').parse().unwrap();

            advance_by(&mut parts, 2); // Starting items:

            let mut items = VecDeque::new();
            let mut cur = parts.next().unwrap();
            while cur != "Operation:" {
                items.push_back(cur.trim_end_matches(',').parse().unwrap());
                cur = parts.next().unwrap();
            }

            advance_by(&mut parts, 3); // new = old

            let operation = match (parts.next().unwrap(), parts.next().unwrap()) {
                ("*", "old") => Operation::Square,
                ("+", "old") => Operation::Double,
                ("*", v) => Operation::Times(v.parse().unwrap()),
                ("+", v) => Operation::Plus(v.parse().unwrap()),
                (op, v) => panic!("Bad part: {op} {v}"),
            };

            advance_by(&mut parts, 3); // Test: divisible by

            let test = parts.next().unwrap().parse().unwrap();

            advance_by(&mut parts, 5); // If true: throw to monkey

            let if_true = parts.next().unwrap().parse().unwrap();

            advance_by(&mut parts, 5); // If false: throw to monkey

            let if_false = parts.next().unwrap().parse().unwrap();

            Self {
                id,
                items,
                operation,
                test,
                if_true,
                if_false,
            }
        }
    }

    #[derive(Debug)]
    pub struct PuzzleData {
        monkeys: Vec<Monkey>,
    }

    impl From<&'static str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &'static str) -> Self {
            Self {
                monkeys: s.split("\n\n").map(Monkey::from).collect(),
            }
        }
    }

    impl PuzzleData {
        pub fn monkeys(&self) -> &[Monkey] {
            &self.monkeys
        }
    }
}
// end::input[]

// tag::solution[]
pub fn round(monkeys: &mut Vec<Monkey>, counts: &mut Vec<usize>, div: usize, m: usize) {
    let mut queue = VecDeque::new();
    for id in 0..monkeys.len() {
        counts[id] += monkeys[id].items.len();
        while let Some(item) = monkeys[id].items.pop_front() {
            let item = (monkeys[id].operation.apply(item) / div) % m;
            if item % monkeys[id].test == 0 {
                queue.push_back((monkeys[id].if_true, item));
            } else {
                queue.push_back((monkeys[id].if_false, item));
            }
        }
        while let Some((id, item)) = queue.pop_front() {
            monkeys[id].items.push_back(item);
        }
    }
}

pub fn solve(data: &PuzzleData, div: usize, rounds: usize) -> usize {
    let mut monkeys = Vec::from(data.monkeys());
    let mut counts = vec![0; monkeys.len()];
    let m = monkeys.iter().fold(1, |m, monkey| m * monkey.test);

    for _round in 0..rounds {
        round(&mut monkeys, &mut counts, div, m);
    }

    counts.sort_unstable();

    counts.pop().unwrap() as usize * counts.pop().unwrap() as usize
}
// end::solution[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_round() {
        let data = PuzzleData::from(CONTENT);
        let mut monkeys = Vec::from(data.monkeys());
        let mut counts = vec![0; monkeys.len()];
        round(&mut monkeys, &mut counts, 3, usize::MAX);
        assert_eq!(VecDeque::from([20, 23, 27, 26]), monkeys[0].items);
        assert_eq!(
            VecDeque::from([2080, 25, 167, 207, 401, 1046]),
            monkeys[1].items
        );
        assert!(monkeys[2].items.is_empty());
        assert!(monkeys[3].items.is_empty());
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(10605, solve(&data, 3, 20));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);

        assert_eq!(4 * 6, solve(&data, 1, 1));
        assert_eq!(99 * 103, solve(&data, 1, 20));

        assert_eq!(2_713_310_158, solve(&data, 1, 10_000));
    }
}
// end::tests[]
