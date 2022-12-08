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
        trees: &'static [u8],
        width: usize,
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = Infallible;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            let trees = s.as_bytes();
            let width = s.find('\n').unwrap();
            Ok(PuzzleData { trees, width })
        }
    }

    impl PuzzleData {
        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            // 1 is added to len to work independently of a trailing new line
            (self.trees.len() + 1) / (self.width + 1)
        }

        pub fn trees(&self) -> &[u8] {
            self.trees
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn is_visible(x: usize, y: usize, trees: &[u8], width: usize, height: usize) -> bool {
    let h = trees[x + (width + 1) * y];
    (0..x).all(|x| trees[x + (width + 1) * y] < h)
        || (x + 1..width).all(|x| trees[x + (width + 1) * y] < h)
        || (0..y).all(|y| trees[x + (width + 1) * y] < h)
        || (y + 1..height).all(|y| trees[x + (width + 1) * y] < h)
}

pub fn star_1(data: &PuzzleData) -> usize {
    (0..data.width())
        .map(|x| {
            (0..data.height())
                .filter(|&y| is_visible(x, y, data.trees(), data.width(), data.height()))
                .count()
        })
        .sum::<usize>()
}
// end::star_1[]

// tag::star_2[]
pub fn scenic_score(x: usize, y: usize, trees: &[u8], width: usize, height: usize) -> usize {
    let h = trees[x + (width + 1) * y];
    let left = x
        - (0..x)
            .rev()
            .find(|x| trees[x + (width + 1) * y] >= h)
            .unwrap_or(0);
    let right = (x + 1..width)
        .find(|x| trees[x + (width + 1) * y] >= h)
        .unwrap_or(width - 1)
        - x;
    let top = y
        - (0..y)
            .rev()
            .find(|y| trees[x + (width + 1) * y] >= h)
            .unwrap_or(0);
    let bottom = (y + 1..height)
        .find(|y| trees[x + (width + 1) * y] >= h)
        .unwrap_or(height - 1)
        - y;

    left * right * top * bottom
}

pub fn star_2(data: &PuzzleData) -> usize {
    (0..data.width())
        .map(|x| {
            (0..data.height())
                .map(|y| scenic_score(x, y, data.trees(), data.width(), data.height()))
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
        assert_eq!(
            4,
            scenic_score(2, 1, data.trees(), data.width(), data.height())
        );
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(8, star_2(&data));
    }
}
// end::tests[]
