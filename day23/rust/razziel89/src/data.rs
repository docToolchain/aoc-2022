// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, PartialEq)]
pub enum Input {
    Elf,
    Free,
}

pub const N: Point = Point { x: 0, y: -1 };
pub const NE: Point = Point { x: 1, y: -1 };
pub const E: Point = Point { x: 1, y: 0 };
pub const SE: Point = Point { x: 1, y: 1 };
pub const S: Point = Point { x: 0, y: 1 };
pub const SW: Point = Point { x: -1, y: 1 };
pub const W: Point = Point { x: -1, y: 0 };
pub const NW: Point = Point { x: -1, y: -1 };

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
            self.add(&N),
            self.add(&NE),
            self.add(&E),
            self.add(&SE),
            self.add(&S),
            self.add(&SW),
            self.add(&W),
            self.add(&NW),
        ]
    }

    pub fn dir_env(&self, dir: &Point) -> Vec<Self> {
        match dir {
            &N => vec![self.add(&NW), self.add(&N), self.add(&NE)],
            &E => vec![self.add(&NE), self.add(&E), self.add(&SE)],
            &S => vec![self.add(&SE), self.add(&S), self.add(&SW)],
            &W => vec![self.add(&SW), self.add(&W), self.add(&NW)],
            _ => panic!("we should never propose another direction"),
        }
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "." => Ok(Self::Free),
            "#" => Ok(Self::Elf),
            _ => Err(Error::msg("cannot parse as tile")),
        }
    }
}
// end::data[]
