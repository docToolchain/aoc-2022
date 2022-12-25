use clap::Parser;
use itertools::Itertools;
use mr_kaffee_aoc::GenericPuzzle;
use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    // parse command line
    let cli = cli::Cli::parse();
    match cli.command {
        Some(cli::Commands::Run(run)) => exec_run(run),
        None => exec_run(cli::Run {
            years: cli::Filter::Range(2015..=2022),
            days: cli::Filter::Range(0..=25),
        }),
    };

    Ok(())
}

fn puzzles() -> Vec<Box<dyn GenericPuzzle>> {
    let mut puzzles: Vec<Box<dyn GenericPuzzle>> = vec![
        Box::new(mr_kaffee_2022_0::puzzle()),
        // INCLUDE_PUZZLES:START
        Box::new(mr_kaffee_2022_25::puzzle()),
        Box::new(mr_kaffee_2022_24::puzzle()),
        Box::new(mr_kaffee_2022_23::puzzle()),
        Box::new(mr_kaffee_2022_22::puzzle()),
        Box::new(mr_kaffee_2022_21::puzzle()),
        Box::new(mr_kaffee_2022_20::puzzle()),
        Box::new(mr_kaffee_2022_19::puzzle()),
        Box::new(mr_kaffee_2022_18::puzzle()),
        Box::new(mr_kaffee_2022_17::puzzle()),
        Box::new(mr_kaffee_2022_16::puzzle()),
        Box::new(mr_kaffee_2022_15::puzzle()),
        Box::new(mr_kaffee_2022_14::puzzle()),
        Box::new(mr_kaffee_2022_13::puzzle()),
        Box::new(mr_kaffee_2022_12::puzzle()),
        Box::new(mr_kaffee_2022_11::puzzle()),
        Box::new(mr_kaffee_2022_10::puzzle()),
        Box::new(mr_kaffee_2022_9::puzzle()),
        Box::new(mr_kaffee_2022_8::puzzle()),
        Box::new(mr_kaffee_2022_7::puzzle()),
        Box::new(mr_kaffee_2022_6::puzzle()),
        Box::new(mr_kaffee_2022_5::puzzle()),
        Box::new(mr_kaffee_2022_4::puzzle()),
        Box::new(mr_kaffee_2022_3::puzzle()),
        Box::new(mr_kaffee_2022_2::puzzle()),
        Box::new(mr_kaffee_2022_1::puzzle()),
        // INCLUDE_PUZZLES:END
    ];

    puzzles.sort_unstable_by(|p1, p2| (p1.year(), p1.day()).cmp(&(p2.year(), p2.day())));

    puzzles
}

fn exec_run(run: cli::Run) {
    // get and sort puzzles
    let puzzles = puzzles();

    // run puzzles grouped by year
    let timer = Instant::now();
    let (cnt, oks): (usize, usize) = puzzles
        .into_iter()
        .filter(|puzzle| run.years.accept(puzzle.year()) && run.days.accept(puzzle.day()))
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

mod cli {
    use clap::{Args, Parser, Subcommand};
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{ops::RangeInclusive, str::FromStr};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None, propagate_version = true)]
    pub(crate) struct Cli {
        #[command(subcommand)]
        pub(crate) command: Option<Commands>,
    }

    #[derive(Subcommand, Debug)]
    pub(crate) enum Commands {
        /// runs puzzles
        Run(Run),
    }

    #[derive(Args, Debug)]
    pub(crate) struct Run {
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
