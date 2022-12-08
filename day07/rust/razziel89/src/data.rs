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

pub struct Stack {
    it_idx: usize,
    entries: Vec<String>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            it_idx: 0,
            entries: vec![],
        }
    }

    pub fn pwd(&self) -> String {
        if self.entries.len() == 0 {
            "/".to_string()
        } else {
            format!("/{}/", self.entries.join("/"))
        }
    }

    pub fn pushd(&mut self, dir: String) {
        self.entries.push(dir);
    }

    pub fn popd(&mut self) {
        // Ignore this here.
        _ = self.entries.pop();
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Iterator for Stack {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.it_idx > self.entries.len() {
            self.it_idx = 0;
            None
        } else if self.it_idx == 0 {
            self.it_idx += 1;
            Some("/".to_string())
        } else {
            self.it_idx += 1;
            Some(format!("/{}/", self.entries[0..self.it_idx - 1].join("/")))
        }
    }
}
// end::data[]
