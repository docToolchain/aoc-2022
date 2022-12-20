// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

pub type Size = i64;

#[derive(Debug)]
pub struct Num(Size);

impl Num {
    pub fn num(&self) -> Size {
        self.0
    }
}

impl FromStr for Num {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.parse()?))
    }
}
// end::data[]
