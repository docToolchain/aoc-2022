use clap::Parser;
use mr_kaffee_aoc::{template::*, puzzle_io::PuzzleIO};
use std::{fs, io::Error, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    target_path: PathBuf,

    #[arg(short, long)]
    lib_path: PathBuf,

    #[arg(short, long, value_parser = clap::value_parser!(u16).range(2015..=2022))]
    year: u16,

    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=25))]
    day: u16,

    #[arg(short, long)]
    force: bool,

    #[arg(short, long)]
    runner_path: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let session = fs::read_to_string("session.cookie")?;
    let input_provider = PuzzleIO {
        session: session.trim(),
    };

    write_files(
        &cli.target_path,
        &cli.lib_path,
        &input_provider,
        cli.year,
        cli.day,
        cli.force,
    )?;

    if let Some(runner_path) = cli.runner_path {
        update_files(runner_path.as_path(), cli.year, cli.day)?;
    }

    Ok(())
}
