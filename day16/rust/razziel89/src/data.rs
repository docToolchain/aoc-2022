// tag::data[]
use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct Valve {
    pub name: u8,
    pub rate: usize,
    pub neighbours: Vec<u8>,
}

// Using u8 instead of two-character strings speeds the entire thing up by about a factor of 4. If
// you have other character combinations, simply add them here.
pub fn str_to_usize(s: &str) -> Option<u8> {
    match s {
        "AA" => Some(0),
        "AF" => Some(1),
        "AK" => Some(2),
        "BB" => Some(3),
        "BC" => Some(4),
        "BF" => Some(5),
        "BV" => Some(6),
        "CA" => Some(7),
        "CC" => Some(8),
        "CM" => Some(9),
        "DD" => Some(10),
        "DO" => Some(11),
        "DW" => Some(12),
        "EE" => Some(13),
        "EI" => Some(14),
        "EJ" => Some(15),
        "EV" => Some(16),
        "FD" => Some(17),
        "FF" => Some(18),
        "FN" => Some(19),
        "GG" => Some(20),
        "GO" => Some(21),
        "GW" => Some(22),
        "HH" => Some(23),
        "HO" => Some(24),
        "HP" => Some(25),
        "HR" => Some(26),
        "HX" => Some(27),
        "II" => Some(28),
        "IR" => Some(29),
        "JJ" => Some(30),
        "JQ" => Some(31),
        "JS" => Some(32),
        "KH" => Some(33),
        "KL" => Some(34),
        "KQ" => Some(35),
        "KX" => Some(36),
        "LR" => Some(37),
        "MS" => Some(38),
        "MW" => Some(39),
        "NB" => Some(40),
        "NC" => Some(41),
        "NQ" => Some(42),
        "OF" => Some(43),
        "OM" => Some(44),
        "OQ" => Some(45),
        "OX" => Some(46),
        "PC" => Some(47),
        "PD" => Some(48),
        "PH" => Some(49),
        "PU" => Some(50),
        "QE" => Some(51),
        "RX" => Some(52),
        "RZ" => Some(53),
        "SG" => Some(54),
        "SM" => Some(55),
        "SY" => Some(56),
        "TN" => Some(57),
        "TS" => Some(58),
        "TY" => Some(59),
        "UE" => Some(60),
        "VL" => Some(61),
        "WE" => Some(62),
        "WU" => Some(63),
        "WW" => Some(64),
        "XG" => Some(65),
        "XN" => Some(66),
        "YD" => Some(67),
        "YQ" => Some(68),
        "ZQ" => Some(69),
        "ZX" => Some(70),
        _ => None,
    }
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let name;
        let rate: usize;
        let neighbours: Vec<_>;

        match s.split(";").collect::<Vec<_>>().as_slice() {
            [valve_data, tunnel_data] => {
                match valve_data.split_whitespace().collect::<Vec<_>>().as_slice() {
                    ["Valve", id, "has", "flow", rate_str] => {
                        name =
                            str_to_usize(id).ok_or(Error::msg("cannot convert chars to usize"))?;
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
                let maybe_neighbours = tunnel_data
                    .split_whitespace()
                    .skip(4)
                    .map(|el| str_to_usize(el.trim().trim_end_matches(",")))
                    .collect::<Vec<_>>();
                // We let this panic if it wants to.
                neighbours = maybe_neighbours
                    .into_iter()
                    .map(|el| el.unwrap())
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
