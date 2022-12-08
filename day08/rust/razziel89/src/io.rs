// tag::io[]
use anyhow::{Context, Error, Result};
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

fn read_lines_from_file(path: &str) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)
        .context("reading from disk")?
        .trim_end()
        .split('\n')
        .map(|el| String::from(el))
        .collect())
}

// Convert Result to Option but make sure to add all errors messages to a vector of strings. Use
// "process_errs" to check whethere there are any errors in the vector.
pub fn filter_and_remember_errs<I, E>(item: Result<I, E>, errs: &mut Vec<String>) -> Option<I>
where
    E: Debug,
{
    match item {
        Ok(val) => Some(val),
        Err(err) => {
            errs.push(format!("{:?}", err));
            None
        }
    }
}

// If there is any element in the string vector, concatenate all ements into an error. Do not
// return an error otherwise.
pub fn process_remembered_errs(errs: Vec<String>) -> Result<()> {
    if errs.len() == 0 {
        Ok(())
    } else {
        // Concatenate errors into one giant error message in case there were any in the file.
        Err(Error::msg(errs.join("\n------------------\n")))
    }
}

pub fn parse_chars_to_data<T>(file: &str, type_name: &str) -> Result<HashMap<(i64, i64), T>>
where
    T: FromStr<Err = Error>,
{
    let mut errs: Vec<String> = vec![];
    let mut data = HashMap::<(i64, i64), T>::new();

    for (line_idx, line) in read_lines_from_file(file)?.into_iter().enumerate() {
        for (col_idx, character) in line.chars().enumerate() {
            match character.to_string().parse::<T>().with_context(|| {
                format!(
                    "cannot parse char in line {} and row {} as {}: {}",
                    line_idx, col_idx, type_name, character
                )
            }) {
                Ok(val) => {
                    // We should never exceed the data range of i64 here.
                    let col: i64 = col_idx.try_into()?;
                    let lin: i64 = line_idx.try_into()?;
                    data.insert((col, lin), val);
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

pub fn hashmap_to_string<T>(map: HashMap<(i64, i64), T>) -> String
where
    T: Debug,
{
    let mut vals = vec![];
    for ((x_idx, y_idx), val) in map {
        vals.push(format!("({},{}): {:?}", x_idx, y_idx, val));
    }
    vals.sort();
    vals.join("\n")
}
// end::io[]
