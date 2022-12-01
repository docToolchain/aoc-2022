use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");


    let elves_data = input.split("\n\n");
    let mut calories = Vec::<u64>::new();

    for elf in elves_data {
        let val: u64 = elf.split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| -> u64 {s.parse().unwrap()})
        .fold(0u64, |acc, b| acc + b);
        calories.push(val);
    }

    println!("{}", calories.iter().max().unwrap());


    // Top 3
    calories.sort();
    let sum: u64 = calories.iter().rev().take(3).sum();
    println!("{}", sum);
}
