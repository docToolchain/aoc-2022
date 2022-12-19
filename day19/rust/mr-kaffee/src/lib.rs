use std::collections::{HashSet, VecDeque};

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

type Count = usize;

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

// tag::input[]
pub mod input {
    use crate::Count;

    #[derive(Debug)]
    pub struct PuzzleData {
        blueprints: Vec<Blueprint>,
    }

    impl From<&str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &'_ str) -> Self {
            Self {
                blueprints: s.lines().map(Blueprint::from).collect(),
            }
        }
    }

    impl PuzzleData {
        pub fn blueprints(&self) -> &[Blueprint] {
            &self.blueprints
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Blueprint {
        costs: [Count; 6],
    }

    impl From<&str> for Blueprint {
        fn from(line: &str) -> Self {
            let mut costs = [0; 6];
            let mut words = line.split_ascii_whitespace().skip(6); // blueprint <id>: each ore robot costs
            costs[Self::ORE_FOR_ORE] = words.next().unwrap().parse().unwrap(); // XX: ore for ore robot
            let mut words = words.skip(5); // ore. each clay robot costs
            costs[Self::ORE_FOR_CLAY] = words.next().unwrap().parse().unwrap(); // XX: ore for clay robot
            let mut words = words.skip(5); // ore. each obsidian robot costs
            costs[Self::ORE_FOR_OBSIDIAN] = words.next().unwrap().parse().unwrap(); // XX: ore for obsidian robot
            let mut words = words.skip(2); // ore and
            costs[Self::CLAY_FOR_OBSIDIAN] = words.next().unwrap().parse().unwrap(); // XX: clay for obsidian robot
            let mut words = words.skip(5); // clay. each geode robot costs
            costs[Self::ORE_FOR_GEODE] = words.next().unwrap().parse().unwrap(); // XX: ore for geode robot
            let mut words = words.skip(2); // ore and
            costs[Self::OBSIDIAN_FOR_GEODE] = words.next().unwrap().parse().unwrap(); // XX: obsidian for geode robot
            assert_eq!(Some("obsidian."), words.next());
            assert_eq!(None, words.next());
            Self { costs }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
    pub enum Material {
        Ore = 0,
        Clay = 1,
        Obsidian = 2,
        Geode = 3,
    }

    impl Material {
        pub const ALL: &[Self] = &[Self::Ore, Self::Clay, Self::Obsidian, Self::Geode];
    }

    impl<T> From<T> for Material
    where
        T: Into<usize>,
    {
        fn from(v: T) -> Self {
            Self::ALL[v.into()]
        }
    }

    impl Blueprint {
        const ORE_FOR_ORE: usize = 0;
        const ORE_FOR_CLAY: usize = 1;
        const ORE_FOR_OBSIDIAN: usize = 2;
        const CLAY_FOR_OBSIDIAN: usize = 3;
        const ORE_FOR_GEODE: usize = 4;
        const OBSIDIAN_FOR_GEODE: usize = 5;

        pub fn can_make_robot(&self, inventory: &Inventory, material: Material) -> bool {
            match material {
                Material::Ore => {
                    inventory.material[Material::Ore as usize] >= self.costs[Self::ORE_FOR_ORE]
                }
                Material::Clay => {
                    inventory.material[Material::Ore as usize] >= self.costs[Self::ORE_FOR_CLAY]
                }
                Material::Obsidian => {
                    inventory.material[Material::Ore as usize] >= self.costs[Self::ORE_FOR_OBSIDIAN]
                        && inventory.material[Material::Clay as usize]
                            >= self.costs[Self::CLAY_FOR_OBSIDIAN]
                }
                Material::Geode => {
                    inventory.material[Material::Ore as usize] >= self.costs[Self::ORE_FOR_GEODE]
                        && inventory.material[Material::Obsidian as usize]
                            >= self.costs[Self::OBSIDIAN_FOR_GEODE]
                }
            }
        }

        pub fn make_robot(&self, inventory: &mut Inventory, material: Material) {
            match material {
                Material::Ore => {
                    inventory.material[Material::Ore as usize] -= self.costs[Self::ORE_FOR_ORE];
                    inventory.robots[Material::Ore as usize] += 1;
                }
                Material::Clay => {
                    inventory.material[Material::Ore as usize] -= self.costs[Self::ORE_FOR_CLAY];
                    inventory.robots[Material::Clay as usize] += 1;
                }
                Material::Obsidian => {
                    inventory.material[Material::Ore as usize] -=
                        self.costs[Self::ORE_FOR_OBSIDIAN];
                    inventory.material[Material::Clay as usize] -=
                        self.costs[Self::CLAY_FOR_OBSIDIAN];
                    inventory.robots[Material::Obsidian as usize] += 1;
                }
                Material::Geode => {
                    inventory.material[Material::Ore as usize] -= self.costs[Self::ORE_FOR_GEODE];
                    inventory.material[Material::Obsidian as usize] -=
                        self.costs[Self::OBSIDIAN_FOR_GEODE];
                    inventory.robots[Material::Geode as usize] += 1;
                }
            }
        }

        pub fn make_robot_if_possible(
            &self,
            inventory: &mut Inventory,
            material: Material,
        ) -> bool {
            if self.can_make_robot(inventory, material) {
                self.make_robot(inventory, material);
                true
            } else {
                false
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Inventory {
        pub robots: [Count; Material::ALL.len()],
        pub material: [Count; Material::ALL.len()],
    }

    impl Default for Inventory {
        fn default() -> Self {
            let mut robots = [0; Material::ALL.len()];
            robots[Material::Ore as usize] = 1;
            Self {
                robots,
                material: [0; Material::ALL.len()],
            }
        }
    }

    impl Inventory {
        pub fn count_geodes(&self) -> Count {
            self.material[Material::Geode as usize]
        }

        pub fn count_robots(&self) -> Count {
            self.robots.iter().sum()
        }

        pub fn collect_materials(&mut self, prev: &Self) {
            for &m in Material::ALL {
                self.material[m as usize] += prev.robots[m as usize];
            }
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn produce_robots(blueprint: &Blueprint, inventory: &Inventory) -> Vec<Inventory> {
    let mut inventories = Vec::from([*inventory]);

    for material in [
        Material::Geode,
        Material::Obsidian,
        Material::Clay,
        Material::Ore,
    ] {
        if blueprint.can_make_robot(inventory, material) {
            let mut inventory = *inventory;
            blueprint.make_robot(&mut inventory, material);
            inventories.push(inventory);
        }
    }

    inventories
}

pub fn simulate(blueprint: &Blueprint, inventory: Inventory, steps: usize) -> Count {
    let mut queue = VecDeque::from([(inventory, 0)]);

    let mut seen = HashSet::from([inventory]);

    let mut mx = 0;
    while let Some((inventory, step)) = queue.pop_front() {
        if step == steps {
            mx = mx.max(inventory.count_geodes());
            continue;
        }

        for mut adjacent in produce_robots(blueprint, &inventory) {
            adjacent.collect_materials(&inventory);
            if seen.insert(adjacent) {
                queue.push_back((adjacent, step + 1));
            }
        }
    }

    println!("{mx} is best for blueprint {blueprint:?}");

    mx
}

pub fn star_1(data: &PuzzleData) -> usize {
    data.blueprints()
        .iter()
        .map(|blueprint| simulate(blueprint, Inventory::default(), 24))
        .enumerate()
        .fold(0, |sum, (k, m)| sum + (k + 1) * m as usize)
}
// end::star_1[]

// tag::star_2[]
pub fn simulate_sequence(
    blueprint: &Blueprint,
    mut inventory: Inventory,
    steps: usize,
    robots: &[Material],
) -> Inventory {
    let mut pos = 0;

    for _step in 0..steps {
        let prev = inventory;
        if pos < robots.len() && blueprint.make_robot_if_possible(&mut inventory, robots[pos]) {
            pos += 1;
        }
        inventory.collect_materials(&prev);
        // println!("Step {_step} from {prev:?} to {inventory:?}");
    }

    inventory
}

const BLOCKS_OF_4: usize = 21;
const BLOCKS_OF_2: usize = 0;

fn robots(k: usize) -> [Material; 32] {
    [
        (k >> 40) & 3,
        (k >> 38) & 3,
        (k >> 36) & 3,
        (k >> 34) & 3,
        (k >> 32) & 3,
        (k >> 30) & 3,
        (k >> 28) & 3,
        (k >> 26) & 3,
        (k >> 24) & 3,
        (k >> 22) & 3,
        (k >> 20) & 3,
        (k >> 18) & 3,
        (k >> 16) & 3,
        (k >> 14) & 3,
        (k >> 12) & 3,
        (k >> 10) & 3,
        (k >> 8) & 3,
        (k >> 6) & 3,
        (k >> 4) & 3,
        (k >> 2) & 3,
        (k >> 0) & 3,
        3,
        3,
        3,
        3,
        3,
        3,
        3,
        3,
        3,
        3,
        3,
    ]
    .map(|v| v.into())
}

pub fn star_2(data: &PuzzleData) -> usize {
    let mut prod: usize = 1;
    for blueprint in data.blueprints().iter().take(3) {
        let mut best = 0;
        let mut best_result = Inventory::default();
        let mut best_k = 0;
        let mut k = 0;
        while k < (1 << (2 * BLOCKS_OF_4 + BLOCKS_OF_2)) {
            let result = simulate_sequence(blueprint, Inventory::default(), 32, &robots(k));
            if result.count_geodes() > best {
                best = result.count_geodes();
                best_result = result;
                best_k = k;
            }
            let sum = result.count_robots();
            if sum < BLOCKS_OF_4 {
                // skip to next robot at failed position
                k += 1 << (2 * (BLOCKS_OF_4 - sum) + BLOCKS_OF_2);
            } else if sum < BLOCKS_OF_4 + BLOCKS_OF_2 {
                // skip to next robot at failed position
                k += 1 << (BLOCKS_OF_4 + BLOCKS_OF_2 - sum);
            } else {
                k += 1;
            }
        }
        println!("{best} is best for {blueprint:?} with result {best_result:?} at {best_k:#x}");
        println!("  {:?}", robots(best_k).map(|m| m as u8));
        prod *= best as usize;
    }

    // 2050 is too low (5 41 10)
    // 2970 is too low (6 45 11)
    // 3240 is too high (6 45 12)
    // 3096 is wrong (6 43 12)

    // I guess 6 44 12 -> 3168
    prod
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
    pub fn test_simulate_search() {
        let data = PuzzleData::from(CONTENT);
        let blueprint = &data.blueprints()[1];
        println!("Blueprint: {blueprint:?}");
        let mx = simulate(blueprint, Inventory::default(), 24);
        println!("Best is {mx}");
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

    #[test]
    pub fn test_robots() {
        let k = 1 << 16;
        let r_1 = robots(k).map(|r| r as usize);
        println!("{r_1:?}");

        let k = k + (1 << (2 * (17 - 10) + 2));
        let r_2 = robots(k).map(|r| r as usize);
        println!("{r_2:?}");

        assert_eq!(r_1[9] + 1, r_2[9]);
        assert!((0..r_1.len()).filter(|&k| k != 9).all(|k| r_1[k] == r_2[k]));
    }

    #[test]
    pub fn test_simulate_sequence() {
        let data = PuzzleData::from(CONTENT);
        let seq = [
            0u8, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3,
        ]
        .map(|v| Material::from(v));
        let blueprint = data.blueprints()[0];
        let result = simulate_sequence(&blueprint, Inventory::default(), 32, &seq);
        assert_eq!(56, result.count_geodes());
        println!("{result:?}");
    }
}
// end::tests[]
