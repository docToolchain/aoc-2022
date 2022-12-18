// tag::data[]
use anyhow::{Error, Result};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Point {
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn env(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Self {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Self {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ]
    }

    pub fn as_node(&self) -> Node {
        Node {
            p: *self,
            neighbours: HashSet::<Point>::new(),
        }
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split(",").collect::<Vec<_>>().as_slice() {
            [x, y, z] => Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
                z: z.parse()?,
            }),
            _ => Err(Error::msg("cannot parse pos")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub p: Point,
    pub neighbours: HashSet<Point>,
}

impl Node {
    pub fn pos(&self) -> Point {
        self.p
    }

    pub fn neighbours<'a>(&'a self) -> &'a HashSet<Point> {
        &self.neighbours
    }

    pub fn infinity_dist(&self, other: &Point) -> usize {
        (self.p.x - other.x).abs() as usize
            + (self.p.y - other.y).abs() as usize
            + (self.p.z - other.z).abs() as usize
    }
}

// We identify a node only by its position.
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
