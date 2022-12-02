// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub enum RPS {
    R,
    P,
    S,
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

// This trait is used so that we don't have to care which round we're scoring.
pub trait Round {
    fn score(&self) -> (usize, usize);
}

#[derive(Debug)]
pub struct RoundPart1 {
    other: RPS,
    me: RPS,
}

#[derive(Debug)]
pub struct RoundPart2 {
    other: RPS,
    outcome: Outcome,
}

impl FromStr for RPS {
    type Err = Error;

    // This parser can be used for rounds 1 and 2.
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" | "X" => Ok(RPS::R),
            "B" | "Y" => Ok(RPS::P),
            "C" | "Z" => Ok(RPS::S),
            _ => Err(Error::msg(format!("cannot parse {} as RPS", s))),
        }
    }
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(Error::msg(format!("cannot parse {} as Outcome", s))),
        }
    }
}

impl FromStr for RoundPart1 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            [other, me] => {
                // Other error.
                if !["A", "B", "C"].contains(other) {
                    Err(Error::msg(format!("unknown value {} for other", other)))
                // Me error.
                } else if !["X", "Y", "Z"].contains(me) {
                    Err(Error::msg(format!("unknown value {} for me", me)))
                // Success case.
                } else {
                    Ok(RoundPart1 {
                        other: other.parse::<RPS>()?,
                        me: me.parse::<RPS>()?,
                    })
                }
            }
            _ => Err(Error::msg(format!("cannot parse {}", s))),
        }
    }
}

impl FromStr for RoundPart2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            [other, result] => {
                // Other error.
                if !["A", "B", "C"].contains(other) {
                    Err(Error::msg(format!("unknown value {} for other", other)))
                // Success case.
                } else {
                    Ok(RoundPart2 {
                        other: other.parse::<RPS>()?,
                        outcome: result.parse::<Outcome>()?,
                    })
                }
            }
            _ => Err(Error::msg(format!("cannot parse {}", s))),
        }
    }
}

impl Outcome {
    fn score(&self) -> usize {
        match &self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl RPS {
    fn score(&self) -> usize {
        match self {
            RPS::R => 1,
            RPS::P => 2,
            RPS::S => 3,
        }
    }

    // Needed for round 1. This function could benefit from testing so that we know the result if
    // called with (a, b) and (b, a) make sense.
    fn check_win(&self, other: &RPS) -> Outcome {
        match (self, other) {
            // Self rock.
            (RPS::R, RPS::R) => Outcome::Draw,
            (RPS::R, RPS::P) => Outcome::Loss,
            (RPS::R, RPS::S) => Outcome::Win,
            // Self paper.
            (RPS::P, RPS::R) => Outcome::Win,
            (RPS::P, RPS::P) => Outcome::Draw,
            (RPS::P, RPS::S) => Outcome::Loss,
            // Self scissors.
            (RPS::S, RPS::R) => Outcome::Loss,
            (RPS::S, RPS::P) => Outcome::Win,
            (RPS::S, RPS::S) => Outcome::Draw,
        }
    }

    // Needed for round 2. This is called on the other's value with a desired outcome.
    fn get_reply(&self, result: &Outcome) -> RPS {
        match (self, result) {
            // Rock.
            (RPS::R, Outcome::Loss) => RPS::S,
            (RPS::R, Outcome::Draw) => RPS::R,
            (RPS::R, Outcome::Win) => RPS::P,
            // Paper.
            (RPS::P, Outcome::Loss) => RPS::R,
            (RPS::P, Outcome::Draw) => RPS::P,
            (RPS::P, Outcome::Win) => RPS::S,
            // Scissors.
            (RPS::S, Outcome::Loss) => RPS::P,
            (RPS::S, Outcome::Draw) => RPS::S,
            (RPS::S, Outcome::Win) => RPS::R,
        }
    }
}

// It turns out we do not need to track the other's score, but we only knew that after the fact...
impl Round for RoundPart1 {
    fn score(&self) -> (usize, usize) {
        (
            self.other.score() + self.other.check_win(&self.me).score(),
            self.me.score() + self.me.check_win(&self.other).score(),
        )
    }
}

impl Round for RoundPart2 {
    fn score(&self) -> (usize, usize) {
        let reply = self.other.get_reply(&self.outcome);
        (
            self.other.score() + self.other.check_win(&reply).score(),
            reply.score() + reply.check_win(&self.other).score(),
        )
    }
}
// end::data[]
