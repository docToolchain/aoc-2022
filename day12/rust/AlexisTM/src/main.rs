use std::fmt::Write;
use std::fs;
use std::{thread, time};

#[derive(Debug, Clone, PartialEq)]
struct Pose {
    pub pos_height: i16,
    pub pos_width: i16,
}

#[derive(Debug, Clone)]
struct Cell {
    pub altitude: u8,
    pub steps_to_reach: u16,
}

type Map = Vec<Vec<Cell>>;

#[derive(Debug)]
struct Problem {
    pub heightmap: Map,
    pub hiker: Pose,
    pub destination: Pose,
    pub width: i16,
    pub height: i16,
}

#[derive(Debug, Clone)]
struct Solution {
    pub pose: Pose,
    pub step: u16,
}

impl Solution {
    // Returns the 4 new solutions splitting from the current one, but filter them if:
    // - Out of boundsss
    // - Not reachable (too high/or low)
    // - Not better than another concurrent solution
    fn split(&self, problem: &mut Problem) -> Vec<Self> {
        let mut vec = Vec::<Solution>::new();
        let mut other_step = self.clone();
        other_step.step += 1;
        let mut other = other_step.clone();
        other.pose.pos_height += 1;
        if other.in_bounds(problem) && other.reachable(problem, self) && other.better(problem) {
            vec.push(other);
        }
        let mut other = other_step.clone();
        other.pose.pos_height -= 1;
        if other.in_bounds(problem) && other.reachable(problem, self) && other.better(problem) {
            vec.push(other);
        }
        let mut other = other_step.clone();
        other.pose.pos_width += 1;
        if other.in_bounds(problem) && other.reachable(problem, self) && other.better(problem) {
            vec.push(other);
        }
        let mut other = other_step.clone();
        other.pose.pos_width -= 1;
        if other.in_bounds(problem) && other.reachable(problem, self) && other.better(problem) {
            vec.push(other);
        }
        vec
    }

    pub fn in_bounds(&self, problem: &Problem) -> bool {
        if self.pose.pos_height >= problem.height
            || self.pose.pos_width >= problem.width
            || self.pose.pos_height < 0
            || self.pose.pos_width < 0
        {
            return false;
        }
        return true;
    }

    // Returns the altitude of the cell
    pub fn altitude(&self, problem: &Problem) -> u8 {
        let cell = &problem.heightmap[usize::try_from(self.pose.pos_height).unwrap()]
            [usize::try_from(self.pose.pos_width).unwrap()];
        cell.altitude
    }

    // Returns true if we can reach it from our current altitude
    pub fn reachable(&self, problem: &Problem, from: &Solution) -> bool {
        if self.altitude(problem).abs_diff(from.altitude(problem)) > 1 {
            return false;
        }
        return true;
    }

    // Returns flase if another solutions already reached this position with an equal or lower number of steps
    pub fn better(&self, problem: &mut Problem) -> bool {
        let cell = &mut problem.heightmap[usize::try_from(self.pose.pos_height).unwrap()]
            [usize::try_from(self.pose.pos_width).unwrap()];
        if self.step >= cell.steps_to_reach {
            return false;
        }
        cell.steps_to_reach = self.step;
        return true;
    }
}

fn solve(problem: &mut Problem) -> Option<Solution> {
    let mut solutions = Vec::<Solution>::new();
    solutions.push(Solution {
        pose: problem.hiker.clone(),
        step: 0,
    });

    let mut steps = 0;
    loop {
        let mut next_solutions = Vec::<Solution>::new();

        for solution in solutions.iter_mut() {
            if solution.pose == problem.destination {
                return Some(solution.clone());
            }

            next_solutions.append(&mut solution.split(problem));
        }

        // println!("Next solutions: {:?}", next_solutions);
        if next_solutions.len() == 0 {
            println!(
                "No solution found :'( Last solutions ongoing were: {:?}",
                solutions
            );
            return None;
        }

        fancy_print_problem_and_solutions(&problem, &next_solutions);
        thread::sleep(time::Duration::from_millis(100));

        drop(solutions);
        solutions = next_solutions;
        steps += 1;
        // println!("Step: {}", steps);
    }
}

fn fancy_print_problem_and_solutions(problem: &Problem, solutions: &Vec<Solution>) {
    let mut target_string = String::new();
    for line_i in 0..problem.heightmap.len() {
        for col_i in 0..problem.heightmap[line_i].len() {
            let mut printed = false;
            for solution in solutions {
                if solution.pose.pos_height == line_i.try_into().unwrap()
                    && solution.pose.pos_width == col_i.try_into().unwrap()
                {
                    write!(target_string, "o").unwrap();
                    printed = true;
                }
            }
            if !printed {
                write!(target_string, "-").unwrap();
            }
        }
        write!(target_string, "\n").unwrap();
    }
    write!(target_string, "\n").unwrap();
    print!("{}", target_string);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let width = input.lines().nth(0).unwrap().chars().count();
    let height = input.lines().count();

    let mut problem = Problem {
        heightmap: Map::new(),
        hiker: Pose {
            pos_height: 0,
            pos_width: 0,
        },
        destination: Pose {
            pos_height: 0,
            pos_width: 0,
        },
        width: width.try_into().unwrap(),
        height: height.try_into().unwrap(),
    };

    // map[height][width]
    problem.heightmap.resize(height, Vec::new());
    for el in problem.heightmap.iter_mut() {
        el.resize(
            width,
            Cell {
                altitude: 0,
                steps_to_reach: u16::MAX,
            },
        );
    }

    // Convert the input to problem data
    let mut problem = input
        .lines()
        .enumerate()
        .fold(problem, |problem, (pos_height, line)| {
            line.chars()
                .enumerate()
                .fold(problem, |mut problem, (pos_width, altitude_char)| {
                    match altitude_char {
                        'S' => {
                            problem.hiker.pos_height = pos_height.try_into().unwrap();
                            problem.hiker.pos_width = pos_width.try_into().unwrap();
                            problem.heightmap[pos_height][pos_width] = Cell {
                                altitude: 0,
                                steps_to_reach: 0,
                            };
                        }
                        'E' => {
                            problem.destination.pos_height = pos_height.try_into().unwrap();
                            problem.destination.pos_width = pos_width.try_into().unwrap();
                            problem.heightmap[pos_height][pos_width] = Cell {
                                altitude: 25,
                                steps_to_reach: u16::MAX,
                            };
                        }
                        'a'..='z' => {
                            problem.heightmap[pos_height][pos_width] = Cell {
                                altitude: (altitude_char as u8) - ('a' as u8),
                                steps_to_reach: u16::MAX,
                            };
                        }
                        _ => {}
                    }
                    problem
                })
        });

    println!("Problem size: {}x{}", width, height);
    println!("Hiker: {:?}", &problem.hiker);
    println!("Best signal: {:?}", &problem.destination);

    let solution = solve(&mut problem);
    if let Some(solution) = solution {
        println!("The solution: {:?}", solution);
    }
}
