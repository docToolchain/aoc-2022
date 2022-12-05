// tag::data[]
use crate::io;
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Move {
    pub num: usize,
    pub src: usize,
    pub dest: usize,
}

// We are using our own stack type here just so that the code is easier to read.
pub type Stack = Vec<char>;

// This is a temporary data type that we use to parse each line of the top part of the input.
#[derive(Debug)]
pub struct StackLine {
    pub stacks: Vec<Option<char>>,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["move", num, "from", src, "to", dest] => Ok(Self {
                num: num.parse()?,
                // We use zero-based indexing but the example uses one-based indxing. Thus, we
                // convert here.
                src: src
                    .parse::<usize>()?
                    .checked_sub(1)
                    .ok_or(Error::msg(format!("{} is not >1", src)))?,
                dest: dest
                    .parse::<usize>()?
                    .checked_sub(1)
                    .ok_or(Error::msg(format!("{} is not >1", dest)))?,
            }),
            _ => Err(Error::msg(format!("cannot parse {} as move", s))),
        }
    }
}

// A hybrid between a result and an option.
pub enum Hybrid<T, E> {
    Some(T),
    Err(E),
    None,
}

impl FromStr for StackLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut errs = vec![];

        let stacks = s
            .chars()
            .collect::<Vec<_>>()
            // Chunking is important here. Each stack entry contains at most 4 characters.
            // Thus, by chunking this way, we make sure to get exactly one chunk per stack.
            // Luckily, none of the stacks contains multi-letter crates ^^.
            .chunks(4)
            .map(|el| match el {
                // Case with data, can be 3 or 4 characters long.
                ['[', ch, ']', ' '] | ['[', ch, ']'] => Hybrid::Some(ch.clone()),
                // Case without data.
                [' ', ' ', ' ', ' '] | [' ', ' ', ' '] => Hybrid::None,
                // Error case.
                _ => Hybrid::Err(Error::msg(format!("cannot parse line {} as stack line", s))),
            })
            .map(|el| match el {
                Hybrid::Some(val) => Some(val),
                Hybrid::Err(err) => {
                    errs.push(format!("{:?}", err));
                    None
                }
                Hybrid::None => None,
            })
            .collect::<Vec<_>>();

        io::process_remembered_errs(errs).map(|_| Self { stacks })
    }
}
// end::data[]
