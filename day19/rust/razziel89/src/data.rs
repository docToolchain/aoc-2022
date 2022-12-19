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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub time: Size,
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
    pub fn start(available_time: Size) -> Self {
        Self {
            time: available_time,
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

    pub fn next(&self, bp: &Blueprint, act: &WhatToBuild) -> Option<Self> {
        // Only perform the drawn operation if we have enough resources for it. Otherwise,
        // implicitly perform a "nothing" operation.
        match act {
            WhatToBuild::Nothing => {}
            WhatToBuild::OreR => {
                if self.ore < bp.ore_ore_cost {
                    return None;
                }
            }
            WhatToBuild::ClayR => {
                if self.ore < bp.clay_ore_cost {
                    return None;
                }
            }
            WhatToBuild::ObsidianR => {
                if self.ore < bp.obsidian_ore_cost || self.clay < bp.obsidian_clay_cost {
                    return None;
                }
            }
            WhatToBuild::GeodeR => {
                if self.ore < bp.geode_ore_cost || self.obsidian < bp.geode_obsidian_cost {
                    return None;
                }
            }
        }
        let mut next = Self {
            time: self.time - 1,
            // Materials.
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geode: self.geode + self.geode_robots,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
        };

        match act {
            WhatToBuild::Nothing => {}
            WhatToBuild::OreR => {
                next.ore_robots += 1;
                next.ore -= bp.ore_ore_cost;
            }
            WhatToBuild::ClayR => {
                next.clay_robots += 1;
                next.ore -= bp.clay_ore_cost;
            }
            WhatToBuild::ObsidianR => {
                next.obsidian_robots += 1;
                next.ore -= bp.obsidian_ore_cost;
                next.clay -= bp.obsidian_clay_cost;
            }
            WhatToBuild::GeodeR => {
                next.geode_robots += 1;
                next.ore -= bp.geode_ore_cost;
                next.obsidian -= bp.geode_obsidian_cost;
            }
        }

        Some(next)
    }
}
// end::data[]
