use clap::Parser;
use itertools::Itertools;
use mr_kaffee_aoc::{
    puzzle_io::{submit_result, PuzzleIO, Star},
    template::{update_files, write_files},
    GenericPuzzle,
};
use std::{error::Error, fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    // parse command line
    let cli = cli::Cli::parse();
    match cli.command {
        Some(cli::Commands::Run(run)) => exec_run(run),
        Some(cli::Commands::Init(init)) => exec_init(init)?,
        Some(cli::Commands::Submit(submit)) => exec_submit(submit)?,
        None => exec_run(cli::Run {
            years: cli::Filter::Range(2015..=2022),
            days: cli::Filter::Range(0..=25),
        }),
    };

    Ok(())
}

fn puzzles() -> Vec<Box<dyn GenericPuzzle>> {
    let puzzles: Vec<Box<dyn GenericPuzzle>> = vec![
        Box::new(mr_kaffee_2022_0::puzzle()),
        // INCLUDE_PUZZLES:START
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
    puzzles
}

fn exec_run(run: cli::Run) {
    // get and sort puzzles
    let mut puzzles = puzzles();
    puzzles.sort_by(|p1, p2| (p1.year(), p1.day()).cmp(&(p2.year(), p2.day())));

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

fn exec_init(init: cli::Init) -> Result<(), std::io::Error> {
    let session = fs::read_to_string("session.cookie")?;
    let input_provider = PuzzleIO {
        session: session.trim(),
    };

    write_files(
        &init.target_path,
        &init.lib_path,
        &input_provider,
        init.year,
        init.day,
        init.force,
    )?;

    if let Some(runner_path) = init.runner_path {
        update_files(runner_path.as_path(), init.year, init.day)?;
    }

    Ok(())
}

fn exec_submit(submit: cli::Submit) -> Result<(), Box<dyn Error>> {
    let puzzles = puzzles();
    let puzzle = puzzles
        .iter()
        .find(|puzzle| puzzle.year() == submit.year && puzzle.day() == submit.day);
    if let Some(puzzle) = puzzle {
        let (star, result) = match submit.part {
            1 => (Star::One, puzzle.solve_star_1()?),
            _ => (Star::Two, puzzle.solve_star_2()?),
        };
        if let Some(result) = result {
            submit_result(None, submit.year, submit.day, star, &result)?;
        } else {
            println!(
                "Part {} not implemented for {}/{}",
                submit.part, submit.year, submit.day
            );
        }
    } else {
        println!("No puzzle for {}/{}", submit.year, submit.day);
    }

    Ok(())
}

mod cli {
    use clap::{Args, Parser, Subcommand};
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{ops::RangeInclusive, path::PathBuf, str::FromStr};

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

        /// initializes a new puzzle from a template
        Init(Init),

        /// submits puzzle solution
        Submit(Submit),
    }

    #[derive(Args, Debug)]
    pub(crate) struct Run {
        #[arg(long, short, value_parser = parse_filter_non_empty, default_value_t = Filter::Range(2015..=2022))]
        pub(crate) years: Filter,

        #[arg(long, short, value_parser = parse_filter_non_empty, default_value_t = Filter::Range(0..=25))]
        pub(crate) days: Filter,
    }

    #[derive(Args, Debug)]
    pub(crate) struct Init {
        #[arg(short, long)]
        pub(crate) target_path: PathBuf,

        #[arg(short, long)]
        pub(crate) lib_path: PathBuf,

        #[arg(short, long, value_parser = clap::value_parser!(u16).range(2015..=2022))]
        pub(crate) year: u16,

        #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=25))]
        pub(crate) day: u16,

        #[arg(short, long)]
        pub(crate) force: bool,

        #[arg(short, long)]
        pub(crate) runner_path: Option<PathBuf>,
    }

    #[derive(Args, Debug)]
    pub(crate) struct Submit {
        #[arg(short, long, value_parser = clap::value_parser!(u16).range(2015..=2022))]
        pub(crate) year: u16,
        #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=25))]
        pub(crate) day: u16,
        #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=2))]
        pub(crate) part: u16,
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
