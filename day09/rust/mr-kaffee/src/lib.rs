use input::*;
use mr_kaffee_aoc::{Puzzle, Star};
use std::{collections::HashSet, ops::RangeInclusive};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 9,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &(|data| solve(data, 2, |_, _, _| ())),
            exp: Some(6_354),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &(|data| solve(data, 10, |_, _, _| ())),
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
                    .map(|l| match (l.as_bytes()[0], l[2..].parse().unwrap()) {
                        (b'U', s) => ((0, -1), s),
                        (b'D', s) => ((0, 1), s),
                        (b'L', s) => ((-1, 0), s),
                        (b'R', s) => ((1, 0), s),
                        _ => panic!("No valid move: {l}"),
                    })
                    .collect(),
            }
        }
    }

    impl PuzzleData {
        pub fn moves(&self) -> &[((isize, isize), usize)] {
            &self.moves
        }
    }
}
// end::input[]

// tag::print[]
/// Print knots for debugging purposes
///
/// Knots are printed by consecutive letters `a`, `b`, ... starting with the head at `a`
///
/// If a knot sits on a spot which is already seen, it is represented by a capital letter `A`, `B`, ...
///
/// The start is indicated by `$` if no knot is located at the start
///
/// Spots that have been seen and where currently no knot is located are shown as `#`
///
/// If several knots sit on top of each other, the first one in the chain is shown.
pub fn print(
    rx: RangeInclusive<isize>,
    ry: RangeInclusive<isize>,
    knots: &[(isize, isize)],
    ((dx, dy), s): ((isize, isize), usize),
    seen: &HashSet<(isize, isize)>,
) {
    println!("\nAfter moving {s} times by ({dx}, {dy}):");
    for y in ry {
        for x in rx.clone() {
            let seen = seen.contains(&(x, y));
            match knots
                .iter()
                .enumerate()
                .find(|(_, (xn, yn))| *xn == x && *yn == y)
            {
                Some((v, _)) if seen => print!("{}", (b'A' + v as u8) as char),
                Some((v, _)) => print!("{}", (b'a' + v as u8) as char),
                None if x == 0 && y == 0 => print!("$"),
                _ if seen => print!("#"),
                _ => print!("\u{00b7}"),
            }
        }
        println!();
    }
}
// end::print[]

// tag::solution[]
pub fn solve<F>(data: &PuzzleData, n: usize, debug: F) -> usize
where
    F: Fn(&[(isize, isize)], ((isize, isize), usize), &HashSet<(isize, isize)>) -> (),
{
    let mut seen: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);
    let mut knots: Vec<(isize, isize)> = vec![(0, 0); n];

    for ((dx, dy), s) in data.moves() {
        for _ in 0..*s {
            knots[0].0 += *dx;
            knots[0].1 += *dy;

            for k in 1..n {
                let dx = knots[k].0 - knots[k - 1].0;
                let dy = knots[k].1 - knots[k - 1].1;

                knots[k].0 += (dx < -1 || (dx == -1 && dy.abs() > 1)) as isize
                    - (dx > 1 || (dx == 1 && dy.abs() > 1)) as isize;

                knots[k].1 += (dy < -1 || (dy == -1 && dx.abs() > 1)) as isize
                    - (dy > 1 || (dy == 1 && dx.abs() > 1)) as isize;
            }

            seen.insert(knots[n - 1]);
        }

        debug(&knots, ((*dx, *dy), *s), &seen);
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
        assert_eq!(
            13,
            solve(&data, 2, |knots, mv, seen| print(
                0..=5,
                -5..=0,
                knots,
                mv,
                seen
            ))
        );
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(1, solve(&data, 10, |_, _, _| ()));

        let data = PuzzleData::from(CONTENT_2);
        assert_eq!(
            36,
            solve(&data, 10, |knots, mv, seen| print(
                -11..=14,
                -15..=5,
                knots,
                mv,
                seen
            ))
        );
    }
}
// end::tests[]
