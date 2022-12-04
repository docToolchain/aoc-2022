// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct GenPair<T, const SEP: char> {
    pub left: T,
    pub right: T,
}

#[derive(Debug)]
pub struct Num(usize);

impl FromStr for Num {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.parse::<usize>()?))
    }
}

impl<T, const SEP: char> FromStr for GenPair<T, { SEP }>
where
    T: FromStr<Err = Error>,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split(SEP).collect::<Vec<_>>().as_slice() {
            [left_str, right_str] => Ok(Self {
                left: left_str.parse::<T>()?,
                right: right_str.parse::<T>()?,
            }),
            _ => Err(Error::msg(format!(
                "cannot parse {} as pair with sep {}",
                s, SEP
            ))),
        }
    }
}

pub type Range = GenPair<Num, '-'>;
pub type Pair = GenPair<Range, ','>;

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.left.0 <= other.left.0 && self.right.0 >= other.right.0
    }

    fn overlap(&self, other: &Range) -> bool {
        (self.left.0 <= other.right.0 && self.left.0 >= other.left.0)
            || (self.right.0 >= other.left.0 && self.right.0 <= other.right.0)
    }
}

impl Pair {
    pub fn full_overlap(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    pub fn partial_overlap(&self) -> bool {
        self.left.overlap(&self.right) || self.right.overlap(&self.left)
    }
}
// end::data[]
