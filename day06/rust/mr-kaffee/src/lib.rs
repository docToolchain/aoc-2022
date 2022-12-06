use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
#[cfg(not(feature = "variant"))]
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 6,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(1_912),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(2_122),
        }),
    }
}

#[cfg(feature = "variant")]
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 6,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &(|data| find_distinct(data.stream, 4)),
            exp: Some(1_912),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &(|data| find_distinct(data.stream, 14)),
            exp: Some(2_122),
        }),
    }
}

// tag::input[]
pub mod input {
    use std::convert::Infallible;

    #[derive(Debug)]
    pub struct PuzzleData {
        pub stream: &'static [u8],
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = Infallible;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            Ok(PuzzleData {
                stream: s.trim().as_bytes(),
            })
        }
    }
}
// end::input[]

// tag::generic[]
pub fn find_distinct(stream: &[u8], n: usize) -> usize {
    let mut k = 0;
    while k < stream.len() - n {
        match (k + 1..k + n).find(|p| stream[*p..k + n].contains(&stream[*p - 1])) {
            Some(q) => k = q,
            None => return k + n,
        }
    }

    panic!("No solution.");
}
// end::generic[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    let (k, _) = data
        .stream
        .windows(4)
        .enumerate()
        .find(|(_, s)| match s {
            &[a, b, c, d] => a != b && a != c && a != d && b != c && b != d && c != d,
            _ => unreachable!(),
        })
        .unwrap();

    k + 4
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    let (k, _) = data
        .stream
        .windows(14)
        .enumerate()
        .find(|(_, w)| {
            w.iter()
                .enumerate()
                .skip(1)
                .all(|(p, c1)| w.iter().take(p).all(|c2| c1 != c2))
        })
        .unwrap();
    k + 14
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(7, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(19, star_2(&data));
    }

    #[test]
    pub fn test_find_distinct() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(7, find_distinct(data.stream, 4));
        assert_eq!(19, find_distinct(data.stream, 14));
    }
}
// end::tests[]
