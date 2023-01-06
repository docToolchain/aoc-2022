// tag::data[]
use anyhow::{Error, Result};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: u8,
    pub y: u8,
    pub t: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Free,
    Wall,
    Blizzard(Direction),
}

pub struct Blizzard {
    start: Point,
    direction: Direction,
    min_x: u8,
    min_y: u8,
    max_y: u16,
    max_x: u16,
}

impl Blizzard {
    pub fn new(start: Point, direction: Direction, min: Point, max: Point) -> Self {
        Self {
            start: Point {
                x: start.x - min.x,
                y: start.y - min.y,
                t: 0,
            },
            direction,
            min_x: min.x,
            min_y: min.y,
            max_x: (max.x - min.x) as u16,
            max_y: (max.y - min.y) as u16,
        }
    }

    pub fn at_time(&self, t: u16) -> Point {
        match self.direction {
            Direction::Up => Point {
                x: self.start.x + self.min_x,
                y: ((self.start.y as u16 + self.max_y - t % self.max_y) % self.max_y) as u8
                    + self.min_y,
                t,
            },
            Direction::Down => Point {
                x: self.start.x + self.min_x,
                y: ((self.start.y as u16 + t % self.max_y) % self.max_y) as u8 + self.min_y,
                t,
            },
            Direction::Left => Point {
                x: ((self.start.x as u16 + self.max_x - t % self.max_x) % self.max_x) as u8
                    + self.min_x,
                y: self.start.y + self.min_y,
                t,
            },
            Direction::Right => Point {
                x: ((self.start.x as u16 + t % self.max_x) % self.max_x) as u8 + self.min_x,
                y: self.start.y + self.min_y,
                t,
            },
        }
    }

    pub fn as_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Point {
    pub fn new(x: isize, y: isize, t: isize) -> Self {
        Self {
            x: x as u8,
            y: y as u8,
            t: t as u16,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            t: self.t,
        }
    }

    pub fn env(&self) -> Vec<Self> {
        let mut result = Vec::<_>::with_capacity(5);
        result.push(
            // Waiting.
            Self {
                x: self.x,
                y: self.y,
                t: self.t + 1,
            },
        );
        if self.x > 0 {
            result.push(
                // Moving left.
                Self {
                    x: self.x - 1,
                    y: self.y,
                    t: self.t + 1,
                },
            );
        }
        result.push(
            // Moving right.
            Self {
                x: self.x + 1,
                y: self.y,
                t: self.t + 1,
            },
        );
        if self.y > 0 {
            result.push(
                // Moving up.
                Self {
                    x: self.x,
                    y: self.y - 1,
                    t: self.t + 1,
                },
            );
        }
        result.push(
            // Moving down.
            Self {
                x: self.x,
                y: self.y + 1,
                t: self.t + 1,
            },
        );
        result
    }

    pub fn as_node(&self, time: Option<u16>) -> Node {
        let mut result = Node { p: *self };
        if let Some(t) = time {
            result.p.t = t
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub p: Point,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err(Error::msg("cannot parse as tile")),
        }
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "." => Ok(Self::Free),
            "#" => Ok(Self::Wall),
            _ => Ok(Self::Blizzard(s.parse()?)),
        }
    }
}

impl Node {
    pub fn pos(&self) -> Point {
        self.p
    }

    pub fn neighbours<'a>(
        &'a self,
        blizz: &Vec<Blizzard>,
        spaces: &HashSet<Node>,
    ) -> HashSet<Point> {
        let occupied = blizz
            .iter()
            .map(|el| el.at_time(self.p.t + 1))
            .collect::<HashSet<_>>();

        // Neighbours are points in the environment that are not occupied in the next turn.
        self.p
            .env()
            .into_iter()
            .filter(|el| !occupied.contains(el) && spaces.contains(&el.as_node(Some(0))))
            .collect::<HashSet<_>>()
    }

    pub fn infinity_dist(&self, other: &Point) -> usize {
        let x_diff = if other.x > self.p.x {
            other.x - self.p.x
        } else {
            self.p.x - other.x
        } as usize;
        let y_diff = if other.y > self.p.y {
            other.y - self.p.y
        } else {
            self.p.y - other.y
        } as usize;
        x_diff + y_diff
    }

    pub fn shift(&self, time: u16) -> Node {
        Self {
            p: Point {
                x: self.p.x,
                y: self.p.y,
                t: time,
            },
        }
    }
}

// We identify a node only by its position.
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.p.hash(state)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Eq for Node {}
// end::data[]
