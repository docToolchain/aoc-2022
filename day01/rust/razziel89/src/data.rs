// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub enum Baggage {
    Calories(u64),
    EndOfElf,
}

#[derive(Debug)]
pub struct Elf {
    idx: usize,
    baggage: Vec<Baggage>,
}

impl FromStr for Baggage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "" => Ok(Baggage::EndOfElf),
            val => Ok(Baggage::Calories(val.parse::<u64>()?)),
        }
    }
}

impl Elf {
    pub fn total_calories(&self) -> u64 {
        self.baggage
            .iter()
            .map(|el| match el {
                Baggage::Calories(val) => val,
                Baggage::EndOfElf => &0,
            })
            .sum()
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }
}

pub fn baggages_to_elves(baggage: Vec<Baggage>) -> Vec<Elf> {
    let mut elves = vec![];
    let mut elf = Elf {
        idx: 1,
        baggage: vec![],
    };

    for el in baggage {
        match el {
            Baggage::Calories(_) => {
                elf.baggage.push(el);
            }
            Baggage::EndOfElf => {
                let next_elf = Elf {
                    idx: elf.idx + 1,
                    baggage: vec![],
                };
                elves.push(elf);
                elf = next_elf;
            }
        }
    }

    elves
}
// end::data[]
