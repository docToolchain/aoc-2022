// tag::data[]
use anyhow::{Error, Result};
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug)]
pub struct Input {
    pub left: Pkg,
    pub right: Pkg,
}

#[derive(Debug)]
pub struct Pkg(Vec<Elem>);

#[derive(Debug)]
pub enum Elem {
    Num(isize),
    Dat(Box<Pkg>),
}

impl Input {
    pub fn is_ordered_correctly(&self) -> bool {
        self.left.compare(&self.right) == Ordering::Less
    }
}

impl Pkg {
    // This is run on the left value with the right value as argument.
    pub fn compare(&self, other: &Self) -> Ordering {
        // The zip operator will end the iteration as soon as one of the two iterators runs out.
        for (left, right) in self.0.iter().zip(other.0.iter()) {
            match left.compare(&right) {
                Ordering::Equal => {}
                ord @ Ordering::Less | ord @ Ordering::Greater => return ord,
            }
        }

        // If we reach here, all value comparisons turned out equal so far. Thus, perform the
        // length comparison.
        self.0.len().cmp(&other.0.len())
    }
}

impl Elem {
    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            // If both are numbers, compare the numbers.
            (&Elem::Num(left), &Elem::Num(right)) => left.cmp(&right),
            // If both are lists, compare the lists.
            (&Elem::Dat(ref left), &Elem::Dat(ref right)) => left.compare(&right),
            // If one is a number and the other one is a list, wrap the number in the list and
            // comapre again.
            (&Elem::Num(left), &Elem::Dat(ref right)) => Pkg(vec![Elem::Num(left)]).compare(&right),
            (&Elem::Dat(ref left), &Elem::Num(right)) => left.compare(&Pkg(vec![Elem::Num(right)])),
        }
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.split("\n");
        let left = lines
            .next()
            .ok_or(Error::msg("cannot find left line"))?
            .parse::<Pkg>()?;
        let right = lines
            .next()
            .ok_or(Error::msg("cannot find right line"))?
            .parse::<Pkg>()?;

        Ok(Self { left, right })
    }
}

// This one is not pretty but it works and correctly reports errors.
impl FromStr for Pkg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut nesting_level: usize = 0;
        let mut chars = String::new();

        if !s.starts_with("[") || !s.ends_with("]") {
            return Err(Error::msg("string is no real package"));
        }

        let mut elems_at_level = vec![];

        // Skip the opening bracket and extract all elements at this level of the hierarchy. This
        // is not pretty but seems to work.
        for char in s[1..s.len()].chars() {
            let val = match char {
                '[' => {
                    // Since we skip the very first "[", this indicates the start of a nested list.
                    chars.push(char);
                    nesting_level += 1;
                    None
                }
                ']' => {
                    if nesting_level == 0 {
                        // Emit what we found so far. This will be the very last element. We use
                        // this closing bracket to ensure we do emit the very last value.
                        let result = chars;
                        chars = String::new();
                        Some(result)
                    } else {
                        // We found the end of a nested list. Remember the current character.
                        chars.push(char);
                        nesting_level -= 1;
                        None
                    }
                }
                ',' => {
                    if nesting_level == 0 {
                        // Emit one value. This is one element of the list at this level.
                        let result = chars;
                        chars = String::new();
                        Some(result)
                    } else {
                        // We are still not at the top nesting level.
                        chars.push(char);
                        None
                    }
                }
                _ => {
                    // Remember all other characters.
                    chars.push(char);
                    None
                }
            };

            if let Some(entry) = val {
                elems_at_level.push(entry);
            }
        }

        // Parse all entries at this level of the hierarchy into "Elem"s. Errors are being handled
        // further down.
        let maybe_parsed_elems = elems_at_level
            .into_iter()
            .map(|el| el.parse::<Elem>())
            .collect::<Vec<_>>();

        let mut has_err = false;
        for el in &maybe_parsed_elems {
            if let Err(err) = el {
                has_err = true;
                eprintln!("{:?}", err);
            }
        }

        if has_err {
            Err(Error::msg("cannot parse package"))
        } else {
            let parsed_elems = maybe_parsed_elems
                .into_iter()
                // This line can never panic.
                .map(|el| el.unwrap())
                .collect::<Vec<_>>();

            Ok(Self(parsed_elems))
        }
    }
}

impl FromStr for Elem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 0 {
            // Empty package.
            Ok(Self::Dat(Box::new(Pkg(vec![]))))
        } else if s.starts_with("[") {
            // This is itself a non-empty package.
            Ok(Self::Dat(Box::new(s.parse::<Pkg>()?)))
        } else {
            // Otherwise, this is a number.
            Ok(Self::Num(s.parse::<isize>()?))
        }
    }
}
// end::data[]
