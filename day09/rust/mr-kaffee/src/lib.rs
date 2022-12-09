use std::{collections::HashSet, ops::RangeInclusive};

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 9,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &(|data| solve(data, 2)),
            exp: Some(6_354),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &(|data| solve(data, 10)),
            exp: Some(2_651),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        moves: Vec<((isize, isize), usize)>,
    }

    impl From<&'static str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &'static str) -> Self {
            Self {
                moves: s
                    .lines()
                    .map(|l| {
                        let (d, s) = l.split_once(' ').unwrap();
                        let s = s.parse().unwrap();
                        match d {
                            "U" => ((0, -1), s),
                            "D" => ((0, 1), s),
                            "L" => ((-1, 0), s),
                            "R" => ((1, 0), s),
                            _ => panic!("No valid move: {l}"),
                        }
                    })
                    .collect(),
            }
        }
    }

    impl PuzzleData {
        pub fn moves(&self) -> &[((isize, isize), usize)] {
            self.moves.as_slice()
        }
    }
}
// end::input[]

// tag::print[]
/// print knots for debugging purposes
pub fn print(rx: RangeInclusive<isize>, ry: RangeInclusive<isize>, knots: &[(isize, isize)]) {
    for y in ry {
        for x in rx.clone() {
            match knots
                .iter()
                .enumerate()
                .find(|(_, (xn, yn))| *xn == x && *yn == y)
            {
                Some((0, _)) => print!("H"),
                Some((v, _)) => print!("{v}"),
                None if x == 0 && y == 0 => print!("s"),
                _ => print!("."),
            }
        }
        println!();
    }
}
// end::print[]

// tag::solution[]
fn update(knots: &mut [(isize, isize)], seen: &mut HashSet<(isize, isize)>) {
    for k in 1..knots.len() {
        let dx = knots[k].0 - knots[k - 1].0;
        let dy = knots[k].1 - knots[k - 1].1;

        if dx < -1 || (dx < 0 && (dy < -1 || dy > 1)) {
            knots[k].0 += 1;
        } else if dx > 1 || (dx > 0 && (dy < -1 || dy > 1)) {
            knots[k].0 -= 1;
        }

        if dy < -1 || (dy < 0 && (dx < -1 || dx > 1)) {
            knots[k].1 += 1;
        } else if dy > 1 || (dy > 0 && (dx < -1 || dx > 1)) {
            knots[k].1 -= 1;
        }
    }

    let i = knots.len() - 1;
    seen.insert(knots[i].clone());
}

pub fn solve(data: &PuzzleData, n: usize) -> usize {
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut knots: Vec<(isize, isize)> = vec![(0, 0); n];
    seen.insert((0, 0));

    for ((dx, dy), s) in data.moves() {
        for _ in 0..*s {
            knots[0].0 += *dx;
            knots[0].1 += *dy;
            update(&mut knots, &mut seen);
        }
    }

    seen.len()
}
// end::solution[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    const CONTENT_2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    pub fn test_try_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(13, solve(&data, 2));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(1, solve(&data, 10));

        let data = PuzzleData::from(CONTENT_2);
        assert_eq!(36, solve(&data, 10));
    }
}
// end::tests[]
