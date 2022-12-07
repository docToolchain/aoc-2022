// tag::data[]
use crate::io;
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub enum Entry {
    CD(String),
    LS,
    DIR(String),
    FILE { name: String, size: usize },
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["$", "cd", dir] => Ok(Self::CD(dir.to_string())),
            ["$", "ls"] => Ok(Self::LS),
            ["dir", dir] => Ok(Self::DIR(dir.to_string())),
            [size, name] => Ok(Self::FILE {
                name: name.to_string(),
                size: size.parse::<usize>()?,
            }),
            _ => Err(Error::msg(format!("canot parse {}", s))),
        }
    }
}
// end::data[]
