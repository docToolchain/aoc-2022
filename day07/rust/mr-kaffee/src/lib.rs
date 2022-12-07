use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 7,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(1_449_447),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(8_679_207),
        }),
    }
}

// tag::input[]
pub mod input {
    use std::{collections::HashMap, convert::Infallible};

    #[derive(Debug)]
    pub struct Directory {
        parent_dir: Option<usize>,
        child_dirs: HashMap<&'static str, usize>,
        child_file_sizes: usize,
    }

    impl Directory {
        fn new(parent_dir: Option<usize>) -> Self {
            Self {
                parent_dir,
                child_dirs: HashMap::new(),
                child_file_sizes: 0,
            }
        }

        pub fn total_size(&self, dirs: &[Directory]) -> usize {
            self.child_file_sizes
                + self
                    .child_dirs
                    .iter()
                    .map(|(_, idx)| dirs[*idx].total_size(dirs))
                    .sum::<usize>()
        }
    }

    #[derive(Debug)]
    pub struct PuzzleData {
        dirs: Vec<Directory>,
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = Infallible;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            let mut dirs = vec![Directory::new(None)];

            let mut current = 0;
            for line in s.lines() {
                match line {
                    "$ cd /" => current = 0,
                    "$ cd .." => current = dirs[current].parent_dir.unwrap(),
                    "$ ls" => (),
                    v if v.starts_with("$ cd ") => {
                        current = *dirs[current]
                            .child_dirs
                            .get(v.strip_prefix("$ cd ").unwrap())
                            .unwrap()
                    }
                    v if v.starts_with("dir ") => {
                        let idx = dirs.len();
                        dirs.push(Directory::new(Some(current)));
                        dirs[current]
                            .child_dirs
                            .insert(v.strip_prefix("dir ").unwrap(), idx);
                    }
                    v => {
                        let (size, _) = v.split_once(" ").unwrap();
                        dirs[current].child_file_sizes += size.parse::<usize>().unwrap()
                    }
                }
            }

            Ok(Self { dirs })
        }
    }

    impl PuzzleData {
        /// immutable access to directories as slice
        pub fn dirs(&self) -> &[Directory] {
            &self.dirs
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    data.dirs()
        .iter()
        .filter_map(|d| {
            let s = d.total_size(&data.dirs());
            if s <= 100000 {
                Some(s)
            } else {
                None
            }
        })
        .sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    let required = data.dirs()[0].total_size(&data.dirs()) - 40_000_000;

    data.dirs()
        .iter()
        .filter_map(|d| {
            let s = d.total_size(&data.dirs());
            if s >= required {
                Some(s)
            } else {
                None
            }
        })
        .fold(usize::MAX, |mn, s| mn.min(s))
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(95_437, star_1(&data))
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::try_from(CONTENT).unwrap();
        assert_eq!(24_933_642, star_2(&data))
    }
}
// end::tests[]
