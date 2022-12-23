use std::collections::{HashMap, HashSet};

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 23,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(3_815),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(893),
        }),
    }
}

// tag::input[]
pub mod input {
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct PuzzleData {
        pub elves: HashSet<(isize, isize)>,
    }

    impl From<&str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &str) -> Self {
            Self {
                elves: s
                    .lines()
                    .enumerate()
                    .map(|(row, line)| {
                        line.as_bytes()
                            .iter()
                            .enumerate()
                            .filter(|(_, b)| **b == b'#')
                            .map(move |(col, _)| (col as isize, row as isize))
                    })
                    .flatten()
                    .collect(),
            }
        }
    }
}
// end::input[]

// tag::star_1[]
pub const DELTAS: [(isize, isize); 8] = [
    (1, 1),   // SOUTH EAST
    (0, 1),   // SOUTH
    (-1, 1),  // SOUTH WEST
    (-1, 0),  // WEST
    (-1, -1), // NORTH WEST
    (0, -1),  // NORTH
    (1, -1),  // NORTH EAST
    (1, 0),   // EAST
];

/// NORTH, SOUTH, WEST, EAST
pub const DIRS: [usize; 4] = [5, 1, 3, 7];

pub fn bbox(elves: &HashSet<(isize, isize)>) -> (isize, isize, isize, isize) {
    elves.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(col_min, row_min, col_max, row_max), &(col, row)| {
            (
                col_min.min(col),
                row_min.min(row),
                col_max.max(col + 1),
                row_max.max(row + 1),
            )
        },
    )
}

pub fn calc_proposal(
    elves: &HashSet<(isize, isize)>,
    search_start: usize,
    col: isize,
    row: isize,
) -> Option<(isize, isize)> {
    // calculate adjacents
    let adjacents = DELTAS.map(|(d_col, d_row)| elves.contains(&(col + d_col, row + d_row)));

    // if there are none, don't propose to move
    if adjacents.iter().all(|a| !a) {
        return None;
    }

    // find the best direction to move to starting from given search_start
    (0..DIRS.len())
        .map(|pos| DIRS[(pos + search_start) % DIRS.len()])
        .find(|&dir| {
            [
                (dir + adjacents.len() - 1) % adjacents.len(),
                dir,
                (dir + 1) % adjacents.len(),
            ]
            .iter()
            .all(|&dir| !adjacents[dir])
        })
        .map(|dir| DELTAS[dir])
        .map(|(d_col, d_row)| (col + d_col, row + d_row))
}

pub fn simulate_round(
    elves: &HashSet<(isize, isize)>,
    search_start: usize,
) -> (HashSet<(isize, isize)>, bool) {
    // create proposals
    // store count of proposals for every coordinate
    let mut proposals = HashMap::new();
    for &(col, row) in elves {
        if let Some((col_prop, row_prop)) = calc_proposal(elves, search_start, col, row) {
            *proposals.entry((col_prop, row_prop)).or_insert(0) += 1;
        }
    }

    // if no elf proposed a move, the final positions are reached
    if proposals.is_empty() {
        return (elves.clone(), false);
    }

    // perform update
    let mut elves_upd = HashSet::with_capacity(elves.len());
    for &(col, row) in elves {
        if let Some((col_prop, row_prop)) = calc_proposal(elves, search_start, col, row) {
            // if the current elf is the only one proposing to move to the given target, move
            if Some(&1) == proposals.get(&(col_prop, row_prop)) {
                elves_upd.insert((col_prop, row_prop));
                continue;
            }
        }

        // not moved
        elves_upd.insert((col, row));
    }

    // sanity check
    assert_eq!(elves.len(), elves_upd.len());

    // return updated elves and flag indicating whether there was any move
    (elves_upd, true)
}

pub fn star_1(data: &PuzzleData) -> usize {
    let mut elves = data.elves.clone();
    for k in 0..10 {
        (elves, _) = simulate_round(&elves, k % DIRS.len());
    }
    let (col_min, row_min, col_max, row_max) = bbox(&elves);
    ((row_max - row_min) * (col_max - col_min)) as usize - elves.len()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    let mut elves = data.elves.clone();
    let mut cnt = 0;
    let mut update = true;
    while update {
        (elves, update) = simulate_round(&elves, cnt % DIRS.len());
        cnt += 1;
    }

    cnt
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT_SMALL: &str = r#".....
..##.
..#..
.....
..##.
.....
"#;

    const CONTENT: &str = r#"
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
"#;

    fn print_elves(elves: &HashSet<(isize, isize)>) {
        let (col_min, row_min, col_max, row_max) = bbox(elves);
        println!("Range: {col_min}..{col_max}, {row_min}..{row_max}");
        for row in row_min..row_max {
            for col in col_min..col_max {
                if elves.contains(&(col, row)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT_SMALL);
        assert_eq!(5, data.elves.len());
    }

    #[test]
    pub fn test_simulate_round() {
        let data = PuzzleData::from(CONTENT_SMALL);

        let (update, _) = simulate_round(&data.elves, 0);
        print_elves(&update);

        let (update, _) = simulate_round(&update, 1);
        print_elves(&update);

        let (update, _) = simulate_round(&update, 2);
        print_elves(&update);
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(110, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(20, star_2(&data));
    }
}
// end::tests[]

/// preparation for a grid based solution -- maybe later
pub mod grid {
    #[derive(Clone)]
    pub struct Grid<T>
    where
        T: Clone,
    {
        data: Vec<T>,
        width: usize,
    }

    impl From<&str> for Grid<bool> {
        fn from(value: &str) -> Self {
            let width = value.as_bytes().iter().position(|&b| b == b'\n').unwrap();
            let data: Vec<bool> = value
                .as_bytes()
                .iter()
                .filter(|&&b| b != b'\n')
                .map(|&b| b == b'#')
                .collect();
            assert!(
                (data.len() / width) * width == data.len(),
                "All lines shall have equal length {width}"
            );
            Self { data, width }
        }
    }

    impl std::fmt::Display for Grid<bool> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in 0..self.data.len() / self.width {
                for col in 0..self.width {
                    if self.data[col + row * self.width] {
                        '#'.fmt(f)?;
                    } else {
                        '.'.fmt(f)?;
                    }
                }
                '\n'.fmt(f)?;
            }
            Ok(())
        }
    }

    impl<T> Grid<T>
    where
        T: Clone,
    {
        pub fn increase_by(&self, inc: usize, default: T) -> Self {
            let height = self.data.len() / self.width;
            let mut data = vec![default; (self.width + 2 * inc) * (height + 2 * inc)];
            for row in 0..self.data.len() / self.width {
                let off = (row + inc) * (self.width + 2 * inc) + inc;
                data[off..off + self.width]
                    .clone_from_slice(&self.data[row * self.width..(row + 1) * self.width]);
            }
            Self {
                data,
                width: self.width + 2 * inc,
            }
        }

        pub fn bbox<F>(&self, predicate: F) -> (usize, usize, usize, usize)
        where
            F: Fn(&T) -> bool,
        {
            self.data
                .iter()
                .enumerate()
                .filter(|&(_, e)| predicate(e))
                .map(|(pos, _)| (pos % self.width, pos / self.width))
                .fold(
                    (usize::MAX, usize::MAX, 0, 0),
                    |(col_min, row_min, col_max, row_max), (col, row)| {
                        (
                            col_min.min(col),
                            row_min.min(row),
                            col_max.max(col + 1),
                            row_max.max(row + 1),
                        )
                    },
                )
        }
    }
}
