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
    #[derive(Debug)]
    pub struct PuzzleData {
        pub trees: &'static [u8],
        pub w: usize,
        pub h: usize,
    }

    impl From<&'static str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &'static str) -> Self {
            let trees = s.as_bytes();
            let w = s.find('\n').unwrap();
            let h = trees.len() / (w + 1);
            Self { trees, w, h }
        }
    }
}
// end::input[]

impl PuzzleData {
    // tag::is_visible[]
    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        let h = self.trees[x + (self.w + 1) * y];

        let fx = |x_: usize| self.trees[x_ + (self.w + 1) * y] < h;
        let fy = |y_: usize| self.trees[x + (self.w + 1) * y_] < h;

        (0..x).all(fx) || (x + 1..self.w).all(fx) || (0..y).all(fy) || (y + 1..self.h).all(fy)
    }
    // end::is_visible[]

    // tag::scenic_score[]
    pub fn scenic_score(&self, x: usize, y: usize) -> usize {
        let h = self.trees[x + (self.w + 1) * y];

        let fx = |&x_: &usize| self.trees[x_ + (self.w + 1) * y] >= h;
        let fy = |&y_: &usize| self.trees[x + (self.w + 1) * y_] >= h;

        (x - (0..x).rev().find(fx).unwrap_or(0))
            * ((x + 1..self.w).find(fx).unwrap_or(self.w - 1) - x)
            * (y - (0..y).rev().find(fy).unwrap_or(0))
            * ((y + 1..self.h).find(fy).unwrap_or(self.h - 1) - y)
    }
    // end::scenic_score[]
}

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    (0..data.w)
        .map(|x| (0..data.h).filter(|&y| data.is_visible(x, y)).count())
        .sum::<usize>()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    (0..data.w)
        .map(|x| (0..data.h).map(|y| data.scenic_score(x, y)).max().unwrap())
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
35390
"#;

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(21, star_1(&data));
    }

    #[test]
    pub fn test_scenic_score() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(4, data.scenic_score(2, 1));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(8, star_2(&data));
    }
}
// end::tests[]
