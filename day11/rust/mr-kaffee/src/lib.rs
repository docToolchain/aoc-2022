use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

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
        pub worries: Vec<usize>,
        pub upd: Operation,
        pub test: usize,
        pub if_true: usize,
        pub if_false: usize,
    }

    impl From<&'static str> for Monkey {
        fn from(monkey: &'static str) -> Self {
            let words = monkey.split_ascii_whitespace();

            let mut words = words.skip(4); // Monkey <id>: Starting items:

            let mut worries = Vec::new();
            let mut word = words.next().unwrap();
            while word != "Operation:" {
                worries.push(word.trim_end_matches(',').parse().unwrap());
                word = words.next().unwrap();
            }

            let mut words = words.skip(3); // new = old

            let upd = match (words.next().unwrap(), words.next().unwrap()) {
                ("*", "old") => Operation::Square,
                ("+", "old") => Operation::Double,
                ("*", v) => Operation::Times(v.parse().unwrap()),
                ("+", v) => Operation::Plus(v.parse().unwrap()),
                _ => unreachable!(),
            };

            let mut words = words.skip(3); // Test: divisible by

            let test = words.next().unwrap().parse().unwrap();

            let mut words = words.skip(5); // If true: throw to monkey

            let if_true = words.next().unwrap().parse().unwrap();

            let mut words = words.skip(5); // If false: throw to monkey

            let if_false = words.next().unwrap().parse().unwrap();

            Self {
                worries,
                upd,
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
    for id in 0..monkeys.len() {
        counts[id] += monkeys[id].worries.len();
        for k in 0..monkeys[id].worries.len() {
            let worry = (monkeys[id].upd.apply(monkeys[id].worries[k]) / div) % m;
            let target = if worry % monkeys[id].test == 0 {
                monkeys[id].if_true
            } else {
                monkeys[id].if_false
            };
            monkeys[target].worries.push(worry);
        }
        monkeys[id].worries.clear();
    }
}

pub fn solve(data: &PuzzleData, div: usize, rounds: usize) -> usize {
    let mut monkeys = Vec::from(data.monkeys());
    let mut counts = vec![0; monkeys.len()];
    let m = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..rounds {
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
        assert_eq!(vec![20, 23, 27, 26], monkeys[0].worries);
        assert_eq!(vec![2080, 25, 167, 207, 401, 1046], monkeys[1].worries);
        assert!(monkeys[2].worries.is_empty());
        assert!(monkeys[3].worries.is_empty());
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
