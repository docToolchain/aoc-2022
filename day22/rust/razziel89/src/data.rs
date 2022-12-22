// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
pub enum Action {
    Left,
    Right,
    Move(usize),
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Tile {
    Free,
    Wall,
    None,
}

#[derive(Debug)]
pub struct Actor {
    pos: Point,
    dir: Direction,
}

const UP: Point = Point { x: 0, y: 1 };
const DOWN: Point = Point { x: 0, y: -1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn env(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x + 1,
                y: self.y,
            },
            Self {
                x: self.x,
                y: self.y - 1,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    pub fn inv(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "L" => Ok(Action::Left),
            "R" => Ok(Action::Right),
            _ => Ok(Action::Move(s.parse()?)),
        }
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "." => Ok(Self::Free),
            "#" => Ok(Self::Wall),
            " " => Ok(Self::None),
            _ => Err(Error::msg("cannot parse as tile")),
        }
    }
}
// end::data[]
