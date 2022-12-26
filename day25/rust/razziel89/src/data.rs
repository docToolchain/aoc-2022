// tag::data[]
use anyhow::{Error, Result};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Snafu(isize);

impl FromStr for Snafu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut num = 0;
        let mut place = 1;

        for c in s.trim().chars().rev() {
            num += place
                * match c {
                    '-' => -1,
                    '=' => -2,
                    '0' | '1' | '2' => String::from(c).parse::<isize>()?,
                    _ => return Err(Error::msg("unknown char in snafu conversion")),
                };
            place *= 5;
        }

        Ok(Self(num))
    }
}

impl Snafu {
    pub fn dec(&self) -> isize {
        self.0
    }

    pub fn new(val: isize) -> Self {
        Self(val)
    }

    pub fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0)
    }

    fn to_str(&self) -> Result<String> {
        let mut digits = vec![];
        // Convert to actual base 5 first.
        let mut num = self.0;
        while num > 0 {
            digits.push(num % 5);
            num /= 5;
        }
        // Now make the conversion to snafu. Do that by replacing the digits 3, 4 and 5 by their
        // snafu equivalents. Do so as long as there have been incorrect digits here.
        for idx in 0..digits.len() {
            match digits[idx] {
                dig @ 3 | dig @ 4 | dig @ 5 => {
                    digits[idx] = dig - 5;
                    if digits.len() - 1 < idx + 1 {
                        digits.push(0);
                    }
                    digits[idx + 1] += 1;
                }
                // Don't do anything.
                _ => {}
            }
        }

        let mut has_err = false;
        // Now create the printable representation.
        let str_rep = digits
            .into_iter()
            .rev()
            .map(|el| match el {
                0 | 1 | 2 => format!("{}", el),
                -1 => format!("-"),
                -2 => format!("="),
                _ => {
                    has_err = true;
                    String::new()
                }
            })
            .collect::<Vec<_>>()
            .join("");

        if has_err {
            Err(Error::msg("cannot print snafu"))
        } else {
            Ok(format!("{}", str_rep))
        }
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(val) = self.to_str() {
            write!(f, "{}", val)
        } else {
            Err(fmt::Error)
        }
    }
}
// end::data[]
