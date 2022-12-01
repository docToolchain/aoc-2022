use regex::Regex;
use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

pub fn write_files(
    target_path: &Path,
    lib_path: &Path,
    input_provider: &dyn InputProvider,
    year: u16,
    day: u8,
    force: bool,
) -> Result<(), Error> {
    let lib_path = lib_path
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Can't convert lib path to str"))?;
    let variables: HashMap<&str, &dyn Display> = HashMap::from([
        ("AOC_PATH", &lib_path as &dyn Display),
        ("YEAR", &year),
        ("DAY", &day),
    ]);

    if target_path.exists() && !force {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!(
                "The target directory '{}' exists. Use the --force option to overwrite.",
                target_path.to_string_lossy()
            ),
        ));
    }

    let target_src_path = target_path.join("src");
    println!(
        "Creating directories for {}",
        target_src_path.to_string_lossy()
    );
    fs::create_dir_all(target_src_path.as_path())?;

    // input file from web
    write_file(
        input_provider.load_input(year, day)?.as_str(),
        &HashMap::new(),
        target_path.join("input.txt").as_path(),
    )?;

    // other files from templates
    write_file(
        GITIGNORE,
        &variables,
        target_path.join(".gitignore").as_path(),
    )?;
    write_file(
        CARGO_TOML,
        &variables,
        target_path.join("Cargo.toml").as_path(),
    )?;
    write_file(
        README_ADOC,
        &variables,
        target_path.join("README.adoc").as_path(),
    )?;
    write_file(
        MAIN_RS,
        &variables,
        target_src_path.join("main.rs").as_path(),
    )?;
    write_file(LIB_RS, &variables, target_src_path.join("lib.rs").as_path())?;

    Ok(())
}

pub trait InputProvider {
    fn load_input(&self, year: u16, day: u8) -> Result<String, Error>;
}

#[derive(Debug)]
pub struct InputLoader<'a> {
    pub session: &'a str,
}

impl<'a> InputProvider for InputLoader<'a> {
    fn load_input(&self, year: u16, day: u8) -> Result<String, Error> {
        reqwest::blocking::Client::new()
            .get(format!("https://adventofcode.com/{}/day/{}/input", year, day).as_str())
            .header("Cookie", format!("session={}", self.session))
            .header("User-Agent", 
                format!(
                    "{}/{} (github.com/mr-kaffee/aoc-2022/day00/rust/mr-kaffee/template by peter@die-wielands.net)", 
                    env!("CARGO_PKG_NAME"), 
                    env!("CARGO_PKG_VERSION")
                )
            )
            .send()
            .map_err(|err| Error::new(ErrorKind::Other, err))?
            .text()
            .map_err(|err| Error::new(ErrorKind::Other, err))
    }
}

fn write_file(
    template: &str,
    variables: &HashMap<&str, &dyn Display>,
    path: &Path,
) -> Result<(), Error> {
    let mut content = template.to_string();
    for (&name, &value) in variables {
        content = content.replace(
            format!("{{{}}}", name).as_str(),
            format!("{}", value).as_str(),
        );
    }

    println!("Writing file {} ...", path.to_string_lossy());
    fs::write(path, content)?;

    Ok(())
}

pub fn update_files(runner_path: &Path, year: u16, day: u8) -> Result<(), Error> {
    update_file(
        "INCLUDE_PUZZLES",
        format!("&mr_kaffee_{year}_{day}::puzzle(),").as_str(),
        runner_path.join("src/main.rs").as_path(),
    )?;
    update_file(
        "INCLUDE_PUZZLES",
        format!(
            "mr-kaffee-{year}-{day} = {{ path = \"../../../day{day:02}/rust/mr-kaffee/\"}}"
        )
        .as_str(),
        runner_path.join("Cargo.toml").as_path(),
    )?;
    Ok(())
}

pub fn update_file(separator: &str, line: &str, path: &Path) -> Result<bool, Error> {
    println!("Updating file {} ...", path.to_string_lossy());
    let re = Regex::new(
        format!(r"(?ms:(?P<prefix>^.*{separator}:START.*?[\r\n]+)(?P<indent>\s*)(?P<data>.*?{separator}:END)(?P<suffix>.*$))")
            .as_str(),
    )
    .unwrap();

    let s = fs::read_to_string(path)?;
    if let Some(captures) = re.captures(s.as_str()) {
        // if regex matches, those groups exist
        let prefix = captures.name("prefix").unwrap().as_str();
        let indent = captures.name("indent").unwrap().as_str();
        let data = captures.name("data").unwrap().as_str();
        let suffix = captures.name("suffix").unwrap().as_str();

        if !data.contains(line) {
            let contents = format!("{prefix}{indent}{line}\n{indent}{data}{suffix}");
            fs::write(path, contents)?;
            println!("-> Updated");
            Ok(true)
        } else {
            println!("-> Nothing to update");
            Ok(false)
        }
    } else {
        println!("-> No section to update ({separator}:START ... {separator}:END) found");
        Ok(false)
    }
}

const MAIN_RS: &str = r###"use mr_kaffee_aoc::{err::PuzzleError, GenericPuzzle};
use mr_kaffee_{YEAR}_{DAY}::*;

fn main() -> Result<(), PuzzleError> {
    puzzle().solve_report_err()
}
"###;

const LIB_RS: &str = r###"use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: {YEAR},
        day: {DAY},
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: None,
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: None,
        }),
    }
}

// tag::input[]
pub mod input {
    use std::convert::Infallible;

    #[derive(Debug)]
    pub struct PuzzleData {
        pub input: &'static str,
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = Infallible;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            Ok(PuzzleData { input: s })
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    println!("{}", data.input);
    0
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    println!("{:?}", data);
    0
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;
    use mr_kaffee_aoc::err::PuzzleError;

    const CONTENT: &str = r#"Hello World!
Advent of Code 2022"#;

    #[test]
    pub fn test_puzzle_data_from_str() -> Result<(), PuzzleError> {
        let data = PuzzleData::try_from(CONTENT)?;
        assert_eq!(data.input, CONTENT.to_string());
        Ok(())
    }
}
// end::tests[]
"###;

const README_ADOC: &str = r###"== Day {DAY}: _TODO_ ==

https://rust-lang.org[Rust] solution to https://adventofcode.com/{YEAR}/day/{DAY}[AoC|{YEAR}|{DAY}].

=== Input ===

[source,rust,numbered]
----
include::src/lib.rs[tags=input]
----

=== Star 1 ===

[source,rust,numbered]
----
include::src/lib.rs[tags=star_1]
----

=== Star 2 ===

[source,rust,numbered]
----
include::src/lib.rs[tags=star_2]
----

=== Tests ===

[source,rust,numbered]
----
include::src/lib.rs[tags=tests]
----
"###;

const CARGO_TOML: &str = r###"[package]
name = "mr-kaffee-{YEAR}-{DAY}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

mr-kaffee-aoc = { path = "{AOC_PATH}" }
"###;

const GITIGNORE: &str = r###"**/target
"###;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;
    use std::process::Command;
    use std::str;

    struct TestInputProvider {}

    impl InputProvider for TestInputProvider {
        fn load_input(&self, year: u16, day: u8) -> Result<String, Error> {
            Ok(format!("Test input for {}/{}\n", year, day))
        }
    }

    /// create test files and execute tests and program with cargo
    #[test]
    pub fn test_write_files() {
        let target_path = Path::new("target/test_write_file");
        let lib_path = Path::new("../../../aoc");
        let input_provider = TestInputProvider {};
        let year = 2022;
        let day = 25;
        let force = true;

        // write files
        let result = write_files(target_path, lib_path, &input_provider, year, day, force);
        assert!(matches!(result, Ok(_)));

        // run tests using `cargo test`
        let result = Command::new("cargo")
            .arg("test")
            .current_dir(target_path)
            .output();
        assert!(
            matches!(result, Ok(_)),
            "'cargo test' did not execute successful"
        );
        let result = result.unwrap();
        println!(
            "{}",
            str::from_utf8(&result.stdout)
                .expect("Could not convert stdout of 'cargo test' to string")
        );
        assert_eq!(
            result.status.code(),
            Some(0),
            "'cargo test' returned with non-zero status"
        );

        // run program using `cargo run`
        let result = Command::new("cargo")
            .arg("run")
            .current_dir(target_path)
            .output();
        assert!(
            matches!(result, Ok(_)),
            "'cargo run' did not execute successful"
        );
        let result = result.unwrap();
        println!(
            "{}",
            str::from_utf8(&result.stdout)
                .expect("Could not convert stdout of 'cargo run' to string")
        );
        assert_eq!(
            result.status.code(),
            Some(0),
            "'cargo run' returned with non-zero status"
        );

        // clean up, if it fails, 'cargo clean' will do the job
        let _ = remove_dir_all(target_path);
    }
}
