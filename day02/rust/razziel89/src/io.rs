use anyhow::{Context, Error, Result};
use std::str::FromStr;

fn read_lines_from_file(path: &str) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)
        .context("reading from disk")?
        .trim_end()
        .split('\n')
        .map(|el| String::from(el))
        .collect())
}

pub fn parse_lines_to_data<T>(file: &str, type_name: &str) -> Result<Vec<T>>
where
    T: FromStr<Err = Error>,
{
    let mut errs: Vec<String> = vec![];

    // Read file and convert into actions.
    let data = read_lines_from_file(file)
        .context("reading lines")?
        .into_iter()
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
