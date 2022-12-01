use mr_kaffee_aoc::{Puzzle, Star};
use std::convert::Infallible;

// tag::definition[]
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 0,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Hello World example",
            f: &star_1,
            exp: Some(0),
        }),
        star2: None,
    }
}
// end::definition[]

// tag::data-structures[]
pub struct PuzzleData {
    input: &'static str,
}

impl TryFrom<&'static str> for PuzzleData {
    type Error = Infallible;

    fn try_from(input: &'static str) -> Result<Self, Self::Error> {
        Ok(Self { input })
    }
}
//end::data-structures[]

// tag::solution[]
pub fn star_1(data: &PuzzleData) -> usize {
    println!("{}", data.input);
    0
}
// end::solution[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use mr_kaffee_aoc::GenericPuzzle;

    use super::*;

    #[test]
    pub fn test_something() {
        let puzzle = puzzle();
        assert!(puzzle.solve_handle_err());
    }
}
// end::tests[]
