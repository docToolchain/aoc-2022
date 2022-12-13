use std::collections::VecDeque;

use std::{fs};


#[derive(Debug, Clone, PartialEq)]
struct Pose {
    pub pos_height: i16,
    pub pos_width: i16,
}

#[derive(Debug, Clone)]
struct Cell {
    pub altitude: u8,
    pub seen: bool,
}

type Map = Vec<Vec<Cell>>;

#[derive(Debug, Clone)]
struct Solution {
    pub pose: Pose,
    pub step: u16,
}

impl Solution {
    // Returns the 4 new solutions splitting from the current one, but filter them if:
    // - Out of boundsss
    // - Not reachable (too high/or low)
    // - Not seen than another concurrent solution
    fn split(&self, problem: &mut Problem) -> VecDeque<Self> {
        let mut vec = VecDeque::<Solution>::new();
        let mut other_step = self.clone();
        other_step.step += 1;
        let mut other = other_step.clone();
        other.pose.pos_height += 1;
        if other.in_bounds(problem) && other.reachable(problem, self) && other.seen(problem) {
            vec.push_back(other);
        }
        let mut other = other_step.clone();
        other.pose.pos_height -= 1;
        if other.in_bounds(problem) && other.reachable(problem, self) && other.seen(problem) {
            vec.push_back(other);
        }
        let mut other = other_step.clone();
        other.pose.pos_width += 1;
        if other.in_bounds(problem) && other.reachable(problem, self) && other.seen(problem) {
            vec.push_back(other);
        }
        let mut other = other_step;
        other.pose.pos_width -= 1;
        if other.in_bounds(problem) && other.reachable(problem, self) && other.seen(problem) {
            vec.push_back(other);
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
        true
    }

    // Returns the altitude of the cell
    pub fn altitude(&self, problem: &Problem) -> u8 {
        let cell = &problem.heightmap[usize::try_from(self.pose.pos_height).unwrap()]
            [usize::try_from(self.pose.pos_width).unwrap()];
        cell.altitude
    }

    // Returns true if we can reach it from our current altitude
    pub fn reachable(&self, problem: &Problem, from: &Solution) -> bool {
        if self.altitude(problem) > from.altitude(problem) + 1 {
            return false;
        }
        true
    }

    // Returns false if another solutions has already seen this one
    pub fn seen(&self, problem: &mut Problem) -> bool {
        let cell = &mut problem.heightmap[usize::try_from(self.pose.pos_height).unwrap()]
            [usize::try_from(self.pose.pos_width).unwrap()];
        if cell.seen {
            false
        } else {
            cell.seen = true;
            true
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    pub heightmap: Map,
    pub hiker: Pose,
    pub destination: Pose,
    pub start_part2: VecDeque<Solution>,
    pub width: i16,
    pub height: i16,
}

fn solve(problem: &mut Problem, solutions: &mut VecDeque<Solution>) -> Option<Solution> {
    while let Some(solution) = solutions.pop_front() {
        if solution.pose == problem.destination {
            return Some(solution.clone());
        }

        solutions.append(&mut solution.split(problem));
        if solutions.is_empty() {
            println!("FAILED: Last solution: {:?}", solution);
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let width = input.lines().next().unwrap().chars().count();
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
        start_part2: VecDeque::<Solution>::new(),
    };

    // map[height][width]
    problem.heightmap.resize(height, Vec::new());
    for el in problem.heightmap.iter_mut() {
        el.resize(
            width,
            Cell {
                altitude: 0,
                seen: false,
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
                                seen: false,
                            };
                            problem.start_part2.push_back(Solution {
                                pose: Pose {
                                    pos_height: pos_height.try_into().unwrap(),
                                    pos_width: pos_width.try_into().unwrap(),
                                },
                                step: 0,
                            });
                        }
                        'E' => {
                            problem.destination.pos_height = pos_height.try_into().unwrap();
                            problem.destination.pos_width = pos_width.try_into().unwrap();
                            problem.heightmap[pos_height][pos_width] = Cell {
                                altitude: 25,
                                seen: false,
                            };
                        }
                        'a' => {
                            problem.start_part2.push_back(Solution {
                                pose: Pose {
                                    pos_height: pos_height.try_into().unwrap(),
                                    pos_width: pos_width.try_into().unwrap(),
                                },
                                step: 0,
                            });
                            problem.heightmap[pos_height][pos_width] = Cell {
                                altitude: 0,
                                seen: false,
                            };
                        }
                        'b'..='z' => {
                            problem.heightmap[pos_height][pos_width] = Cell {
                                altitude: (altitude_char as u8) - b'a',
                                seen: false,
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
    let mut problem_p2 = problem.clone();

    let mut solutions_p1 = VecDeque::<Solution>::new();
    solutions_p1.push_back(Solution {
        pose: problem.hiker.clone(),
        step: 0,
    });
    let solution = solve(&mut problem, &mut solutions_p1);
    if let Some(solution) = solution {
        println!("The solution: {:?}", solution);
    }

    let mut solutions_p2 = VecDeque::<Solution>::new();
    solutions_p2.append(&mut problem.start_part2);
    let solution = solve(&mut problem_p2, &mut solutions_p2);
    if let Some(solution) = solution {
        println!("The solution: {:?}", solution);
    }
}
