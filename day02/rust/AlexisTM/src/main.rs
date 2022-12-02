use std::{collections::HashMap, fs};

fn solve(input: &String, quick_conversion: &HashMap<&str, u64>) -> u64 {
    let result: u64 = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .fold(0u64, |acc, b| acc + quick_conversion[b]);
    return result;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let quick_conversion_p1: HashMap<&str, u64> = [
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]
    .iter()
    .cloned()
    .collect();

    /*
    A ROCK
    B PAPER
    C CISSOR
    */

    let quick_conversion_p2: HashMap<&str, u64> = [
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]
    .iter()
    .cloned()
    .collect();
    let p1 = solve(&input, &quick_conversion_p1);
    let p2 = solve(&input, &quick_conversion_p2);

    print!("{} {}", p1, p2);
}
