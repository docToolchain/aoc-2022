use input::*;
use mr_kaffee_aoc::{letters::Letters, Puzzle, Star};

const LIT: char = '#';
const DARK: char = '.';

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, isize, isize, String, String> {
    Puzzle {
        year: 2022,
        day: 10,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(11_720),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some("ERCREPCJ".to_string()),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Instr {
        NoOp,
        AddX(isize),
    }

    #[derive(Debug)]
    pub struct PuzzleData {
        instructions: Vec<Instr>,
    }

    impl From<&'static str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &'static str) -> Self {
            Self {
                instructions: s
                    .lines()
                    .map(|l| match l {
                        "noop" => Instr::NoOp,
                        _ => Instr::AddX(l[5..].parse().unwrap()),
                    })
                    .collect(),
            }
        }
    }

    impl PuzzleData {
        pub fn instructions(&self) -> &[Instr] {
            &&self.instructions
        }
    }
}
// end::input[]

// tag::cpu[]
pub struct Cpu {
    instructions: Vec<Instr>,
    pointer: usize,
    cycle: usize,
    timer: usize,
    x: isize,
}

impl Cpu {
    pub fn init(instructions: &[Instr]) -> Self {
        let mut cpu = Self {
            instructions: Vec::from(instructions),
            pointer: 0,
            cycle: 0,
            timer: 0,
            x: 1,
        };

        // set initial timer if required
        if let Instr::AddX(_) = cpu.instructions[0] {
            cpu.timer = 1;
        }

        cpu
    }

    pub fn step(&mut self) {
        // should not be called if no more instructions
        assert!(self.pointer < self.instructions.len(), "Cpu halted");

        if self.timer > 0 {
            // only decrement timer
            self.timer -= 1;
        } else {
            // apply increment
            if let Instr::AddX(inc) = self.instructions[self.pointer] {
                self.x += inc;
            }

            // increment pointer
            self.pointer += 1;

            // set timer if required
            if self.pointer < self.instructions.len() {
                if let Instr::AddX(_) = self.instructions[self.pointer] {
                    self.timer = 1;
                }
            }
        }

        // increment cycle counter
        self.cycle += 1;
    }
}
// end::cpu[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> isize {
    let mut cpu = Cpu::init(data.instructions());

    // we first look during 20th cycle, i.e., after 19th cycle
    for _ in 0..19 {
        cpu.step();
    }
    let mut v = (cpu.cycle as isize + 1) * cpu.x;

    // 5 blocks of 40 cycles
    for _ in 0..5 {
        for _ in 0..40 {
            cpu.step();
        }
        v += (cpu.cycle as isize + 1) * cpu.x;
    }

    v
}
// end::star_1[]

// tag::star_2[]
pub fn solve_2(data: &PuzzleData) -> [u8; 240] {
    const W: usize = 40;
    const H: usize = 6;

    // initialize lcd with new lines, W + 1 to keep new lines at the and
    let mut lcd = [b'.'; W * H];

    let mut cpu = Cpu::init(data.instructions());
    for row in 0..H {
        for col in 0..W {
            lcd[col as usize + W * row] = if (cpu.x - 1..=cpu.x + 1).contains(&(col as _)) {
                LIT as u8
            } else {
                DARK as u8
            };
            cpu.step();
        }
    }

    lcd
}

pub fn star_2(data: &PuzzleData) -> String {
    solve_2(data).decode(0).unwrap()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_try_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(13_140, star_1(&data));
    }

    #[test]
    pub fn test_solve_2() {
        let data = PuzzleData::from(CONTENT);
        let b = solve_2(&data);
        let mut s = String::with_capacity(41 * 6);
        for row in 0..6 {
            for col in 0..40 {
                s.push(b[col + 40 * row] as _);
            }
            s.push('\n');
        }
        assert_eq!(EXP_2, s);
    }

    #[test]
    pub fn test_simple() {
        let data = PuzzleData::from(CONTENT_SIMPLE);
        let mut cpu = Cpu::init(data.instructions());
        let r = (0..5)
            .map(|_| {
                cpu.step();
                cpu.x
            })
            .collect::<Vec<_>>();
        let exp: &[isize] = &[1, 1, 4, 4, -1];
        assert_eq!(exp, r);
    }

    const CONTENT_SIMPLE: &str = r#"noop
addx 3
addx -5"#;

    const EXP_2: &str = r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    const CONTENT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;
}
// end::tests[]
