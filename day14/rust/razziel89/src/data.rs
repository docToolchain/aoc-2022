// tag::data[]
use anyhow::{Error, Result};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
pub struct Rocks {
    pub edges: Vec<Point>,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Self) -> Self {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    fn contains(&self, other: &Self) -> bool {
        if other.x == 0 && other.y == 0 {
            true
        } else if self.x != 0 {
            other.y == 0
                && self.x.clamp(-1, 1) == other.x.clamp(-1, 1)
                && self.x.abs() >= other.x.abs()
        } else {
            other.x == 0
                && self.y.clamp(-1, 1) == other.y.clamp(-1, 1)
                && self.y.abs() >= other.y.abs()
        }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left_down(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    pub fn right_down(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

impl Rocks {
    pub fn contains(&self, p: Point) -> bool {
        for (left, right) in self.edges.iter().zip(self.edges.iter().skip(1)) {
            let edge_diff = right.dist(left);
            let point_diff = p.dist(left);
            if edge_diff.contains(&point_diff) {
                return true;
            }
        }
        false
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split(",").collect::<Vec<_>>().as_slice() {
            [x, y] => Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
            }),
            _ => Err(Error::msg("cannot parse point")),
        }
    }
}

impl FromStr for Rocks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let maybe_edges = s
            .split(" -> ")
            .map(|el| el.parse::<Point>())
            .collect::<Vec<_>>();

        let mut has_err = false;
        for edge in &maybe_edges {
            if let Err(err) = edge {
                eprintln!("{:?}", err);
                has_err = true;
            }
        }

        if has_err {
            Err(Error::msg("cannot parse as edges"))
        } else {
            let edges = maybe_edges
                .into_iter()
                .map(|el| el.unwrap())
                .collect::<Vec<_>>();

            // Check whether edges go diagonally.
            for (left, right) in edges.iter().zip(edges.iter().skip(1)) {
                let edge_diff = right.dist(left);
                if edge_diff.x != 0 && edge_diff.y != 0 {
                    return Err(Error::msg("found an edge that isn't straight"));
                }
            }

            Ok(Self { edges })
        }
    }
}

// end::data[]
