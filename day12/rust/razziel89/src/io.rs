// tag::io[]
use crate::data;
use anyhow::{Context, Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

pub fn read_lines_from_file(path: &str, chunk_size: usize) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)
        .context("reading from disk")?
        .trim_end()
        .split('\n')
        .map(|el| String::from(el))
        .collect::<Vec<_>>()
        .as_slice()
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec().join("\n"))
        .collect::<Vec<_>>())
}

pub fn parse_chars_to_data<T>(file: &str, type_name: &str) -> Result<HashMap<data::Point, T>>
where
    T: FromStr<Err = Error>,
{
    let mut errs: Vec<String> = vec![];
    let mut data = HashMap::<data::Point, T>::new();

    for (line_idx, line) in read_lines_from_file(file, 1)?.into_iter().enumerate() {
        for (col_idx, character) in line.chars().enumerate() {
            match character.to_string().parse::<T>().with_context(|| {
                format!(
                    "cannot parse char in line {} and row {} as {}: {}",
                    line_idx, col_idx, type_name, character
                )
            }) {
                Ok(val) => {
                    // We should never exceed the data range of i64 here.
                    let col: isize = col_idx.try_into()?;
                    let lin: isize = line_idx.try_into()?;
                    data.insert(data::Point::new(col, lin), val);
                }
                Err(err) => {
                    errs.push(format!("{:?}", err));
                }
            }
        }
    }

    if errs.len() == 0 {
        Ok(data)
    } else {
        // Concatenate errors into one giant error message in case there were any in the file.
        Err(Error::msg(errs.join("\n------------------\n")))
    }
}
// end::io[]
