// tag::data[]
use anyhow::{Error, Result};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Diamond {
    pub x: isize,
    pub y: isize,
    pub bx: isize,
    pub by: isize,
    pub dist: isize,
}

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
        let remaining = self.dist - dist;
        if dist <= self.dist {
            self.x - remaining..self.x + remaining + 1
        } else {
            0..0
        }
    }

    // This has turned out to be more efficient than always computing the full manhattan ditance.
    pub fn contains(&self, x: &isize, y: &isize) -> bool {
        let dx = (x - self.x).abs();
        if dx > self.dist {
            false
        } else {
            let dy = (y - self.y).abs();
            if dy > self.dist {
                false
            } else {
                dx + dy <= self.dist
            }
        }
    }

    // Returns true if this diamond completely encloses another one.
    pub fn encompasses(&self, other: &Self) -> bool {
        // If the other zone's beacon and centre are in this zone, then this zone fully contains
        // the other zone.
        self.contains(&other.x, &other.y) && self.contains(&other.bx, &other.by)
    }
}
// end::data[]
