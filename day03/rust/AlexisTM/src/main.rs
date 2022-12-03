use std::{fs};
use itertools::Itertools;

// tag::common[]
fn common_char(start: &str, end: &str) -> Option<char> {
    start.chars().filter(|c| end.contains(*c)).nth(0)
}


fn common_chars(b1: &str, b2: &str, b3: &str) -> Option<char> {
    b1.chars().filter(|c| b2.contains(*c)).filter(|c| b3.contains(*c)).nth(0)
}
// end::common[]

// tag::letter_to_number[]
fn accumulate_letters(acc: u64, val: Option<char>) -> u64 {
    if let Some(val) = val {
        if val <= 'Z' {
            return acc + 27 + (val as u8 - 'A' as u8) as u64;
        } else {
            return acc + 1 + (val as u8 - 'a' as u8) as u64;
        }
    } else {
        println!("Empty result, can't be used!");
        return acc;
    }
}
// end::letter_to_number[]

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // tag::solution_part1[]
    let part1: u64 = input
        .lines()
        .map(|f| common_char(&f[0..f.len() / 2], &f[f.len() / 2..]))
        .fold(0, accumulate_letters);
    // end::solution_part1[]

    // tag::solution_part2[]
    let part2 = input.lines().chunks(3).into_iter().map(|chunk| {
        let values: Vec<&str> = chunk.collect();
        common_chars(values[0], values[1], values[2])
     }).fold(0u64, accumulate_letters);
     // end::solution_part2[]

    print!("{} {}", part1, part2);
}
