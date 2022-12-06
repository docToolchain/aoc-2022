use clap::Parser;
use itertools::Itertools;
use mr_kaffee_aoc::GenericPuzzle;
use std::time::Instant;

// tag::run[]
fn main() {
    // parse commmand line
    let cli = cli::Cli::parse();

    // define puzzles
    let puzzles: &[&dyn GenericPuzzle] = &[
        &mr_kaffee_2022_0::puzzle(),
        // INCLUDE_PUZZLES:START
        &mr_kaffee_2022_6::puzzle(),
        &mr_kaffee_2022_5::puzzle(),
        &mr_kaffee_2022_4::puzzle(),
        &mr_kaffee_2022_3::puzzle(),
        &mr_kaffee_2022_2::puzzle(),
        &mr_kaffee_2022_1::puzzle(),
        // INCLUDE_PUZZLES:END
    ];

    // sort puzzles
    let mut puzzles = Vec::from(puzzles);
    puzzles.sort_by(|p1, p2| (p1.year(), p1.day()).cmp(&(p2.year(), p2.day())));

    // run puzzles grouped by year
    let timer = Instant::now();
    let (cnt, oks): (usize, usize) = puzzles
        .into_iter()
        .filter(|puzzle| cli.years.accept(puzzle.year()) && cli.days.accept(puzzle.day()))
        .group_by(|puzzle| puzzle.year())
        .into_iter()
        .fold((0, 0), |(cnt, oks), (year, puzzles)| {
            println!();
            let timer = Instant::now();
            let (sub_cnt, sub_oks) = puzzles.into_iter().fold((0, 0), |(cnt, oks), puzzle| {
                println!();
                (cnt + 1, oks + puzzle.solve_handle_err() as usize)
            });
            let d = timer.elapsed();
            println!("\n==> Solved {sub_oks} out of {sub_cnt} puzzles for {year} in {d:?}");
            (cnt + sub_cnt, oks + sub_oks)
        });
    let d = timer.elapsed();
    println!("\n====> Solved {oks} out of {cnt} puzzles in {d:?}");
}
// end::run[]

// tag::cli[]
mod cli {
    use clap::Parser;
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{ops::RangeInclusive, str::FromStr};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub(crate) struct Cli {
        #[arg(long, short, value_parser = parse_filter_non_empty, default_value_t = Filter::Range(2015..=2022))]
        pub(crate) years: Filter,

        #[arg(long, short, value_parser = parse_filter_non_empty, default_value_t = Filter::Range(0..=25))]
        pub(crate) days: Filter,
    }

    fn parse_filter_non_empty(s: &str) -> Result<Filter, String> {
        let filter: Filter = s.parse()?;

        if filter.is_empty() {
            Err("Empty filter".into())
        } else {
            Ok(filter)
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub(crate) enum Filter {
        List(Vec<u16>),
        Range(RangeInclusive<u16>),
    }

    impl std::fmt::Display for Filter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Filter::List(list) if list.is_empty() => "()".fmt(f)?,
                Filter::List(list) => {
                    list.first().unwrap().fmt(f)?;
                    for i in list.iter().skip(1) {
                        ",".fmt(f)?;
                        i.fmt(f)?;
                    }
                }
                Filter::Range(range) => {
                    range.start().fmt(f)?;
                    "..=".fmt(f)?;
                    range.end().fmt(f)?;
                }
            }
            Ok(())
        }
    }

    impl FromStr for Filter {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"^(?:(?P<s>\d+)\.\.=(?P<e>\d+)|\d+(?:,\d+)*|(?P<v>\(\)))$")
                        .unwrap();
            }

            let captures = RE
                .captures(s)
                .ok_or("Expected range <from>..=<to> or comma separated list")?;
            if let (Some(s), Some(e)) = (captures.name("s"), captures.name("e")) {
                // at this point, we are sure to have valid integers
                // errors may occur, e.g., on overflows
                let s = s.as_str().parse().map_err(|e| format!("{}", e))?;
                let e = e.as_str().parse().map_err(|e| format!("{}", e))?;
                Ok(Self::Range(s..=e))
            } else if let Some(_) = captures.name("v") {
                Ok(Self::List(vec![]))
            } else {
                // at this point, we are sure to have a comma separated list of integers
                // errors may occur, e.g., on overflows
                let items = s
                    .split(",")
                    .map(&str::parse)
                    .collect::<Result<_, _>>()
                    .map_err(|e| format!("{}", e))?;
                Ok(Self::List(items))
            }
        }
    }

    impl Filter {
        pub(crate) fn accept(&self, v: u16) -> bool {
            match self {
                Filter::List(list) => list.contains(&v),
                Filter::Range(range) => range.contains(&v),
            }
        }

        fn is_empty(&self) -> bool {
            match self {
                Filter::List(list) => list.is_empty(),
                Filter::Range(range) => range.is_empty(),
            }
        }
    }

    #[test]
    fn test_filter_display_from_str() {
        let filter = Filter::List(vec![]);
        let s = format!("{filter}");
        assert_eq!("()", s);
        assert_eq!(s.parse::<Filter>().unwrap(), filter);

        let filter = Filter::List(vec![1]);
        let s = format!("{filter}");
        assert_eq!("1", s);
        assert_eq!(s.parse::<Filter>().unwrap(), filter);

        let filter = Filter::List(vec![1, 2, 5]);
        let s = format!("{filter}");
        assert_eq!("1,2,5", s);
        assert_eq!(s.parse::<Filter>().unwrap(), filter);

        let filter = Filter::Range(2..=8);
        let s = format!("{filter}");
        assert_eq!("2..=8", s);
        assert_eq!(s.parse::<Filter>().unwrap(), filter);

        let filter = Filter::Range(8..=2);
        let s = format!("{filter}");
        assert_eq!("8..=2", s);
        assert_eq!(s.parse::<Filter>().unwrap(), filter);
    }
}
// end::cli[]
