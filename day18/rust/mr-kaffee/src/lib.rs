use std::collections::{HashSet, VecDeque};

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 18,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            #[cfg(not(feature = "pairwise"))]
            f: &star_1_traversal,
            #[cfg(feature = "pairwise")]
            f: &star_1_pairwise_comp,
            exp: Some(4_288),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(2_494),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        cubes: Vec<(isize, isize, isize)>,
    }

    impl<'a> From<&'a str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &'a str) -> Self {
            Self {
                cubes: s
                    .lines()
                    .map(|line| line.split(','))
                    .map(|mut split| {
                        (
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                        )
                    })
                    .collect(),
            }
        }
    }

    impl<'a> PuzzleData {
        pub fn cubes(&self) -> &[(isize, isize, isize)] {
            &self.cubes
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1_pairwise_comp(data: &PuzzleData) -> usize {
    let cubes = data.cubes();

    let mut sides = vec![0b111111u8; cubes.len()];

    for k1 in 0..cubes.len() - 1 {
        let (x_1, y_1, z_1) = cubes[k1];
        for k2 in k1 + 1..cubes.len() {
            let (x_2, y_2, z_2) = cubes[k2];
            if x_1 == x_2 + 1 && y_1 == y_2 && z_1 == z_2 {
                sides[k1] &= !(1 << 0);
                sides[k2] &= !(1 << 1);
            } else if x_1 == x_2 - 1 && y_1 == y_2 && z_1 == z_2 {
                sides[k1] &= !(1 << 1);
                sides[k2] &= !(1 << 0);
            } else if x_1 == x_2 && y_1 == y_2 + 1 && z_1 == z_2 {
                sides[k1] &= !(1 << 2);
                sides[k2] &= !(1 << 3);
            } else if x_1 == x_2 && y_1 == y_2 - 1 && z_1 == z_2 {
                sides[k1] &= !(1 << 3);
                sides[k2] &= !(1 << 2);
            } else if x_1 == x_2 && y_1 == y_2 && z_1 == z_2 + 1 {
                sides[k1] &= !(1 << 4);
                sides[k2] &= !(1 << 5);
            } else if x_1 == x_2 && y_1 == y_2 && z_1 == z_2 - 1 {
                sides[k1] &= !(1 << 5);
                sides[k2] &= !(1 << 4);
            }
        }
    }

    sides.iter().map(|s| s.count_ones() as usize).sum()
}
// end::star_1[]

// tag::star_1_traversal[]
pub fn star_1_traversal(data: &PuzzleData) -> usize {
    let droplet: HashSet<(isize, isize, isize)> = HashSet::from_iter(data.cubes().iter().cloned());

    let mut sides = 0;

    let mut queue = VecDeque::new();
    let mut remain = droplet.clone();
    while !remain.is_empty() {
        let &start = remain.iter().next().unwrap();
        remain.remove(&start);

        queue.push_back(start);

        while let Some((x, y, z)) = queue.pop_front() {
            for a in [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ] {
                if remain.remove(&a) {
                    // cube in droplet and not yet seen
                    queue.push_back(a);
                } else if !droplet.contains(&a) {
                    // cube which is direct adjacent is not contained
                    sides += 1;
                }
            }
        }
    }

    sides
}
// end::star_1_traversal[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    let ((x_mn, y_mn, z_mn), (x_mx, y_mx, z_mx)) = data.cubes().iter().fold(
        (
            (isize::MAX, isize::MAX, isize::MAX),
            (isize::MIN, isize::MIN, isize::MIN),
        ),
        |(mn, mx), c| {
            (
                (mn.0.min(c.0 - 1), mn.1.min(c.1 - 1), mn.2.min(c.2 - 1)),
                (mx.0.max(c.0 + 1), mx.1.max(c.1 + 1), mx.2.max(c.2 + 1)),
            )
        },
    );

    let droplet: HashSet<(isize, isize, isize)> = HashSet::from_iter(data.cubes().iter().cloned());

    let mut queue = VecDeque::new();
    for x in [x_mn, x_mx] {
        for y in [y_mn, y_mx] {
            for z in [z_mn, z_mx] {
                queue.push_back((x, y, z));
            }
        }
    }

    let mut seen: HashSet<(isize, isize, isize)> = HashSet::from_iter(queue.iter().cloned());

    let mut sides = 0;

    while let Some((x, y, z)) = queue.pop_front() {
        for a in [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ]
        .into_iter()
        .filter(|&(x, y, z)| {
            x >= x_mn && y >= y_mn && z >= z_mn && x <= x_mx && y <= y_mx && z <= z_mx
        }) {
            if droplet.contains(&a) {
                sides += 1;
            } else if seen.insert(a) {
                queue.push_back(a);
            }
        }
    }

    sides
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(13, data.cubes().len());
        assert_eq!((2, 1, 2), data.cubes()[3]);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(64, star_1_pairwise_comp(&data));
        assert_eq!(64, star_1_traversal(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(58, star_2(&data));
    }
}
// end::tests[]
