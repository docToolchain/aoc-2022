use std::collections::HashMap;

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData<'static>, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 17,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(3_055),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(1_507_692_307_690),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData<'a> {
        jets: &'a [u8],
    }

    impl<'a> From<&'a str> for PuzzleData<'a> {
        /// parse the puzzle input
        fn from(s: &'a str) -> Self {
            Self {
                jets: s.trim().as_bytes(),
            }
        }
    }

    impl<'a> PuzzleData<'a> {
        pub fn jets(&self) -> &'a [u8] {
            &self.jets
        }
    }
}
// end::input[]

// tag::ring[]
pub struct RingBuffer<'a, T> {
    data: &'a [T],
    pos: usize,
}

impl<'a, T> From<&'a [T]> for RingBuffer<'a, T> {
    fn from(data: &'a [T]) -> Self {
        Self { data, pos: 0 }
    }
}

impl<T> RingBuffer<'_, T> {
    pub fn next(&mut self) -> &T {
        let v = &self.data[self.pos];
        self.pos = (self.pos + 1) % self.data.len();
        v
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}
// end::ring[]

// tag::chamber[]
pub struct Chamber<'a> {
    chamber: Vec<u8>,
    jets: RingBuffer<'a, u8>,
    rocks: RingBuffer<'static, (&'static str, usize)>,
}

impl<'a> From<&PuzzleData<'a>> for Chamber<'a> {
    fn from(data: &PuzzleData<'a>) -> Self {
        Self {
            chamber: Vec::new(),
            jets: data.jets().into(),
            rocks: Self::ROCKS.into(),
        }
    }
}

impl Chamber<'_> {
    const WIDTH: usize = 7;
    const ROCKS: &'static [(&'static str, usize)] = &[
        ("####", 4),
        (".#.###.#.", 3),
        ("###..#..#", 3),
        ("####", 1),
        ("####", 2),
    ];
    const X0: usize = 2;
    const DY0: usize = 3;

    pub fn check(&self, rock: &[u8], x: usize, y: usize, w: usize) -> bool {
        (0..rock.len())
            .filter(|&k| rock[k] == b'#')
            .filter(|k| y + k / w < self.height())
            .all(|k| self.chamber[x + k % w + (y + k / w) * Self::WIDTH] == b'.')
    }

    pub fn height(&self) -> usize {
        self.chamber.len() / Self::WIDTH
    }

    pub fn top(&self, rows: usize) -> (Vec<u8>, usize, usize) {
        (
            self.chamber[self.chamber.len() - rows * Self::WIDTH..].to_vec(),
            self.rocks.pos(),
            self.jets.pos(),
        )
    }

    pub fn integrate_rock<F>(&mut self, f: F)
    where
        F: Fn(&[u8], &[u8], usize, usize, usize),
    {
        let &(rock, w) = self.rocks.next();
        let rock = rock.as_bytes();

        let mut x = Self::X0;
        let mut y = self.height() + Self::DY0;

        let mut stop = false;

        while !stop {
            f(&self.chamber, &rock, x, y, w);
            let &jet = self.jets.next();
            x = if jet == b'<' && x > 0 && self.check(rock, x - 1, y, w) {
                x - 1
            } else if jet == b'>' && x + w < Chamber::WIDTH && self.check(rock, x + 1, y, w) {
                x + 1
            } else {
                x
            };

            if y == 0 {
                stop = true;
            } else if y <= self.height() {
                if self.check(rock, x, y - 1, w) {
                    y -= 1;
                } else {
                    stop = true;
                }
            } else {
                y -= 1;
            }

            if stop {
                // add new lines to chamber
                while self.height() < y + rock.len() / w {
                    self.chamber.extend([b'.'; Self::WIDTH])
                }
                // update chamber
                for k in (0..rock.len()).filter(|&k| rock[k] == b'#') {
                    self.chamber[x + k % w + (y + k / w) * Self::WIDTH] = rock[k];
                }
            }
        }
    }
}
// end::chamber[]

// tag::display[]
pub struct RockInChamber<'a, 'b> {
    pub chamber: &'a [u8],
    pub rock: &'b [u8],
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub rock_part: usize,
    pub print_lim: usize,
}

impl Default for RockInChamber<'_, '_> {
    fn default() -> Self {
        Self {
            chamber: &[],
            rock: &[],
            x: 0,
            y: 0,
            w: 1,
            rock_part: 8,
            print_lim: 17,
        }
    }
}

impl std::fmt::Display for RockInChamber<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = self.chamber.len() / Chamber::WIDTH;
        let y_mx = h + self.rock_part;
        let y_mn = 0.max(h - self.print_lim.min(h));
        for y_ in (y_mn..y_mx).rev() {
            '|'.fmt(f)?;
            for x_ in 0..Chamber::WIDTH {
                if x_ >= self.x
                    && x_ < self.x + self.w
                    && y_ >= self.y
                    && y_ < self.y + self.rock.len() / self.w
                    && self.rock[x_ - self.x + (y_ - self.y) * self.w] == b'#'
                {
                    '@'.fmt(f)?;
                } else if y_ < h {
                    (self.chamber[x_ + y_ * Chamber::WIDTH] as char).fmt(f)?;
                } else {
                    '.'.fmt(f)?;
                }
            }
            "|\n".fmt(f)?;
        }
        if y_mn == 0 {
            '+'.fmt(f)?;
            for _ in 0..Chamber::WIDTH {
                '-'.fmt(f)?;
            }
            "+".fmt(f)?;
        } else {
            "|~y=".fmt(f)?;
            (y_mn - 1).fmt(f)?;
            "~".fmt(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Chamber<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        RockInChamber {
            chamber: &self.chamber,
            ..RockInChamber::default()
        }
        .fmt(f)
    }
}
// end::display[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    let mut chamber = Chamber::from(data);

    for _ in 0..2022 {
        chamber.integrate_rock(|_, _, _, _, _| ());
    }

    chamber.height()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    let mut chamber = Chamber::from(data);

    let mut seen = HashMap::new();
    let rows = 30; // this is somehow arbitrary

    let rounds: usize = 1_000_000_000_000;

    let mut cnt = 0;
    while chamber.height() < rows {
        chamber.integrate_rock(|_, _, _, _, _| ());
        cnt += 1;
    }

    for cur in cnt..rounds {
        if let Some((prev, prev_height)) = seen.insert(chamber.top(rows), (cur, chamber.height())) {
            let d_round = cur - prev;
            let d_height = chamber.height() - prev_height;

            let n = (rounds - cur) / d_round;
            let h = n * d_height;

            let rem = (rounds - cur) % d_round;
            for _ in 0..rem {
                chamber.integrate_rock(|_, _, _, _, _| ());
            }

            return chamber.height() + h;
        }
        chamber.integrate_rock(|_, _, _, _, _| ());
    }

    unreachable!()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(3_068, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(1_514_285_714_288, star_2(&data));
    }
}
// end::tests[]
