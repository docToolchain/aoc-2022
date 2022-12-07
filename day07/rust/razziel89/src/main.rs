#![feature(let_chains)]
// Expected input file names.
const SAMPLE: &str = "sample.dat";
const REAL: &str = "stage_1.dat";

// Dependencies.
extern crate derive_more;
mod data;
mod io;

// tag::main[]
use anyhow::{Error, Result};
use std::collections::HashMap;
// Constants.
// Part 1.
const MAX_SIZE: usize = 100000;
// Part 2.
const TOTAL_SIZE: usize = 70000000;
const REQUIRED_SIZE: usize = 30000000;

// This could probably be improved but it works.
fn join(first: &String, second: String, trail_slash: bool) -> String {
    match (first.as_str(), trail_slash) {
        ("/", true) => format!("/{}/", second),
        ("/", false) => format!("/{}", second),
        (_, true) => format!("{}/{}/", first, second),
        (_, false) => format!("{}/{}", first, second),
    }
}

// This finds the parent directory. There is a lot of special handling here for the root directory.
fn parent(dir: String) -> String {
    if dir == "/" {
        "/".to_string()
    } else {
        let parent = dir
            .as_str()
            // This reverses the direction of iteration.
            .rsplit("/")
            .skip(1)
            // We need to collect here because we need to reverse the direction of iteration later
            // but the return type of `skip` doesn't allow that.
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("/");
        if parent.starts_with("/") {
            parent
        } else {
            format!("/{}", parent)
        }
    }
}

// We don'tuse a tree to represent the file system because trees are hard. Instead, we use a map
// mapping slash-separated strings to size values. An entry ending in a slash is a directory. That
// way, we can build everything up at once.
fn build_fs(entries: Vec<data::Entry>) -> Result<HashMap<String, usize>> {
    let mut cwd = "/".to_string();
    let mut fs = HashMap::<String, usize>::new();
    // This boolean serves as a way to check that we retrieve listed values only after the ls
    // command has been issued. It's just a sanity check for the input.
    let mut listing = false;

    fs.insert("/".to_string(), 0);

    // Build up the file system. We fill in directory sizes later down below, which simplifies that
    // but is less efficient.
    for entry in entries {
        match entry {
            data::Entry::CD(dir) => {
                listing = false;
                cwd = match dir.as_str() {
                    ".." => parent(cwd),
                    "/" => dir,
                    _ => join(&cwd, dir, false),
                };
                if cwd != "/" && !fs.contains_key(&cwd) {
                    // Entries with a trailing slash are directories.
                    fs.insert(format!("{}/", cwd), 0);
                }
            }
            data::Entry::LS => {
                listing = true;
            }
            data::Entry::DIR(dir) => {
                if !listing {
                    return Err(Error::msg("found dir entry but not not in list mode"));
                }
                fs.insert(join(&cwd, dir, true), 0);
            }
            data::Entry::FILE { name, size } => {
                if !listing {
                    return Err(Error::msg("found file entry but not not in list mode"));
                }
                if cwd != "/" && !fs.contains_key(format!("{}/", cwd).as_str()) {
                    return Err(Error::msg(format!("missing parent node {}", cwd)));
                }
                fs.insert(join(&cwd, name, false), size);
            }
        }
    }

    // I did not bother trying to find a solution tha works without copying.
    let dirs = fs
        .iter()
        .filter_map(|(name, _)| {
            if name.ends_with("/") {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Accumulate sizes for all dirs. We iterate over all dirs and then add up the sizes of all
    // files underneath them. That's inefficient but OK enough here.
    for dir in dirs {
        let dir_size = fs
            .iter()
            .filter_map(|(name, size)| {
                // Is a file underneath `dir`, emit its size. Otherwise, skip it.
                if !name.ends_with("/") && name.starts_with(&dir) {
                    Some(size)
                } else {
                    None
                }
            })
            .sum::<usize>();

        fs.insert(dir.to_string(), dir_size);
    }

    Ok(fs)
}

fn solve(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let entries = io::parse_lines_to_data::<data::Entry>(file, "entry", None, None)?;

    let filesystem = build_fs(entries)?;

    // Part 1.
    let result_part1 = filesystem
        .iter()
        .filter_map(|(name, size)| {
            if name.ends_with("/") && size <= &MAX_SIZE {
                Some(size)
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("requested size is {}", result_part1);

    // Part 2.
    // We already accumulated sizes, so getting this is easy. The amount of free space is the total
    // size minus what we currently occupy, which is the size of the root directory.
    // Note the use of checked_sub here because I wanted to try it out for subtracting from
    // unsigned values. Those checked_* methods allow graceful handling of overflows. Without them,
    // rust would panic if there was a violation of a type's value range.
    let used_space = *filesystem
        .get("/")
        .ok_or(Error::msg("cannot retrieve used space"))?;
    let free_space = TOTAL_SIZE
        .checked_sub(used_space)
        .ok_or(Error::msg("cannot compute free space"))?;
    let required_space = REQUIRED_SIZE
        .checked_sub(free_space)
        .ok_or(Error::msg("cannot compute required space"))?;

    eprintln!("need to free up at least {}", required_space);

    // Find the smallest directory that fulfils that condition.
    let min_free_size = filesystem
        .iter()
        .filter_map(|(name, size)| {
            if name.ends_with("/") && size >= &required_space {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .ok_or(Error::msg("cannot find any directory for part 2"))?;

    println!("freeing {} is enough", min_free_size);

    Ok(())
}

fn main() -> Result<()> {
    solve(SAMPLE)?;
    solve(REAL)?;

    Ok(())
}
// end::main[]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_parent() {
        assert_eq!("/".to_string(), parent("/".to_string()));
        assert_eq!("/asdf".to_string(), parent("/asdf/blub".to_string()));
        assert_eq!("/".to_string(), parent("/asdf".to_string()));
        assert_eq!(
            "/a/s/d/f".to_string(),
            parent(parent("/a/s/d/f/g/h".to_string()))
        );
    }

    #[test]
    fn test_join() {
        assert_eq!(
            "/a/s/d/f".to_string(),
            join(&"/a/s/d".to_string(), "f".to_string(), false)
        );
        assert_eq!(
            "/a".to_string(),
            join(&"/".to_string(), "a".to_string(), false)
        );
    }
}
