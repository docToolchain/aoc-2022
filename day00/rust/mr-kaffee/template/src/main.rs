use clap::Parser;
use std::{fs, io::Error, path::PathBuf};
use mr_kaffee_template::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    target_path: PathBuf,

    #[arg(short, long)]
    lib_path: PathBuf,

    #[arg(short, long, value_parser = clap::value_parser!(u16).range(2015..=2022))]
    year: u16,

    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    #[arg(short, long)]
    force: bool,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let session = fs::read_to_string("session.cookie")?;
    let input_provider = InputLoader {
        session: session.trim(),
    };

    write_files(
        &cli.target_path,
        &cli.lib_path,
        &input_provider,
        cli.year,
        cli.day,
        cli.force,
    )
}
