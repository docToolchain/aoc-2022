// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum Op {
    None,
    Some(isize),
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s
            .split_whitespace()
            .collect::<std::vec::Vec<_>>()
            .as_slice()
        {
            ["noop"] => Ok(Self::None),
            ["addx", val] => Ok(Self::Some(val.parse()?)),
            _ => Err(Error::msg(format!("cannot parse {} as op", s))),
        }
    }
}
// end::data[]
