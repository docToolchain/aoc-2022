// tag::data[]
use anyhow::{Context, Error, Result};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
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
}

#[derive(Copy, Clone, Debug, Hash)]
pub enum Height {
    Normal(usize),
    Start,
    End,
}

impl Height {
    pub fn height(&self) -> usize {
        match self {
            &Self::End => 25,
            &Self::Start => 0,
            &Self::Normal(val) => val,
        }
    }
}

// This one is not pretty but it works and correctly reports errors. More context can always be
// added if there are unexpected errors.
impl FromStr for Height {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 1 {
            Ok(match s {
                "S" => Self::Start,
                "E" => Self::End,
                // The next line can never panic.
                _ => Self::Normal(s.chars().next().unwrap() as usize - 'a' as usize),
            })
        } else {
            Err(Error::msg("received several characters or none"))
        }
    }
}

#[derive(Debug)]
pub struct Node {
    p: Point,
    h: Height,
    neighbours: HashSet<Point>,
}

impl Node {
    pub fn pos(&self) -> Point {
        self.p
    }

    pub fn new(p: Point, h: Height, neighbours: HashSet<Point>) -> Self {
        Self { p, h, neighbours }
    }
}

// We identify a node only by its position and never by its associated height.
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.p.hash(state)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}
impl Eq for Node {}

// end::data[]
