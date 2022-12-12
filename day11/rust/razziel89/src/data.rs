// tag::data[]
use anyhow::{Context, Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Monkey {
    idx: usize,
    activity: usize,
    items: Vec<isize>,
    op: QuadraticOp,
    test: DivisitilibytTest,
}

impl Monkey {
    pub fn inspect_and_toss(&mut self) -> Vec<(usize, isize)> {
        let result = self
            .items
            .iter()
            .map(|item_val| {
                self.activity += 1;
                let new_item_val = self.op.apply(item_val);
                (self.test.which_monkey(&new_item_val), new_item_val)
            })
            .collect::<Vec<_>>();

        self.items = vec![];

        result
    }

    pub fn catch(&mut self, item: isize) {
        self.items.push(item);
    }

    pub fn whoami(&self) -> usize {
        self.idx
    }

    pub fn how_active(&self) -> usize {
        self.activity
    }

    pub fn set_all_divs(&mut self, prod: isize) {
        self.op.prod = Some(prod);
    }

    pub fn get_div(&self) -> isize {
        self.test.div_val
    }
}

// All operations can be realised as a*x^2 + b*x + c
#[derive(Debug)]
struct QuadraticOp {
    a: isize,
    b: isize,
    c: isize,
    prod: Option<isize>,
}

impl QuadraticOp {
    fn apply(&self, val: &isize) -> isize {
        if let Some(prod) = self.prod {
            // We update our worry level but don't divide by anything. Instead, to keep the numbers
            // small and avoid weirdness due to divisibility checks, we take the modulo with
            // respect to the product of all unique divisibility checks. Doing so never influences
            // any of the divisibility checks.
            (self.a * val * val + self.b * val + self.c) % prod
        } else {
            // We update our worry level but always divide by 3 in the end. This is for part 1.
            (self.a * val * val + self.b * val + self.c) / 3
        }
    }
}

#[derive(Debug)]
struct DivisitilibytTest {
    div_val: isize,
    true_monkey: usize,
    false_monkey: usize,
}

impl DivisitilibytTest {
    fn which_monkey(&self, val: &isize) -> usize {
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
                .map(|el| el.trim().parse::<isize>())
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
                ["old", "*", "old"] => Ok(QuadraticOp {
                    a: 1,
                    b: 0,
                    c: 0,
                    prod: None,
                }),
                ["old", "*", num] => Ok(QuadraticOp {
                    a: 0,
                    b: num.parse().context("multiplier")?,
                    c: 0,
                    prod: None,
                }),
                ["old", "+", num] => Ok(QuadraticOp {
                    a: 0,
                    b: 1,
                    c: num.parse().context("adder")?,
                    prod: None,
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

        if idx == true_monkey || idx == false_monkey {
            return Err(Error::msg("trying to toss to myself"));
        }

        if !is_prime(div_val) {
            return Err(Error::msg("div val is no prime"));
        }

        let test = DivisitilibytTest {
            div_val,
            true_monkey,
            false_monkey,
        };

        let activity = 0;

        Ok(Self {
            idx,
            activity,
            items,
            op,
            test,
        })
    }
}

// This is a quick check for being prime.
fn is_prime(val: isize) -> bool {
    for i in 2..val {
        if val % i == 0 {
            return false;
        }
    }
    true
}

// end::data[]
