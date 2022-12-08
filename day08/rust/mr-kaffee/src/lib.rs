use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 8,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(1_776),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(234_416),
        }),
    }
}

// tag::input[]
pub mod input {
    use std::convert::Infallible;

    #[derive(Debug)]
    pub struct PuzzleData {
        pub trees: &'static [u8],
        pub width: usize,
        pub height: usize,
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = Infallible;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            let trees = s.as_bytes();
            let width = s.find('\n').unwrap();
            // len + 1 is required if puzzle data has no trailing newline
            let height = (trees.len() + 1) / (width + 1);
            Ok(Self {
                trees,
                width,
                height,
            })
        }
    }
}
// end::input[]

impl PuzzleData {
    // tag::is_visible[]
    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        let h = self.trees[x + (self.width + 1) * y];
        (0..x).all(|x| self.trees[x + (self.width + 1) * y] < h)
            || (x + 1..self.width).all(|x| self.trees[x + (self.width + 1) * y] < h)
            || (0..y).all(|y| self.trees[x + (self.width + 1) * y] < h)
            || (y + 1..self.height).all(|y| self.trees[x + (self.width + 1) * y] < h)
    }
    // end::is_visible[]

    // tag::scenic_score[]
    pub fn scenic_score(&self, x: usize, y: usize) -> usize {
        let h = self.trees[x + (self.width + 1) * y];
        let left = x
            - (0..x)
                .rev()
                .find(|x| self.trees[x + (self.width + 1) * y] >= h)
                .unwrap_or(0);
        let right = (x + 1..self.width)
            .find(|x| self.trees[x + (self.width + 1) * y] >= h)
            .unwrap_or(self.width - 1)
            - x;
        let top = y
            - (0..y)
                .rev()
                .find(|y| self.trees[x + (self.width + 1) * y] >= h)
                .unwrap_or(0);
        let bottom = (y + 1..self.height)
            .find(|y| self.trees[x + (self.width + 1) * y] >= h)
            .unwrap_or(self.height - 1)
            - y;

        left * right * top * bottom
    }
    // end::scenic_score[]
}

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    (0..data.width)
        .map(|x| (0..data.height).filter(|&y| data.is_visible(x, y)).count())
        .sum::<usize>()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    (0..data.width)
        .map(|x| {
            (0..data.height)
                .map(|y| data.scenic_score(x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(21, star_1(&data));
    }

    #[test]
    pub fn test_scenic_score() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(4, data.scenic_score(2, 1));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(8, star_2(&data));
    }
}
// end::tests[]
