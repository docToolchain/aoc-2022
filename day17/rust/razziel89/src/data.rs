// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Push {
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
pub struct Stream {
    pub flow: Vec<Push>,
}

// The position of each rock is indicated by the point in its bottom left. There doesn't have to be
// anything there?
#[derive(Debug, Clone, PartialEq)]
pub enum Rock {
    Minus,
    Plus,
    InverseL,
    I,
    Block,
}

// LRIS stands for last rock in sequence but it's nicer to type.
pub const LRIS: Rock = Rock::Block;

#[derive(Debug, PartialEq)]
pub enum Blocked {
    Rock,
    Wall,
    None,
}

impl Pos {
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn drop(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
}

impl Push {
    pub fn apply(&self, start: &Pos) -> Pos {
        match self {
            Self::Left => Pos {
                x: -1 + start.x,
                y: start.y,
            },
            Self::Right => Pos {
                x: 1 + start.x,
                y: start.y,
            },
        }
    }
}

impl Rock {
    // Get an infinite stream of rocks in the correct order.
    pub fn infinite_stream() -> std::iter::Cycle<std::vec::IntoIter<Rock>> {
        vec![
            Self::Minus,
            Self::Plus,
            Self::InverseL,
            Self::I,
            Self::Block,
        ]
        .into_iter()
        .cycle()
    }

    // We order the returned positions in such a way that we have the highest likelihood of getting
    // a collision early on.
    pub fn occupied_fields(&self, start: &Pos) -> std::vec::IntoIter<Pos> {
        match self {
            Self::Minus => vec![
                Pos {
                    x: 0 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 3 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 1 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 2 + start.x,
                    y: 0 + start.y,
                },
            ],
            // This one has no rock at 0,0!
            Self::Plus => vec![
                Pos {
                    x: 1 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 2 + start.x,
                    y: 1 + start.y,
                },
                Pos {
                    x: 0 + start.x,
                    y: 1 + start.y,
                },
                Pos {
                    x: 1 + start.x,
                    y: 1 + start.y,
                },
                Pos {
                    x: 1 + start.x,
                    y: 2 + start.y,
                },
            ],
            Self::Block => vec![
                Pos {
                    x: 0 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 1 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 0 + start.x,
                    y: 1 + start.y,
                },
                Pos {
                    x: 1 + start.x,
                    y: 1 + start.y,
                },
            ],
            Self::I => vec![
                Pos {
                    x: 0 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 0 + start.x,
                    y: 3 + start.y,
                },
                Pos {
                    x: 0 + start.x,
                    y: 1 + start.y,
                },
                Pos {
                    x: 0 + start.x,
                    y: 2 + start.y,
                },
            ],
            Self::InverseL => vec![
                Pos {
                    x: 0 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 1 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 2 + start.x,
                    y: 0 + start.y,
                },
                Pos {
                    x: 2 + start.x,
                    y: 1 + start.y,
                },
                Pos {
                    x: 2 + start.x,
                    y: 2 + start.y,
                },
            ],
        }
        .into_iter()
    }
}

impl Stream {
    // This consumes the stream object, but we don't need it anymore.
    pub fn infinite(self) -> std::iter::Cycle<std::vec::IntoIter<Push>> {
        self.flow.into_iter().cycle()
    }
}

impl FromStr for Stream {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            flow: s
                .chars()
                .map(|el| match el {
                    '>' => Push::Right,
                    '<' => Push::Left,
                    _ => panic!("oh no, we got weird input"),
                })
                .collect(),
        })
    }
}
// end::data[]
