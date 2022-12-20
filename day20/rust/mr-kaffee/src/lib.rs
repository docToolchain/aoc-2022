use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 20,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: None,
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: None,
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        pub numbers: Vec<isize>,
        pub list: Vec<usize>,
    }

    impl From<&str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &str) -> Self {
            let numbers: Vec<isize> = s.trim().lines().map(|l| l.parse().unwrap()).collect();
            let mut list: Vec<usize> = vec![0; numbers.len()];
            for k in 0..numbers.len() {
                list[numbers[k].rem_euclid(numbers.len() as isize) as usize] =
                    numbers[(k + 1) % numbers.len()].rem_euclid(numbers.len() as isize) as usize;
            }
            Self { numbers, list }
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    data.numbers.len()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    data.numbers.len()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"1
2
-3
3
-2
0
4
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(CONTENT.len(), star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(CONTENT.len(), star_2(&data));
    }
}
// end::tests[]
