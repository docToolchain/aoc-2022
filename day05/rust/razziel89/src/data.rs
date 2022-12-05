// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Move {
    pub num: usize,
    pub src: usize,
    pub dest: usize,
}

// We are using out own stack type here just so that the code is easier to read.
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
                    .expect("all numbers shal be >1"),
                dest: dest
                    .parse::<usize>()?
                    .checked_sub(1)
                    .expect("all numbers shal be >1"),
            }),
            _ => Err(Error::msg(format!("cannot parse {} as move", s))),
        }
    }
}

impl FromStr for StackLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            stacks: s
                .chars()
                .collect::<Vec<_>>()
                // Chunking is important here. Each stack entry contains at most 4 characters.
                // Thuds, by chunking this way, we make sure to get one chunk per stack. Luckily,
                // none of the stacks contains multi-letter crates ^^.
                .chunks(4)
                .map(|el| match el {
                    // Case with data.
                    ['[', ch, ']', ' '] | ['[', ch, ']'] => Some(ch.clone()),
                    // Case without data.
                    [' ', ' ', ' ', ' '] | [' ', ' ', ' '] => None,
                    // Error case. We only print the error to stderr this time.
                    _ => {
                        eprintln!("cannot parse line {} as stack line", s);
                        None
                    }
                })
                .collect::<Vec<_>>(),
        })
    }
}
// end::data[]
