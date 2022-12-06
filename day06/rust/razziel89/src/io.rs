// tag::io[]
use anyhow::{Context, Result};

pub fn read_lines_from_file(path: &str) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)
        .context("reading from disk")?
        .trim_end()
        .split('\n')
        .map(|el| String::from(el))
        .collect())
}
// end::io[]
