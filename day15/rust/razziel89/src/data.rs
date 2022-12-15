// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Diamond {
    pub x: isize,
    pub y: isize,
    pub bx: isize,
    pub by: isize,
    pub dist: isize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Range {
    pub left: isize,
    pub right: isize,
}

impl Range {
    pub fn clamp(&self, min: isize, max: isize) -> Range {
        if self.right <= min || self.left >= max {
            NULL_RANGE
        } else {
            Range {
                left: self.left.clamp(min, max),
                right: self.right.clamp(min, max),
            }
        }
    }
}

pub const NULL_RANGE: Range = Range { left: 0, right: 0 };

impl FromStr for Diamond {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Sensor", "at", sensor_x, sensor_y, "closest", "beacon", "is", "at", beacon_x, beacon_y] =>
            {
                if !sensor_x.starts_with("x=") || !sensor_y.starts_with("y=") {
                    return Err(Error::msg("malformed sensor coordinates"));
                }
                if !beacon_x.starts_with("x=") || !beacon_y.starts_with("y=") {
                    return Err(Error::msg("malformed beacon coordinates"));
                }
                let conv = |val: &str| {
                    val.trim_start_matches("x=")
                        .trim_start_matches("y=")
                        .trim_end_matches(",")
                        .trim_end_matches(":")
                        .parse::<isize>()
                };
                let x = conv(sensor_x)?;
                let y = conv(sensor_y)?;
                let bx = conv(beacon_x)?;
                let by = conv(beacon_y)?;
                let dist = (bx - x).abs() + (by - y).abs();
                if dist > 0 {
                    Ok(Self { x, y, bx, by, dist })
                } else {
                    Err(Error::msg("negative distance encountered"))
                }
            }
            _ => Err(Error::msg("cannot parse point")),
        }
    }
}

impl Diamond {
    pub fn xs_at_y(&self, y: &isize) -> std::ops::Range<isize> {
        let dist = (y - self.y).abs();
        if dist <= self.dist {
            let remaining = self.dist - dist;
            self.x - remaining..self.x + remaining + 1
        } else {
            0..0
        }
    }

    pub fn xrange_at_y(&self, y: &isize) -> Range {
        let dist = (y - self.y).abs();
        if dist <= self.dist {
            let remaining = self.dist - dist;
            Range {
                left: self.x - remaining,
                right: self.x + remaining + 1,
            }
        } else {
            NULL_RANGE
        }
    }
}
// end::data[]
