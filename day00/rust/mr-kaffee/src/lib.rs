use mr_kaffee_aoc::{Puzzle, Star};

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

impl From<&'static str> for PuzzleData {
    fn from(input: &'static str) -> Self {
        Self { input }
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
    use super::*;
    use mr_kaffee_aoc::GenericPuzzle;

    #[test]
    pub fn test_something() {
        let puzzle = puzzle();
        assert!(puzzle.solve_handle_err());
    }
}
// end::tests[]
