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

// We don'tuse a tree to represent the file system because trees are hard. Instead, we use a map
// mapping slash-separated strings to size values. An entry ending in a slash is a directory. That
// way, we can build everything up at once.
fn build_fs(entries: Vec<data::Entry>) -> Result<HashMap<String, usize>> {
    let mut cwd = data::Stack::new();
    let mut fs = HashMap::<String, usize>::new();
    // This boolean serves as a way to check that we retrieve listed values only after the ls
    // command has been issued. It's just a sanity check for the input.
    let mut listing = false;

    fs.insert("/".to_string(), 0);

    // Build up the file system.
    for entry in entries {
        match entry {
            data::Entry::CD(dir) => {
                listing = false;
                match dir.as_str() {
                    ".." => cwd.popd(),
                    "/" => cwd.clear(),
                    _ => cwd.pushd(dir),
                }
                // Entries with a trailing slash are directories.
                let dir = cwd.pwd();
                if !fs.contains_key(&dir) {
                    fs.insert(dir, 0);
                }
            }
            data::Entry::LS => {
                listing = true;
            }
            data::Entry::DIR(dir) => {
                if !listing {
                    return Err(Error::msg("found dir entry but not not in list mode"));
                }
                // Entries with a trailing slash are directories.
                fs.insert(format!("{}{}/", cwd.pwd(), dir), 0);
            }
            data::Entry::FILE { name, size } => {
                if !listing {
                    return Err(Error::msg("found file entry but not not in list mode"));
                }
                let dir = cwd.pwd();
                if !fs.contains_key(&dir) {
                    return Err(Error::msg(format!("missing parent node {}", dir)));
                }
                // Entries without a trailing slash are files.
                fs.insert(format!("{}{}", dir, name), size);
                // Add directory sizes.
                for dir in &mut cwd {
                    *fs.get_mut(&dir)
                        .ok_or(Error::msg(format!("cannot read directory {}", dir)))? += size;
                }
            }
        }
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
