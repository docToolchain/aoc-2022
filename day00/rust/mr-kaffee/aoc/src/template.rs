use crate::{err::PuzzleError, puzzle_io::PuzzleIO};
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};

pub fn write_files<F, S>(
    path: &Path,
    input_provider: &dyn InputProvider,
    config: F,
    year: u16,
    day: u16,
    force: bool,
) -> Result<(), PuzzleError>
where
    S: AsRef<str>,
    F: FnOnce() -> S,
{
    let vars = build_var_map(config, year, day);

    if path.exists() && !force {
        return Err(PuzzleError::from(format!(
            "The target directory '{}' exists. Use the --force option to overwrite.",
            path.to_string_lossy()
        )));
    }

    let src_path = path.join("src");
    println!("Creating directories for {}", src_path.to_string_lossy());
    fs::create_dir_all(src_path.as_path())?;

    // input file from provided input
    let input = input_provider.load_input(year, day)?;
    write_file(&input, &HashMap::new(), path.join("input.txt").as_path())?;

    // other files from templates
    write_file(GITIGNORE, &vars, path.join(".gitignore").as_path())?;
    write_file(CARGO_TOML, &vars, path.join("Cargo.toml").as_path())?;
    write_file(README_ADOC, &vars, path.join("README.adoc").as_path())?;
    write_file(MAIN_RS, &vars, src_path.join("main.rs").as_path())?;
    write_file(LIB_RS, &vars, src_path.join("lib.rs").as_path())?;

    Ok(())
}

fn write_file(
    template: &str,
    vars: &HashMap<String, String>,
    path: &Path,
) -> Result<(), PuzzleError> {
    let content = replace_vars(template, vars);

    println!("Writing file {} ...", path.to_string_lossy());
    fs::write(path, content)?;

    Ok(())
}

pub trait InputProvider {
    fn load_input(&self, year: u16, day: u16) -> Result<String, PuzzleError>;
}

impl InputProvider for PuzzleIO {
    fn load_input(&self, year: u16, day: u16) -> Result<String, PuzzleError> {
        PuzzleIO::load_input(self, year, day)
    }
}

pub fn upd_files<F, S>(path: &Path, config: F, year: u16, day: u16) -> Result<(), PuzzleError>
where
    S: AsRef<str>,
    F: FnOnce() -> S,
{
    let vars = build_var_map(config, year, day);
    let separator = "INCLUDE_PUZZLES";

    let line = replace_vars(PUZZLE_FACTORY_SNIPPET, &vars);
    upd_file(separator, &line, path.join("src/main.rs").as_path())?;

    let line = replace_vars(PUZZLE_INCLUDE_SNIPPET, &vars);
    upd_file(separator, &line, path.join("Cargo.toml").as_path())?;

    Ok(())
}

fn upd_file(separator: &str, line: &str, path: &Path) -> Result<bool, PuzzleError> {
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

const MAIN_RS: &str = include_str!("../templates/_main.rs_");
const LIB_RS: &str = include_str!("../templates/_lib.rs_");
const README_ADOC: &str = include_str!("../templates/_README.adoc_");
const CARGO_TOML: &str = include_str!("../templates/_Cargo.toml_");
const GITIGNORE: &str = include_str!("../templates/_.gitignore_");
const PUZZLE_FACTORY_SNIPPET: &str = include_str!("../templates/_puzzle_factory_snippet_");
const PUZZLE_INCLUDE_SNIPPET: &str = include_str!("../templates/_puzzle_include_snippet_");

/// build variables map, the function `config` is used to load custom configuration,
/// the standard use case is `config = read_config`
fn build_var_map<F, S>(config: F, year: u16, day: u16) -> HashMap<String, String>
where
    S: AsRef<str>,
    F: FnOnce() -> S,
{
    let mut vars = HashMap::from([
        ("YEAR".to_string(), year.to_string()),
        ("YEAR2".to_string(), format!("{:02}", year % 100)),
        (
            "YEAR4".to_string(),
            format!("{:04}", if year < 100 { year + 2000 } else { year }),
        ),
        ("DAY".to_string(), day.to_string()),
        ("DAY2".to_string(), format!("{day:02}")),
    ]);

    let mut configs = match serde_json::from_str::<HashMap<String, String>>(config().as_ref()) {
        Ok(configs) => configs,
        Err(err) => {
            println!("Could not parse config JSON: {err}");
            HashMap::new()
        }
    };
    for value in configs.values_mut() {
        *value = replace_vars(value, &vars);
    }

    vars.extend(configs);

    vars
}

/// replace variables in given template
fn replace_vars(template: &str, vars: &HashMap<String, String>) -> String {
    let mut content = template.to_string();
    for (name, value) in vars {
        content = content.replace(format!("{{{name}}}").as_str(), value);
    }
    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;
    use std::process::Command;
    use std::str;

    struct TestInputProvider {}

    impl InputProvider for TestInputProvider {
        fn load_input(&self, year: u16, day: u16) -> Result<String, PuzzleError> {
            Ok(format!("Test input for {}/{}\n", year, day))
        }
    }

    /// create test files and execute tests and program with cargo
    #[test]
    pub fn test_write_files() {
        let target_path = Path::new("target/test_write_file");
        let input_provider = TestInputProvider {};
        let config = || r#"{"LIB_DIR": "../../"}"#.to_string();
        let year = 2022;
        let day = 25;
        let force = true;

        // write files
        let result = write_files(target_path, &input_provider, config, year, day, force);
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

    #[test]
    pub fn test_build_var_map() {
        let vars = build_var_map(
            || {
                r#"{"VAR1": "I want to solve {YEAR}/{DAY}", "VAR2": "{YEAR} {YEAR2} {YEAR4} {DAY} {DAY2}"}"#.to_string()
            },
            2025,
            2,
        );
        assert_eq!("I want to solve 2025/2", vars.get("VAR1").unwrap());
        assert_eq!("2025 25 2025 2 02", vars.get("VAR2").unwrap());
    }
}
