use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 3,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(8_072),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(2_567),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        pub rucksacks: Vec<Vec<u8>>,
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = &'static str;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            s.lines()
                .map(|l| {
                    l.as_bytes()
                        .iter()
                        .map(|&b| match b {
                            b if b'a' <= b && b <= b'z' => Ok(b - b'a' + 1),
                            b if b'A' <= b && b <= b'Z' => Ok(b - b'A' + 27),
                            _ => Err("Unexpected bytes in input"),
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|rucksacks| Self { rucksacks })
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    data.rucksacks
        .iter()
        .filter_map(|rucksack| {
            rucksack
                .iter()
                .find(|item| rucksack.as_slice()[rucksack.len() / 2..].contains(item))
        })
        .fold(0usize, |sum, item| sum + *item as usize)
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    data.rucksacks
        .chunks_exact(3)
        .filter_map(|group| {
            group[0]
                .iter()
                .find(|item| group[1].contains(item) && group[2].contains(item))
        })
        .fold(0usize, |sum, item| sum + *item as usize)
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(157, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(70, star_2(&data));
    }
}
// end::tests[]
