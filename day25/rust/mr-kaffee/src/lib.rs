use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, &'static str, String, String, usize, usize> {
    Puzzle {
        year: 2022,
        day: 25,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some("2-1=10=1=1==2-1=-221".into()),
        }),
        star2: None,
    }
}

// tag::star_1[]
pub fn star_1(data: &&str) -> String {
    // convert SNAFU to decimal and sum
    let mut sum: isize = data
        .lines()
        .map(|l| {
            l.as_bytes().iter().fold(0, |value, &digit| {
                5 * value
                    + match digit {
                        b'2' => 2,
                        b'1' => 1,
                        b'0' => 0,
                        b'-' => -1,
                        b'=' => -2,
                        _ => unreachable!(),
                    }
            })
        })
        .sum();
    println!("The sum is {sum}");

    // convert decimal to SNAFU
    let mut digits = Vec::new();
    while sum != 0 {
        let v = sum % 5;
        sum /= 5;
        if v > 2 {
            sum += 1;
        }
        digits.push(match v {
            3 => '=',
            4 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        });
    }
    digits.iter().rev().collect()
}
// end::star_1[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"#;

    #[test]
    pub fn test_star_1() {
        assert_eq!("2=-1=0", star_1(&CONTENT));
    }
}
// end::tests[]
