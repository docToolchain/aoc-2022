// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Vec {
    x: i64,
    y: i64,
}

pub const NULL_VEC: Vec = Vec { x: 0, y: 0 };

impl FromStr for Vec {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s
            .split_whitespace()
            .collect::<std::vec::Vec<_>>()
            .as_slice()
        {
            ["R", dist] => Ok(Self {
                x: dist.parse()?,
                y: 0,
            }),
            ["L", dist] => Ok(Self {
                x: -dist.parse()?,
                y: 0,
            }),
            ["U", dist] => Ok(Self {
                x: 0,
                y: dist.parse()?,
            }),
            ["D", dist] => Ok(Self {
                x: 0,
                y: -dist.parse()?,
            }),
            _ => Err(Error::msg(format!("cannot parse {} as vector", s))),
        }
    }
}

impl Vec {
    pub fn add(&self, other: &Vec) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    // Length in infinity metric.
    fn infinity_len(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }

    fn is_infinity_unit(&self) -> bool {
        self.infinity_len() == 1
    }

    // Map to the 2d unit sphere in manhattan metric. The null vector cannot be mapped and, thus,
    // remains unchanged.
    fn as_manhattan_unit(&self) -> Self {
        Self {
            x: self.x.clamp(-1, 1),
            y: self.y.clamp(-1, 1),
        }
    }

    // Move the vector exactly one space along one direction.
    pub fn mv(&mut self, other: &Vec) -> Result<()> {
        if other.is_infinity_unit() {
            *self = self.add(other);
            Ok(())
        } else {
            Err(Error::msg(format!("cannot move by {:?}", other)))
        }
    }

    // Provide an iterator over unit-sized steps (unit in manhattan metric not infinity metric)
    // that, if followed, describes the same distance traveled as `self`.
    pub fn iter(&self) -> std::vec::IntoIter<Self> {
        let unit = self.as_manhattan_unit();
        let mut pos = self.as_manhattan_unit();
        // As this is my second time working with a custom iterator, I was not sure how to avoid
        // cloning here.
        let mut disps = vec![];
        while &pos != self {
            let new_pos = pos.add(&unit);
            disps.push(unit);
            pos = new_pos;
        }
        disps.push(unit);

        disps.into_iter()
    }

    pub fn get_tail_update(&self, other: &Self) -> Self {
        let diff = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };

        // We want to update with a unit vector in manhattan metric, but only if that would not
        // mean that `other` is on the same space as `self`.
        if &other.add(&diff.as_manhattan_unit()) == self {
            NULL_VEC
        } else {
            diff.as_manhattan_unit()
        }
    }
}

// end::data[]
