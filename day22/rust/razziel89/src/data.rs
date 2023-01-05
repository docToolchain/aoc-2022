// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
pub enum Action {
    Left,
    Right,
    Move(usize),
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Free,
    Wall,
    None,
}

#[derive(Debug)]
pub struct Neighbours {
    pub up: Point,
    pub down: Point,
    pub left: Point,
    pub right: Point,
}

#[derive(Debug)]
pub struct Actor {
    pub pos: Point,
    pub dir: Direction,
}

impl Actor {
    pub fn peek(&self, neigh: &Neighbours) -> Point {
        match self.dir {
            Direction::Left => neigh.left.clone(),
            Direction::Right => neigh.right.clone(),
            Direction::Up => neigh.up.clone(),
            Direction::Down => neigh.down.clone(),
        }
    }

    pub fn mv(&mut self, neigh: &Neighbours, next_neigh: &Neighbours) -> Result<()> {
        let next_pos = self.peek(neigh);
        let next_dir = self.next_dir(next_neigh)?;
        self.pos = next_pos;
        self.dir = next_dir;
        Ok(())
    }

    pub fn right(&mut self) {
        self.dir = match self.dir {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        };
    }

    pub fn left(&mut self) {
        self.dir = match self.dir {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
        };
    }

    pub fn num(&self) -> isize {
        match self.dir {
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Down => 1,
        }
    }

    fn next_dir(&mut self, next_neigh: &Neighbours) -> Result<Direction> {
        match &self.pos {
            p if p == &next_neigh.left => Ok(Direction::Right),
            p if p == &next_neigh.right => Ok(Direction::Left),
            p if p == &next_neigh.up => Ok(Direction::Down),
            p if p == &next_neigh.down => Ok(Direction::Up),
            _ => Err(Error::msg(
                "previous position is no neighbour of current position",
            )),
        }
    }
}

pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn env(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x + 1,
                y: self.y,
            },
            Self {
                x: self.x,
                y: self.y - 1,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    pub fn inv(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "L" => Ok(Action::Left),
            "R" => Ok(Action::Right),
            _ => Ok(Action::Move(s.parse()?)),
        }
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "." => Ok(Self::Free),
            "#" => Ok(Self::Wall),
            " " => Ok(Self::None),
            _ => Err(Error::msg("cannot parse as tile")),
        }
    }
}
// end::data[]
