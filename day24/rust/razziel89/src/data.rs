// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: u8,
    pub y: u8,
    pub t: u16,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum Tile {
    Free,
    Wall,
    Blizzard(Direction),
}

impl Point {
    pub fn new(x: isize, y: isize, t: isize) -> Self {
        Self {
            x: x as u8,
            y: y as u8,
            t: t as u16,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            t: self.t,
        }
    }

    pub fn env(&self) -> Vec<Self> {
        vec![
            // Waiting.
            Self {
                x: self.x,
                y: self.y,
                t: self.t + 1,
            },
            // Moving left.
            Self {
                x: self.x - 1,
                y: self.y,
                t: self.t + 1,
            },
            // Moving right.
            Self {
                x: self.x + 1,
                y: self.y,
                t: self.t + 1,
            },
            // Moving up.
            Self {
                x: self.x,
                y: self.y - 1,
                t: self.t + 1,
            },
            // Moving down.
            Self {
                x: self.x,
                y: self.y + 1,
                t: self.t + 1,
            },
        ]
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err(Error::msg("cannot parse as tile")),
        }
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "." => Ok(Self::Free),
            "#" => Ok(Self::Wall),
            _ => Ok(Self::Blizzard(s.parse()?)),
        }
    }
}
// end::data[]
