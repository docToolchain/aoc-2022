// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Pos {
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
}

impl FromStr for Pos {
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
// end::data[]
