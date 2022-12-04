use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // 556
    // tag::solution_part1[]
    let part1: u32 = input
        .lines()
        .map(|f| f.split(|f| f == ',' || f == '-'))
        .map(|mut f| {
            (
                (
                    f.next().unwrap().parse::<u32>().unwrap(),
                    f.next().unwrap().parse::<u32>().unwrap(),
                ),
                (
                    f.next().unwrap().parse::<u32>().unwrap(),
                    f.next().unwrap().parse::<u32>().unwrap(),
                ),
            )
        })
        .map(|(elf1, elf2)| {
            if (elf1.0 <= elf2.0 && elf2.1 <= elf1.1) || (elf2.0 <= elf1.0 && elf1.1 <= elf2.1) {
                return 1;
            }
            return 0;
        })
        .sum();
    // end::solution_part1[]

    // 876
    let part2: u32 = input
        .lines()
        .map(|f| f.split(|f| f == ',' || f == '-'))
        .map(|mut f| {
            (
                (
                    f.next().unwrap().parse::<u32>().unwrap(),
                    f.next().unwrap().parse::<u32>().unwrap(),
                ),
                (
                    f.next().unwrap().parse::<u32>().unwrap(),
                    f.next().unwrap().parse::<u32>().unwrap(),
                ),
            )
        })
        // tag::solution_part2[]
        .map(|(elf1, elf2)| {
            if (elf1.0 <= elf2.0 && elf2.0 <= elf1.1) || (elf2.0 <= elf1.0 && elf1.0 <= elf2.1) {
                return 1;
            }
            return 0;
        })
        // end::solution_part2[]
        .sum();

    print!("{} {}", part1, part2);
}
