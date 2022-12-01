use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 1,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(67658),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(200158),
        }),
    }
}

// tag::input[]
pub mod input {
    use std::num::ParseIntError;

    #[derive(Debug)]
    pub struct PuzzleData {
        pub calories: Vec<Vec<usize>>,
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = ParseIntError;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            s.split("\n\n")
                .map(|elf| {
                    elf.lines()
                        .map(|l| l.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|calories| Self { calories })
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    data.calories
        .iter()
        .fold(0, |mx, elf| mx.max(elf.iter().sum()))
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    let mut calories = data
        .calories
        .iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<_>>();

    calories.sort();

    calories.iter().rev().take(3).sum()
}
// end::star_2[]
