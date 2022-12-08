// tag::data[]
use crate::io;
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Tree(u8);

impl FromStr for Tree {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // We increase the size by one to be able to perform simple unsigned size comparisons.
        Ok(Self(
            s.parse::<u8>()?
                .checked_add(1)
                .ok_or(Error::msg("cannot increase tree size"))?,
        ))
    }
}

impl Tree {
    pub fn size(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vec {
    x: i64,
    y: i64,
}

impl Vec {
    pub fn add(&self, other: &Vec) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn pos(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

// end::data[]
