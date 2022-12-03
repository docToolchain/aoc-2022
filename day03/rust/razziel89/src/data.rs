// tag::data[]
use anyhow::{Error, Result};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub struct Rucksack {
    pub left: HashSet<char>,
    pub right: HashSet<char>,
}

impl Rucksack {
    pub fn everything(&self) -> HashSet<char> {
        &self.left | &self.right
    }
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let left = str_to_set(&s[..s.len() / 2]);
        let right = str_to_set(&s[s.len() / 2..]);
        Ok(Rucksack { left, right })
    }
}

fn str_to_set(s: &str) -> HashSet<char> {
    let mut set = HashSet::with_capacity(s.len());
    for c in s.chars() {
        set.insert(c);
    }
    set
}
// end::data[]
