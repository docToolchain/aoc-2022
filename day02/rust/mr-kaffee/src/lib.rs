use input::*;
use mr_kaffee_aoc::Puzzle;

/// the puzzle
#[cfg(not(feature = "modulo"))]
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 2,
        input: include_str!("../input.txt"),
        star1: Some(mr_kaffee_aoc::Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(11_063),
        }),
        star2: Some(mr_kaffee_aoc::Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(10_349),
        }),
    }
}

#[cfg(feature = "modulo")]
pub fn puzzle() -> Puzzle<'static, alternative::PuzzleData, usize, usize, usize, usize> {
    alternative::puzzle()
}

// tag::input[]
pub mod input {

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RockPaperScissors {
        Rock,
        Paper,
        Scissors,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum XYZ {
        X,
        Y,
        Z,
    }

    #[derive(Debug)]
    pub struct PuzzleData {
        pub strategy: Vec<(RockPaperScissors, XYZ)>,
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = String;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            s.lines()
                .map(|l| {
                    l.split_once(' ')
                        .ok_or_else(|| format!("Could not parse line '{l}'"))
                        .and_then(|(a, b)| {
                            match a {
                                "A" => Ok(RockPaperScissors::Rock),
                                "B" => Ok(RockPaperScissors::Paper),
                                "C" => Ok(RockPaperScissors::Scissors),
                                _ => Err(format!("Expected one of A, B, C, found '{a}'")),
                            }
                            .and_then(|a| match b {
                                "X" => Ok((a, XYZ::X)),
                                "Y" => Ok((a, XYZ::Y)),
                                "Z" => Ok((a, XYZ::Z)),
                                _ => Err(format!("Expected one of X, Y, Z, found '{b}'")),
                            })
                        })
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|strategy| Self { strategy })
        }
    }
}
// end::input[]

impl XYZ {
    // tag::convert_star_1[]
    fn to_rock_paper_scissors(&self) -> RockPaperScissors {
        match self {
            XYZ::X => RockPaperScissors::Rock,
            XYZ::Y => RockPaperScissors::Paper,
            XYZ::Z => RockPaperScissors::Scissors,
        }
    }
    // end::convert_star_1[]

    // tag::convert_star_2[]
    fn for_result(&self, opponent: &RockPaperScissors) -> RockPaperScissors {
        match (&self, opponent) {
            (XYZ::X, RockPaperScissors::Rock) => RockPaperScissors::Scissors,
            (XYZ::X, RockPaperScissors::Paper) => RockPaperScissors::Rock,
            (XYZ::X, RockPaperScissors::Scissors) => RockPaperScissors::Paper,
            (XYZ::Y, _) => *opponent,
            (XYZ::Z, RockPaperScissors::Rock) => RockPaperScissors::Paper,
            (XYZ::Z, RockPaperScissors::Paper) => RockPaperScissors::Scissors,
            (XYZ::Z, RockPaperScissors::Scissors) => RockPaperScissors::Rock,
        }
    }
    // end::convert_star_2[]
}

impl RockPaperScissors {
    // tag::score[]
    fn result(&self, other: &Self) -> usize {
        match (self, other) {
            (RockPaperScissors::Rock, RockPaperScissors::Rock) => 1 + 3,
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => 1 + 0,
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => 1 + 6,
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => 2 + 6,
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => 2 + 3,
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => 2 + 0,
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => 3 + 0,
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => 3 + 6,
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => 3 + 3,
        }
    }
    // end::score[]
}

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    data.strategy
        .iter()
        .map(|(a, b)| b.to_rock_paper_scissors().result(a))
        .sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    data.strategy
        .iter()
        .map(|(a, b)| b.for_result(a).result(a))
        .sum()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"A Y
B X
C Z"#;

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(15, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(12, star_2(&data));
    }
}
// end::tests[]

#[cfg(feature = "modulo")]
pub mod alternative {
    use mr_kaffee_aoc::{Puzzle, Star};

    pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
        Puzzle {
            year: 2022,
            day: 2,
            input: include_str!("../input.txt"),
            star1: Some(Star {
                name: "Star 1",
                f: &star_1,
                exp: Some(11_063),
            }),
            star2: Some(Star {
                name: "Star 2",
                f: &star_2,
                exp: Some(10_349),
            }),
        }
    }

    #[derive(Debug)]
    pub struct PuzzleData {
        pub strategy: Vec<(usize, usize)>,
    }

    // tag::parse_variants[]
    impl TryFrom<&'static str> for PuzzleData {
        type Error = String;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            if cfg!(feature = "unchecked_parse") {
                Ok(Self {
                    strategy: s
                        .lines()
                        .map(str::as_bytes)
                        .map(|bytes| ((bytes[0] - b'A') as usize, (bytes[2] - b'X') as usize))
                        .collect(),
                })
            } else {
                s.lines()
                    .map(|l| {
                        l.split_once(' ')
                            .ok_or_else(|| format!("Could not parse line '{l}'"))
                            .and_then(|(a, b)| {
                                match a {
                                    "A" => Ok(0),
                                    "B" => Ok(1),
                                    "C" => Ok(2),
                                    _ => Err(format!("Expected one of A, B, C, found '{a}'")),
                                }
                                .and_then(|a| match b {
                                    "X" => Ok((a, 0)),
                                    "Y" => Ok((a, 1)),
                                    "Z" => Ok((a, 2)),
                                    _ => Err(format!("Expected one of X, Y, Z, found '{b}'")),
                                })
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(|strategy| Self { strategy })
            }
        }
    }
    // end::parse_variants[]

    // tag::alternative[]
    pub fn star_1(data: &PuzzleData) -> usize {
        // 0 rock
        // 1 paper
        // 2 scissor
        // (rock - paper + 1) % 3 = 0
        // (rock - rock + 1) % 3 = 1
        // (rock - scissor + 1) % 3 = 2
        data.strategy
            .iter()
            .map(|(a, b)| ((b + 4 - a) % 3) * 3 + (b + 1))
            .sum()
    }

    pub fn star_2(data: &PuzzleData) -> usize {
        // 0 rock, 1 paper, 2 scissor
        // 0 loose, 1 draw, 2 win
        // to loose, subtract 1 (% 3), to win add 1 (% 3)
        // play (a + b - 1) % 3 -> add this in formula for first star
        data.strategy
            .iter()
            .map(|(a, b)| b * 3 + (a + b + 2) % 3 + 1)
            .sum()
    }
    // end::alternative[]
}
