use std::collections::VecDeque;

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 12,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(394),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(388),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        pub grid: Vec<u8>,
        pub width: usize,
        pub start: usize,
        pub target: usize,
    }

    impl<'a> From<&'a str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &'a str) -> Self {
            let width = s.find('\n').unwrap();

            let mut grid: Vec<_> = s
                .as_bytes()
                .iter()
                .cloned()
                .filter(|b| *b != b'\n')
                .collect();
            let start = grid.iter().position(|&b| b == b'S').unwrap();
            let target = grid.iter().position(|&b| b == b'E').unwrap();

            grid[start] = b'a';
            grid[target] = b'z';

            Self {
                grid,
                width,
                start,
                target,
            }
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn shortest_path(data: &PuzzleData, start: usize) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    let mut seen = vec![false; data.grid.len()];
    seen[start] = true;

    let height = data.grid.len() / data.width;

    while let Some((steps, pos)) = queue.pop_front() {
        if pos == data.target {
            return Some(steps);
        }

        let x = pos % data.width;
        let y = pos / data.width;

        for (chk, nxt) in [
            (x > 0, pos as isize - 1),
            (x < data.width - 1, pos as isize + 1),
            (y > 0, pos as isize - data.width as isize),
            (y < height - 1, pos as isize + data.width as isize),
        ] {
            if chk && !seen[nxt as usize] && data.grid[nxt as usize] <= data.grid[pos] + 1 {
                queue.push_back((steps + 1, nxt as _));
                seen[nxt as usize] = true;
            }
        }
    }

    None
}

pub fn star_1(data: &PuzzleData) -> usize {
    shortest_path(data, data.start).unwrap()
}
// end::star_1[]

// tag::star_2[]
pub fn shortest_path_2<F, G>(data: &PuzzleData, start: usize, reached: F, check: G) -> Option<usize>
where
    F: Fn(usize) -> bool,
    G: Fn(u8, u8) -> bool,
{
    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    let mut seen = vec![false; data.grid.len()];
    seen[start] = true;

    let height = data.grid.len() / data.width;

    while let Some((steps, pos)) = queue.pop_front() {
        if reached(pos) {
            return Some(steps);
        }

        let x = pos % data.width;
        let y = pos / data.width;

        for (chk, nxt) in [
            (x > 0, pos as isize - 1),
            (x < data.width - 1, pos as isize + 1),
            (y > 0, pos as isize - data.width as isize),
            (y < height - 1, pos as isize + data.width as isize),
        ] {
            if chk && !seen[nxt as usize] && check(data.grid[pos], data.grid[nxt as usize]) {
                queue.push_back((steps + 1, nxt as _));
                seen[nxt as usize] = true;
            }
        }
    }

    None
}

pub fn star_2(data: &PuzzleData) -> usize {
    shortest_path_2(
        data,
        data.target,
        |pos| data.grid[pos] == b'a',
        |f, t| f <= t + 1,
    )
    .unwrap()
}
// end::star_2[]

// tag::star_2_original[]
pub fn star_2_original(data: &PuzzleData) -> usize {
    (0..data.grid.len())
        .filter(|&k| data.grid[k] == b'a')
        .filter_map(|start| shortest_path(data, start))
        .min()
        .unwrap()
}
// end::star_2_original[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(8, data.width);
        assert_eq!(0, data.start);
        assert_eq!(21, data.target);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(31, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(29, star_2(&data));
    }
}
// end::tests[]
