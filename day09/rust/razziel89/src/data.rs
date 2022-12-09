// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

    pub fn neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    fn len(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }

    fn is_eukledian_unit(&self) -> bool {
        self.len() == 1
    }

    fn as_manhattan_unit_clamp(&self) -> Self {
        Self {
            x: self.x.clamp(-1, 1),
            y: self.y.clamp(-1, 1),
        }
    }

    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn mv(&mut self, other: &Vec) -> Result<()> {
        if other.is_eukledian_unit() {
            *self = self.add(other);
            Ok(())
        } else {
            Err(Error::msg(format!("cannot move by {:?}", other)))
        }
    }

    pub fn iter(&self) -> std::vec::IntoIter<Self> {
        let unit = self.as_manhattan_unit_clamp();
        let mut pos = self.as_manhattan_unit_clamp();
        let mut disps = vec![];

        while &pos != self {
            let new_pos = pos.add(&unit);
            disps.push(unit.clone());
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

        if &other.add(&diff.as_manhattan_unit_clamp()) == self {
            NULL_VEC.clone()
        } else {
            diff.as_manhattan_unit_clamp()
        }
    }
}

// end::data[]
