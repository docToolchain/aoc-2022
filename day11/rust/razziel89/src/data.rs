// tag::data[]
use anyhow::{Context, Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Monkey {
    idx: usize,
    items: Vec<usize>,
    op: QuadraticOp,
    test: DivisitilibytTest,
}

// All operations can be realised as a*x^2 + b*x + c
#[derive(Debug)]
struct QuadraticOp {
    a: isize,
    b: isize,
    c: isize,
}

impl QuadraticOp {
    fn apply(&self, val: isize) -> isize {
        // We update our worry level but always divide by 3 in the end.
        (self.a * val * val + self.b * val + self.c) / (3 as isize)
    }
}

#[derive(Debug)]
struct DivisitilibytTest {
    div_val: isize,
    true_monkey: usize,
    false_monkey: usize,
}

impl DivisitilibytTest {
    fn which_monkey(&self, val: isize) -> usize {
        if val % self.div_val == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

// This one is not pretty but it works and correctly reports errors. More context can always be
// added if there are unexpected errors.
impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines = s
            .split("\n")
            .map(|el| el.trim())
            .collect::<std::vec::Vec<_>>();

        let idx = if let ["Monkey", idx_str] =
            lines[0].split_whitespace().collect::<Vec<_>>().as_slice()
        {
            idx_str.trim_end_matches(":").parse().context("monkey id")?
        } else {
            return Err(Error::msg("canot find monkey id string"));
        };

        let maybe_items = if let ["Starting items", items_str] =
            lines[1].split(":").collect::<Vec<_>>().as_slice()
        {
            items_str
                .split(", ")
                .map(|el| el.trim().parse::<usize>())
                .collect::<Vec<_>>()
        } else {
            return Err(Error::msg("canot find items line"));
        };

        if maybe_items
            .iter()
            .any(|el| if let Err(_) = el { true } else { false })
        {
            return Err(Error::msg("cannot parse items line"));
        }

        // The next line can never panic.
        let items = maybe_items
            .into_iter()
            .map(|el| el.unwrap())
            .collect::<Vec<_>>();

        let op = if let ["Operation: new ", op_str] =
            lines[2].split("=").collect::<Vec<_>>().as_slice()
        {
            match op_str.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["old", "*", "old"] => Ok(QuadraticOp { a: 1, b: 0, c: 0 }),
                ["old", "*", num] => Ok(QuadraticOp {
                    a: 0,
                    b: num.parse().context("multiplier")?,
                    c: 0,
                }),
                ["old", "+", num] => Ok(QuadraticOp {
                    a: 0,
                    b: 0,
                    c: num.parse().context("adder")?,
                }),
                _ => Err(Error::msg("cannot build op")),
            }
        } else {
            return Err(Error::msg("cannot find operations line"));
        }
        .context("operation")?;

        let div_val = if let ["Test:", "divisible", "by", val] =
            lines[3].split_whitespace().collect::<Vec<_>>().as_slice()
        {
            val.parse().context("divisibility")?
        } else {
            return Err(Error::msg("cannot find divisibility line"));
        };

        let true_monkey = if let ["If", "true:", "throw", "to", "monkey", val] =
            lines[4].split_whitespace().collect::<Vec<_>>().as_slice()
        {
            val.parse().context("true monkey")?
        } else {
            return Err(Error::msg("cannot find true monkey line"));
        };

        let false_monkey = if let ["If", "false:", "throw", "to", "monkey", val] =
            lines[5].split_whitespace().collect::<Vec<_>>().as_slice()
        {
            val.parse().context("false monkey")?
        } else {
            return Err(Error::msg("cannot find false monkey line"));
        };

        let test = DivisitilibytTest {
            div_val,
            true_monkey,
            false_monkey,
        };

        Ok(Self {
            idx,
            items,
            op,
            test,
        })
    }
}
// end::data[]
