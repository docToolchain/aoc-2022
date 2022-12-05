use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, String, String, String, String> {
    Puzzle {
        year: 2022,
        day: 5,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some("ZRLJGSCTR".to_string()),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some("PRTTGRFPB".to_string()),
        }),
    }
}

// tag::input[]
pub mod input {
    use mr_kaffee_aoc::err::PuzzleError;

    #[derive(Debug, PartialEq, Eq)]
    pub struct PuzzleData {
        pub crates: Vec<Vec<char>>,
        pub moves: Vec<(usize, usize, usize)>,
    }

    fn parse_move(mv: &str) -> Result<(usize, usize, usize), PuzzleError> {
        let mut parts = mv.split(" ");

        parts.next(); // skip "move"
        let n = parts
            .next()
            .ok_or_else(|| format!("Missing number in move '{mv}'"))?
            .parse::<usize>()?;
        parts.next(); // skip "from"
        let f = parts
            .next()
            .ok_or_else(|| format!("Missing from in move '{mv}'"))?
            .parse::<usize>()?
            - 1;
        parts.next(); // skip "to"
        let t = parts
            .next()
            .ok_or_else(|| format!("Missing to in move '{mv}'"))?
            .parse::<usize>()?
            - 1;

        Ok((n, f, t))
    }

    fn parse_crate_layer(crates: &mut Vec<Vec<char>>, layer: &str) -> Result<(), PuzzleError> {
        for (k, c) in layer.chars().skip(1).step_by(4).enumerate() {
            if k >= crates.len() {
                return Err(format!("Inconsistent layer length in '{layer}'").into());
            }
            if c != ' ' {
                crates[k].push(c);
            }
        }
        Ok(())
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = PuzzleError;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            let (crates_part, moves) = s
                .split_once("\n\n")
                .ok_or("Could not separate crates from moves")?;

            let len = (crates_part.lines().next().ok_or("No crates found")?.len() + 1) / 4;

            let mut crates: Vec<Vec<char>> = vec![vec![]; len];
            for layer in crates_part.lines().rev().skip(1) {
                parse_crate_layer(&mut crates, layer)?;
            }

            let moves = moves
                .lines()
                .map(parse_move)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(PuzzleData { crates, moves })
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> String {
    let mut crates = data.crates.clone();
    for (n, f, t) in &data.moves {
        for _ in 0..*n {
            let c = crates[*f].pop().unwrap();
            crates[*t].push(c);
        }
    }

    crates.iter().map(|c| c.last().unwrap()).collect()
}
// end::star_1[]

// tag::star_2[]
fn mut_references<T>(v: &mut Vec<T>, idx1: usize, idx2: usize) -> (&mut T, &mut T) {
    if idx1 > idx2 {
        let (left, right) = v.split_at_mut(idx1);
        (&mut right[0], &mut left[idx2])
    } else {
        let (left, right) = v.split_at_mut(idx2);
        (&mut left[idx1], &mut right[0])
    }
}

pub fn star_2(data: &PuzzleData) -> String {
    let mut crates = data.crates.clone();
    for (n, f, t) in &data.moves {
        // I need a mutable reference to the from and the to part at the same time
        // to avoid creating intermediate storage
        let (fr, to) = mut_references(&mut crates, *f, *t);

        let len = fr.len();
        for c in fr.drain(len - n..) {
            to.push(c);
        }
    }

    crates.iter().map(|c| c.last().unwrap()).collect()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    pub fn test_parse() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(
            PuzzleData {
                crates: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                moves: vec![(1, 1, 0), (3, 0, 2), (2, 1, 0), (1, 0, 1)]
            },
            data
        );
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!("CMZ", star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!("MCD", star_2(&data));
    }
}
// end::tests[]
