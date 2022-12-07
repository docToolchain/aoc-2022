// tag::io[]
use anyhow::{Context, Error, Result};
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

pub type Predicate = fn(&String) -> bool;
pub type Transform = fn(String) -> String;

pub fn parse_lines_to_data<T>(
    file: &str,
    type_name: &str,
    filter: Option<Predicate>,
    transform: Option<Transform>,
) -> Result<Vec<T>>
where
    T: FromStr<Err = Error>,
{
    let filter_fn = filter.unwrap_or(|_| true);
    let transformer = transform.unwrap_or(|el| el);

    let mut errs: Vec<String> = vec![];

    // Read file and convert into actions.
    let data = read_lines_from_file(file)
        .context("reading lines")?
        .into_iter()
        .filter(filter_fn)
        .map(transformer)
        .enumerate()
        .filter_map(|(idx, el)| {
            match el
                .parse::<T>()
                .with_context(|| format!("cannot parse line {} as {}: {}", idx, type_name, el))
            {
                Ok(val) => Some(val),
                Err(err) => {
                    errs.push(format!("{:?}", err));
                    None
                }
            }
        })
        .collect();

    if errs.len() == 0 {
        Ok(data)
    } else {
        // Concatenate errors into one giant error message in case there were any in the file.
        Err(Error::msg(errs.join("\n------------------\n")))
    }
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
// end::io[]
