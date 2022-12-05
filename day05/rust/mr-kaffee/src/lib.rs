use input::*;
use mr_kaffee_aoc::{err::PuzzleError, Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<
    'static,
    PuzzleData,
    String,
    Result<String, PuzzleError>,
    String,
    Result<String, PuzzleError>,
> {
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
        stacks: Vec<Vec<char>>,
        moves: Vec<(usize, usize, usize)>,
    }

    fn parse_move(line: &str, len: usize) -> Result<(usize, usize, usize), PuzzleError> {
        let mut parts = line.split(" ");

        parts.next(); // skip "move"
        let n = parts
            .next()
            .ok_or_else(|| format!("Missing number in move '{line}'"))?
            .parse::<usize>()?;
        parts.next(); // skip "from"
        let from = parts
            .next()
            .ok_or_else(|| format!("Missing from in move '{line}'"))?
            .parse::<usize>()?
            - 1;
        parts.next(); // skip "to"
        let to = parts
            .next()
            .ok_or_else(|| format!("Missing to in move '{line}'"))?
            .parse::<usize>()?
            - 1;

        if from >= len || to >= len {
            Err(format!("Invalid move: '{line}', <from>, <to> <= {len} required.").into())
        } else if from == to {
            Err(format!("Invalid move: '{line}', <from> != <to> required.").into())
        } else {
            Ok((n, from, to))
        }
    }

    fn parse_crate_layer(stacks: &mut Vec<Vec<char>>, line: &str) {
        for (k, item) in line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, item)| *item != ' ')
        {
            while k >= stacks.len() {
                stacks.push(Vec::new());
            }
            stacks[k].push(item);
        }
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = PuzzleError;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            let (stacks_part, moves_part) = s
                .split_once("\n\n")
                .ok_or("Could not separate crates from moves")?;

            let mut stacks: Vec<Vec<char>> = vec![];
            for line in stacks_part.lines().rev().skip(1) {
                parse_crate_layer(&mut stacks, line);
            }

            let moves = moves_part
                .lines()
                .map(|line| parse_move(line, stacks.len()))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(PuzzleData { stacks, moves })
        }
    }

    impl PuzzleData {
        /// get moves
        pub fn moves(&self) -> &[(usize, usize, usize)] {
            &self.moves
        }

        /// get cloned crates
        pub fn stacks(&self) -> Vec<Vec<char>> {
            self.stacks.clone()
        }
    }
}
// end::input[]

// tag::star_1[]
fn msg(stacks: &[Vec<char>]) -> Result<String, PuzzleError> {
    stacks
        .iter()
        .map(|c| {
            c.last().ok_or_else(|| {
                PuzzleError::from(format!(
                    "Can't construct message. Empty stack in {stacks:?}"
                ))
            })
        })
        .collect()
}

pub fn star_1(data: &PuzzleData) -> Result<String, PuzzleError> {
    let mut stacks = data.stacks();
    for (n, from, to) in data.moves() {
        for _ in 0..*n {
            let item = stacks[*from]
                .pop()
                .ok_or_else(|| format!("Tried to pop from empty stack {from}, stacks: {stacks:?}"))?;
            stacks[*to].push(item);
        }
    }

    msg(&stacks)
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

pub fn star_2(data: &PuzzleData) -> Result<String, PuzzleError> {
    let mut stacks = data.stacks();
    for (n, from, to) in data.moves() {
        // I need a mutable reference to the from and the to part at the same time
        // to avoid creating intermediate storage
        let (source, dest) = mut_references(&mut stacks, *from, *to);

        let len = source.len();
        if *n > len {
            return Err(format!(
                "Trying to pop {n} elements from stack {from} containing {len}, stacks: {stacks:?}"
            )
            .into());
        }
        for item in source.drain(len - n..) {
            dest.push(item);
        }
    }

    msg(&stacks)
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
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            data.stacks()
        );
        assert_eq!(&[(1, 1, 0), (3, 0, 2), (2, 1, 0), (1, 0, 1)], data.moves());
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!("CMZ", star_1(&data).unwrap());
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!("MCD", star_2(&data).unwrap());
    }
}
// end::tests[]
