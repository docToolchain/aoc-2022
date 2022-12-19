// tag::data[]
use anyhow::{Error, Result};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

pub type Size = u16;

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: Size,
    pub ore_ore_cost: Size,
    pub clay_ore_cost: Size,
    pub obsidian_ore_cost: Size,
    pub obsidian_clay_cost: Size,
    pub geode_ore_cost: Size,
    pub geode_obsidian_cost: Size,
}

#[derive(Debug, PartialEq)]
pub enum WhatToBuild {
    OreR,
    ClayR,
    ObsidianR,
    GeodeR,
    Nothing,
}

// Allow drawing a random member of the enum.
impl Distribution<WhatToBuild> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WhatToBuild {
        match rng.gen_range(0..=14) {
            0 | 1 => WhatToBuild::OreR,
            2 | 3 | 4 => WhatToBuild::ClayR,
            5 | 6 | 7 | 8 => WhatToBuild::ObsidianR,
            9 | 10 | 11 | 12 | 13 => WhatToBuild::GeodeR,
            _ => WhatToBuild::Nothing,
        }
    }
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Blueprint", id, "Each", "ore", "robot", "costs", ore_ore_cost, "ore.", "Each", "clay", "robot", "costs", clay_ore_cost, "ore.", "Each", "obsidian", "robot", "costs", obsidian_ore_cost, "ore", "and", obsidian_clay_cost, "clay.", "Each", "geode", "robot", "costs", geode_ore_cost, "ore", "and", geode_obsidian_cost, "obsidian."] => {
                Ok(Self {
                    id: id.trim_end_matches(":").parse()?,
                    ore_ore_cost: ore_ore_cost.parse()?,
                    clay_ore_cost: clay_ore_cost.parse()?,
                    obsidian_ore_cost: obsidian_ore_cost.parse()?,
                    obsidian_clay_cost: obsidian_clay_cost.parse()?,
                    geode_ore_cost: geode_ore_cost.parse()?,
                    geode_obsidian_cost: geode_obsidian_cost.parse()?,
                })
            }
            _ => Err(Error::msg("cannot parse blueprint")),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub ore: Size,
    pub ore_robots: Size,
    pub clay: Size,
    pub clay_robots: Size,
    pub obsidian: Size,
    pub obsidian_robots: Size,
    pub geode: Size,
    pub geode_robots: Size,
}

impl State {
    pub fn next(&mut self, bp: &Blueprint) {
        let random_action = rand::random::<WhatToBuild>();
        // Only perform the drawn operation if we have enough resources for it. Otherwise,
        // implicitly perform a "nothing" operation.
        let can_build = match random_action {
            WhatToBuild::Nothing => false,
            WhatToBuild::OreR => self.ore >= bp.ore_ore_cost,
            WhatToBuild::ClayR => self.ore >= bp.clay_ore_cost,
            WhatToBuild::ObsidianR => {
                self.ore >= bp.obsidian_ore_cost && self.clay >= bp.obsidian_clay_cost
            }
            WhatToBuild::GeodeR => {
                self.ore >= bp.geode_ore_cost && self.obsidian >= bp.geode_obsidian_cost
            }
        };
        // Materials.
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        // New constructions. There can be at most be one robot constructed at a time. I suspect
        // it's gonna be different for part 2.
        if can_build {
            match random_action {
                WhatToBuild::Nothing => {}
                WhatToBuild::OreR => {
                    self.ore_robots += 1;
                    self.ore -= bp.ore_ore_cost;
                }
                WhatToBuild::ClayR => {
                    self.clay_robots += 1;
                    self.ore -= bp.clay_ore_cost;
                }
                WhatToBuild::ObsidianR => {
                    self.obsidian_robots += 1;
                    self.ore -= bp.obsidian_ore_cost;
                    self.clay -= bp.obsidian_clay_cost;
                }
                WhatToBuild::GeodeR => {
                    self.geode_robots += 1;
                    self.ore -= bp.geode_ore_cost;
                    self.obsidian -= bp.geode_obsidian_cost;
                }
            }
        }
    }

    pub fn start() -> Self {
        Self {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geode: 0,
            geode_robots: 0,
        }
    }
}
// end::data[]
