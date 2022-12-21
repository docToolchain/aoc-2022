// tag::data[]
use anyhow::{Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Monkey {
    pub name: String,
    pub action: Action,
}

#[derive(Debug)]
pub enum Action {
    Shout(isize),
    Op(String, String, char),
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            [name, num] => Ok(Self {
                name: name.trim_end_matches(":").to_string(),
                action: Action::Shout(num.parse()?),
            }),
            [name, mon1, op, mon2] => {
                if op.len() == 1 {
                    Ok(Self {
                        name: name.trim_end_matches(":").to_string(),
                        action: Action::Op(
                            mon1.to_string(),
                            mon2.to_string(),
                            op.chars().next().ok_or(Error::msg("empty op detected"))?,
                        ),
                    })
                } else {
                    Err(Error::msg("multi-char op detected"))
                }
            }
            _ => Err(Error::msg("cannot parse monkey")),
        }
    }
}
// end::data[]
