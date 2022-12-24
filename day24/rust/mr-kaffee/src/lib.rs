use std::collections::{BinaryHeap, HashSet};

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 24,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(247),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(728),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        pub blizzards_r: Vec<bool>,
        pub blizzards_u: Vec<bool>,
        pub blizzards_l: Vec<bool>,
        pub blizzards_d: Vec<bool>,
        pub width: usize,
        pub height: usize,
        pub entry: usize,
        pub exit: usize,
    }

    impl From<&str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &str) -> Self {
            let b = s.as_bytes();

            let width = b.iter().position(|&b| b == b'\n').unwrap() - 2;
            let entry = b.iter().position(|&b| b == b'.').unwrap() - 1;
            let exit = width
                - b.iter()
                    .rev()
                    .filter(|&&b| b != b'\n')
                    .position(|&b| b == b'.')
                    .unwrap();

            let blizzards_r: Vec<bool> = b
                .iter()
                .skip(width + 3)
                .take(b.len() - 2 * width - 6)
                .filter(|&&b| b != b'\n' && b != b'#')
                .map(|&b| b == b'>')
                .collect();
            let blizzards_l = b
                .iter()
                .skip(width + 3)
                .take(b.len() - 2 * width - 6)
                .filter(|&&b| b != b'\n' && b != b'#')
                .map(|&b| b == b'<')
                .collect();
            let blizzards_u = b
                .iter()
                .skip(width + 3)
                .take(b.len() - 2 * width - 6)
                .filter(|&&b| b != b'\n' && b != b'#')
                .map(|&b| b == b'^')
                .collect();
            let blizzards_d = b
                .iter()
                .skip(width + 3)
                .take(b.len() - 2 * width - 6)
                .filter(|&&b| b != b'\n' && b != b'#')
                .map(|&b| b == b'v')
                .collect();

            let height = blizzards_r.len() / width;

            Self {
                blizzards_r,
                blizzards_l,
                blizzards_u,
                blizzards_d,
                width,
                height,
                entry,
                exit,
            }
        }
    }
}
// end::input[]

// tag::star_1[]
impl PuzzleData {
    pub fn is_blizzard_r(&self, (col, row): (usize, usize), time: usize) -> bool {
        self.blizzards_r[(col + self.width - time % self.width) % self.width + self.width * row]
    }

    pub fn is_blizzard_u(&self, (col, row): (usize, usize), time: usize) -> bool {
        self.blizzards_u[col + self.width * ((row + time) % self.height)]
    }

    pub fn is_blizzard_l(&self, (col, row): (usize, usize), time: usize) -> bool {
        self.blizzards_l[(col + time) % self.width + self.width * row]
    }

    pub fn is_blizzard_d(&self, (col, row): (usize, usize), time: usize) -> bool {
        self.blizzards_d
            [col + self.width * ((row + self.height - time % self.height) % self.height)]
    }

    pub fn is_blizzard(&self, pos: (usize, usize), time: usize) -> bool {
        self.is_blizzard_r(pos, time)
            || self.is_blizzard_u(pos, time)
            || self.is_blizzard_l(pos, time)
            || self.is_blizzard_d(pos, time)
    }

    pub fn print(&self, col: usize, row: usize, time: usize) {
        // first row
        print!("#");
        for col_ in 0..self.width {
            if col_ != self.entry {
                print!("#");
            } else if col_ == col && row == 0 {
                print!("E");
            } else {
                print!(".");
            }
        }
        println!("#");

        for row_ in 0..self.height {
            print!("#");
            for col_ in 0..self.width {
                let r = self.is_blizzard_r((col_, row_), time) as u8;
                let u = self.is_blizzard_u((col_, row_), time) as u8;
                let l = self.is_blizzard_l((col_, row_), time) as u8;
                let d = self.is_blizzard_d((col_, row_), time) as u8;
                if (r + u + l + d) > 1 {
                    print!("{}", r + u + l + d);
                } else if r > 0 {
                    print!(">");
                } else if u > 0 {
                    print!("^");
                } else if l > 0 {
                    print!("<");
                } else if d > 0 {
                    print!("v");
                } else if row_ + 1 == row && col_ == col {
                    print!("E");
                } else {
                    print!(".");
                }
            }
            println!("#");
        }

        // last row
        print!("#");
        for col_ in 0..self.width {
            if col_ != self.exit {
                print!("#");
            } else if col_ == col && row == self.height + 1 {
                print!("E");
            } else {
                print!(".");
            }
        }
        println!("#");
    }

    pub fn shortest_path(
        &self,
        start: (usize, usize),
        target: (usize, usize),
        start_time: usize,
    ) -> usize {
        let w = self.width;
        let h = self.height;

        let mut queue = BinaryHeap::new();

        // items on the queue are
        // - lower bound of time to target (Manhattan distance)
        // - time used so far
        // - current position (row, col)
        queue.push((
            !(self.entry.max(self.exit) - self.entry.min(self.exit) + h + 1),
            !start_time,
            start,
        ));

        // do not visit nodes twice
        let mut seen = HashSet::from([(start_time % (w * h), start)]);

        while let Some((_n_bound, n_time, (col, row))) = queue.pop() {
            let time = !n_time;
            if (col, row) == target {
                return time;
            }

            let time_upd = time + 1;

            let mut enqueue = |(c, r), t| {
                if (r == 0 || r == h + 1 || !self.is_blizzard((c, r - 1), t))
                    && seen.insert((t % (w * h), (c, r)))
                {
                    let bound =
                        target.0.max(c) - target.0.min(c) + target.1.max(r) - target.1.min(r) + t;
                    queue.push((!bound, !t, (c, r)))
                }
            };

            // move to entry
            if row == 1 && col == self.entry {
                enqueue((self.entry, 0), time_upd);
            }
            // move to exit
            if row == h && col == self.exit {
                enqueue((self.exit, h + 1), time_upd);
            }
            // don't move at entry or exit or any position where there is no blizzard
            enqueue((col, row), time_upd);
            // move left (not in entry / exit rows)
            if row != 0 && row != h + 1 && col > 0 {
                enqueue((col - 1, row), time_upd);
            }
            // move up (not in entry row and row immediately below)
            if row > 1 {
                enqueue((col, row - 1), time_upd);
            }
            // move right (not in entry / exit rows)
            if row != 0 && row != h + 1 && col < w - 1 {
                enqueue((col + 1, row), time_upd);
            }
            // move down (not in exit row and row immediately above)
            if row < h {
                enqueue((col, row + 1), time_upd);
            }
        }

        panic!(
            "I've seen {} nodes but did not find a path from {start:?} at t = {start_time} to {target:?}",
            seen.len()
        );
    }
}

pub fn star_1(data: &PuzzleData) -> usize {
    data.shortest_path((data.entry, 0), (data.exit, data.height + 1), 0)
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    let start = (data.entry, 0);
    let target = (data.exit, data.height + 1);
    let m1 = data.shortest_path(start, target, 0);
    let m2 = data.shortest_path(target, start, m1);
    data.shortest_path(start, target, m2)
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
"#;

    const CONTENT_SIMPLE: &str = r#"#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(24, data.blizzards_r.len());
        assert_eq!(24, data.blizzards_u.len());
        assert_eq!(24, data.blizzards_l.len());
        assert_eq!(24, data.blizzards_d.len());
        assert_eq!(6, data.width);
        assert_eq!(0, data.entry);
        assert_eq!(5, data.exit);
        println!("{data:?}");
    }

    #[test]
    pub fn test_blizzards_move() {
        let data = PuzzleData::from(CONTENT_SIMPLE);

        for time in 0..=12 {
            println!("t = {time}");
            data.print(data.entry, 0, time);
        }
    }

    #[test]
    pub fn test_blizzards_is_blizzard() {
        let data = PuzzleData::from(CONTENT);
        println!(
            "entry: ({}, 0), exit: ({}, {})",
            data.entry,
            data.exit,
            data.height + 1
        );

        let steps_col = [0, 0, 0, 0, 1, 1, 0, -1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0];
        let steps_row = [1, 1, 0, -1, 0, 0, 1, 0, -1, 0, 0, 1, 1, 0, 0, 0, 1, 1];
        let mut time = 0;
        let mut col: usize = 0;
        let mut row: usize = 0;
        for k in 0..steps_col.len() {
            col = col.wrapping_add_signed(steps_col[k]);
            row = row.wrapping_add_signed(steps_row[k]);
            time += 1;
            println!("t = {time}: ({col}, {row})");
            data.print(col, row, time);
            assert!(row == data.height + 1 || !data.is_blizzard((col, row - 1), time));
        }
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(18, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(54, star_2(&data));
    }
}
// end::tests[]
