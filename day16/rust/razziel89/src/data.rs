// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Valve {
    pub name: String,
    pub rate: usize,
    pub neighbours: Vec<String>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let name: String;
        let rate: usize;
        let neighbours: Vec<String>;

        match s.split(";").collect::<Vec<_>>().as_slice() {
            [valve_data, tunnel_data] => {
                match valve_data.split_whitespace().collect::<Vec<_>>().as_slice() {
                    ["Valve", id, "has", "flow", rate_str] => {
                        name = id.to_string();
                        rate = rate_str.trim_start_matches("rate=").parse()?;
                    }
                    _ => {
                        return Err(Error::msg("cannot parse valve data"));
                    }
                }
                if !tunnel_data.starts_with(" tunnels lead to valve")
                    && !tunnel_data.starts_with(" tunnel leads to valve")
                {
                    return Err(Error::msg("cannot parse tunnel data"));
                }
                neighbours = tunnel_data
                    .split_whitespace()
                    .skip(4)
                    .map(|el| el.trim().trim_end_matches(",").to_string())
                    .collect::<Vec<_>>();
                Ok(Self {
                    name,
                    rate,
                    neighbours,
                })
            }
            _ => Err(Error::msg("cannot parse valve")),
        }
    }
}
// end::data[]
