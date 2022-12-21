use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 19,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(1_480),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(3_168),
        }),
    }
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

// tag::input[]
pub mod input {
    use crate::{CLAY, GEODE, OBSIDIAN, ORE};

    #[derive(Debug)]
    pub struct PuzzleData {
        pub blueprints: Vec<[usize; 12]>,
    }

    fn parse_blueprint(line: &str) -> [usize; 12] {
        let mut blueprint = [0; 12];

        let mut words = line.split_ascii_whitespace().skip(6); // blueprint <id>: each ore robot costs
        blueprint[ORE + 3 * ORE] = words.next().unwrap().parse().unwrap(); // XX: ore for ore robot
        let mut words = words.skip(5); // ore. each clay robot costs
        blueprint[ORE + 3 * CLAY] = words.next().unwrap().parse().unwrap(); // XX: ore for clay robot
        let mut words = words.skip(5); // ore. each obsidian robot costs
        blueprint[ORE + 3 * OBSIDIAN] = words.next().unwrap().parse().unwrap(); // XX: ore for obsidian robot
        let mut words = words.skip(2); // ore and
        blueprint[CLAY + 3 * OBSIDIAN] = words.next().unwrap().parse().unwrap(); // XX: clay for obsidian robot
        let mut words = words.skip(5); // clay. each geode robot costs
        blueprint[ORE + 3 * GEODE] = words.next().unwrap().parse().unwrap(); // XX: ore for geode robot
        let mut words = words.skip(2); // ore and
        blueprint[OBSIDIAN + 3 * GEODE] = words.next().unwrap().parse().unwrap(); // XX: obsidian for geode robot
        assert_eq!(Some("obsidian."), words.next());
        assert_eq!(None, words.next());

        blueprint
    }

    impl From<&str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &str) -> Self {
            Self {
                blueprints: s.lines().map(parse_blueprint).collect(),
            }
        }
    }
}
// end::input[]

// tag::dfs[]
pub fn max_geodes(blueprint: &[usize], steps: usize) -> usize {
    // start with one ore robot, zero material and steps to go
    let start = ([1usize, 0, 0, 0], [0; 4], steps);

    // initialize queue
    let mut queue = Vec::from([start]);

    // maximum required amount of robots per type
    let max_req = (ORE..=OBSIDIAN).fold([usize::MAX; 4], |mut max_req, m| {
        max_req[m] = (ORE..=GEODE).map(|r| blueprint[m + 3 * r]).max().unwrap();
        max_req
    });

    // optimum
    let mut opt: usize = 0;

    while let Some((robots, materials, steps)) = queue.pop() {
        // update optimum, including geodes opened in remaining steps
        opt = opt.max(materials[GEODE] + robots[GEODE] * steps);

        // time elapsed
        if steps == 0 {
            continue;
        }

        // if geodes opened by making a geode robot in every subsequent step does not lead to
        // an improvement, stop here
        if (0..steps).fold(materials[GEODE], |bound, step| {
            bound + (robots[GEODE] + step) * (steps - step)
        }) < opt
        {
            continue;
        }

        for r in (ORE..=GEODE).rev().filter(|&r| robots[r] < max_req[r]) {
            // calculate steps required to build robot
            let Some(s) = (ORE..=OBSIDIAN)
                .map(|m| {
                    if materials[m] >= blueprint[m + 3 * r] {
                        Some(0)
                    } else if robots[m] == 0 {
                        None
                    } else {
                        Some((blueprint[m + 3 * r] - materials[m] + robots[m] - 1) / robots[m])
                    }
                })
                .fold(Some(0), |s_max, s| match (s_max, s) {
                    (Some(s_max), Some(s)) => Some(s_max.max(s)),
                    _ => None,
                }) else {
                    // can't build robot
                    continue
                };

            if s + 1 > steps {
                // time elapsed
                continue;
            }

            // update robots and materials
            let mut materials = materials;
            for m in ORE..=OBSIDIAN {
                materials[m] += (s + 1) * robots[m];
                materials[m] -= blueprint[m + 3 * r];
            }
            materials[GEODE] += (s + 1) * robots[GEODE];
            let mut robots = robots;
            robots[r] += 1;

            // push to queue
            queue.push((robots, materials, steps - s - 1));
        }
    }

    opt
}
// end::dfs[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    data.blueprints
        .iter()
        .map(|blueprint| max_geodes(blueprint, 24))
        .enumerate()
        .map(|(k, opt)| (k + 1) * opt)
        .sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    data.blueprints
        .iter()
        .take(3)
        .map(|blueprint| max_geodes(blueprint, 32))
        .product()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(9 * 1 + 12 * 2, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(56 * 62, star_2(&data));
    }
}
// end::tests[]
