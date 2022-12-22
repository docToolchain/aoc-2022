use std::collections::{HashMap, VecDeque};

use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 22,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(58_248),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &(|data| star_2(data, 50)),
            exp: Some(179_091),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug, PartialEq, Eq)]
    pub enum Step {
        Fwd(usize),
        Left,
        Right,
    }

    #[derive(Debug)]
    pub struct PuzzleData {
        pub grid: Vec<u8>,
        pub width: usize,
        pub steps: Vec<Step>,
    }

    impl From<&str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &str) -> Self {
            let (grid_part, steps_part) = s.split_once("\n\n").unwrap();

            // store the grid with rows of equal width
            let lines = grid_part.lines().collect::<Vec<_>>();
            let width = lines.iter().map(|line| line.len()).max().unwrap();
            let mut grid = vec![b' '; width * lines.len()];
            for row in 0..lines.len() {
                for (col, b) in lines[row].as_bytes().iter().enumerate() {
                    grid[col + row * width] = *b;
                }
            }

            let mut steps = Vec::new();
            let mut fwd = 0;
            for b in steps_part.trim().as_bytes() {
                if (b'0'..=b'9').contains(b) {
                    fwd = 10 * fwd + (b - b'0') as usize;
                } else {
                    if fwd != 0 {
                        steps.push(Step::Fwd(fwd));
                        fwd = 0;
                    }
                    if *b == b'L' {
                        steps.push(Step::Left);
                    } else if *b == b'R' {
                        steps.push(Step::Right);
                    } else {
                        panic!("Unexpected {} in {steps_part}", *b as char);
                    }
                }
            }
            if fwd != 0 {
                steps.push(Step::Fwd(fwd));
            }

            Self { grid, width, steps }
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    let mut col = data.grid.iter().position(|&b| b == b'.').unwrap();
    let mut row = 0;
    let mut d: usize = 0;
    let height = data.grid.len() / data.width;

    for step in &data.steps {
        match step {
            Step::Fwd(fwd) => {
                let (d_col, d_row, n) = match d {
                    0 => (1, 0, data.width),
                    1 => (0, 1, height),
                    2 => (data.width - 1, 0, data.width),
                    3 => (0, height - 1, height),
                    _ => unreachable!(),
                };
                (col, row) = (0..n)
                    .cycle()
                    .map(|s| ((col + s * d_col) % data.width, (row + s * d_row) % height))
                    .filter(|(col, row)| data.grid[col + data.width * row] != b' ')
                    .take(fwd + 1)
                    .take_while(|(col, row)| data.grid[col + row * data.width] != b'#')
                    .last()
                    .unwrap();
            }
            Step::Left => d = (d + 3) % 4,
            Step::Right => d = (d + 1) % 4,
        }
    }

    (row + 1) * 1000 + (col + 1) * 4 + d
}
// end::star_1[]

// tag::star_2[]
pub const EAST: usize = 0; // [1 0 0]
pub const NORTH: usize = 1; // [0 1 0]
pub const UP: usize = 2; // [0 0 1]
pub const WEST: usize = 3; // [-1 0 0]
pub const SOUTH: usize = 4; // [0 -1 0]
pub const DOWN: usize = 5; // [0 0 -1]

pub const HEAD_RIGHT: usize = 0;
pub const HEAD_DOWN: usize = 1;
pub const HEAD_LEFT: usize = 2;
pub const HEAD_UP: usize = 3;

pub fn star_2(data: &PuzzleData, face_width: usize) -> usize {
    let layout_w = data.width / face_width;
    let layout_h = (data.grid.len() / data.width) / face_width;
    assert_eq!(
        layout_w * face_width * layout_h * face_width,
        data.grid.len(),
        "Grid len does not match layout"
    );

    // get the cube layout (each face is represented by a '#')
    let cube_layout = (0..data.grid.len() / (face_width * face_width))
        .map(|p| (p % layout_w, p / layout_w))
        .map(|(c, r)| {
            if data.grid[c * face_width + r * face_width * data.width] == b' ' {
                ' '
            } else {
                '#'
            }
        })
        .collect::<Vec<_>>();

    // compile a map of cube faces with the normal direction as key and
    // ((col, row), (n_dir, x_dir, y_dir)) as value where (col, row) is the
    // position in the grid and (n_dir, x_dir, y_dir) is normal direction
    // and direction of x- and y- respectively
    let pos: usize = cube_layout.iter().position(|&face| face == '#').unwrap();
    let start = ((pos % layout_w, pos / layout_w), (UP, EAST, SOUTH));
    let mut faces = HashMap::from([(UP, start)]);
    let mut queue = VecDeque::from([start]);
    while let Some(((col, row), (n_dir, x_dir, y_dir))) = queue.pop_front() {
        if col > 0 && cube_layout[col - 1 + row * layout_w] == '#' {
            let next = ((col - 1, row), ((x_dir + 3) % 6, n_dir, y_dir));
            if !faces.contains_key(&next.1 .0) {
                faces.insert(next.1 .0, next);
                queue.push_back(next);
            }
        }
        if col < layout_w - 1 && cube_layout[col + 1 + row * layout_w] == '#' {
            let next = ((col + 1, row), (x_dir, (n_dir + 3) % 6, y_dir));
            if !faces.contains_key(&next.1 .0) {
                faces.insert(next.1 .0, next);
                queue.push_back(next);
            }
        }
        if row > 0 && cube_layout[col + (row - 1) * layout_w] == '#' {
            let next = ((col, row - 1), ((y_dir + 3) % 6, x_dir, n_dir));
            if !faces.contains_key(&next.1 .0) {
                faces.insert(next.1 .0, next);
                queue.push_back(next);
            }
        }
        if row < layout_h - 1 && cube_layout[col + (row + 1) * layout_w] == '#' {
            let next = ((col, row + 1), (y_dir, x_dir, (n_dir + 3) % 6));
            if !faces.contains_key(&next.1 .0) {
                faces.insert(next.1 .0, next);
                queue.push_back(next);
            }
        }
    }

    // current position is given by (x, y) within face and (col, row) of face
    // in addition normal direction and x-/y-direction of current face and
    // the current orientation `d`
    let pos = data.grid.iter().position(|&b| b == b'.').unwrap();
    let mut x = pos % face_width;
    let mut y = 0;
    let mut col = pos / face_width;
    let mut row = 0;
    let (_, (mut n_dir, mut x_dir, mut y_dir)) = faces.get(&UP).unwrap();
    let mut d: usize = 0;

    #[cfg(feature = "print-path")]
    let mut path: HashMap<usize, usize> = HashMap::from([(pos, d)]);

    for step in &data.steps {
        match step {
            Step::Fwd(fwd) => {
                for _ in 0..*fwd {
                    // determine x and y updates (wrapping)
                    // and normal direction of next face (if wrapping)
                    let (x_upd, y_upd, n_dir_upd) = match d {
                        HEAD_RIGHT => (
                            (x + 1) % face_width,
                            y,
                            if x < face_width - 1 { n_dir } else { x_dir },
                        ),
                        HEAD_DOWN => (
                            x,
                            (y + 1) % face_width,
                            if y < face_width - 1 { n_dir } else { y_dir },
                        ),
                        HEAD_LEFT => (
                            (x + face_width - 1) % face_width,
                            y,
                            if x > 0 { n_dir } else { (x_dir + 3) % 6 },
                        ),
                        HEAD_UP => (
                            x,
                            (y + face_width - 1) % face_width,
                            if y > 0 { n_dir } else { (y_dir + 3) % 6 },
                        ),
                        _ => unreachable!(),
                    };

                    let (x_upd, y_upd, d_upd, col_upd, row_upd, x_dir_upd, y_dir_upd) = if n_dir_upd
                        != n_dir
                    {
                        // calculate update on a new face
                        // get face
                        let ((col_upd, row_upd), (n_dir_upd, x_dir_upd, y_dir_upd)) =
                            *faces.get(&n_dir_upd).unwrap();

                        // do the transformation
                        let (x_upd, y_upd, d_upd) = if n_dir_upd == x_dir {
                            // move right
                            if y_dir_upd == y_dir && x_dir_upd == (n_dir + 3) % 6 {
                                (0, y, HEAD_RIGHT)
                            } else if y_dir_upd == (y_dir + 3) % 6 && x_dir_upd == n_dir {
                                (face_width - 1, face_width - 1 - y, HEAD_LEFT)
                            } else if x_dir_upd == y_dir && y_dir_upd == n_dir {
                                (y, face_width - 1, HEAD_UP)
                            } else if x_dir_upd == (y_dir + 3) % 6 && y_dir_upd == (n_dir + 3) % 6 {
                                (face_width - 1 - y, 0, HEAD_DOWN)
                            } else {
                                unreachable!(
                                    "right: {}, _{}_, {} --> _{}_, {}, {}",
                                    n_dir, x_dir, y_dir, n_dir_upd, x_dir_upd, y_dir_upd
                                )
                            }
                        } else if n_dir_upd == (x_dir + 3) % 6 {
                            // move left
                            if y_dir_upd == y_dir && x_dir_upd == n_dir {
                                (face_width - 1, y, HEAD_LEFT)
                            } else if y_dir_upd == (y_dir + 3) % 6 && x_dir_upd == (n_dir + 3) % 6 {
                                (0, face_width - 1 - y, HEAD_RIGHT)
                            } else if x_dir_upd == y_dir && y_dir_upd == (n_dir + 3) % 6 {
                                (y, 0, HEAD_DOWN)
                            } else if x_dir_upd == (y_dir + 3) % 6 && y_dir_upd == n_dir {
                                (face_width - 1 - y, face_width - 1, HEAD_UP)
                            } else {
                                unreachable!(
                                    "left: {}, _{}_, {} --> _{}_, {}, {}",
                                    n_dir, x_dir, y_dir, n_dir_upd, x_dir_upd, y_dir_upd
                                )
                            }
                        } else if n_dir_upd == y_dir {
                            // move down
                            if x_dir_upd == x_dir && y_dir_upd == (n_dir + 3) % 6 {
                                (x, 0, HEAD_DOWN)
                            } else if x_dir_upd == (x_dir + 3) % 6 && y_dir_upd == n_dir {
                                (face_width - 1 - x, face_width - 1, HEAD_UP)
                            } else if y_dir_upd == x_dir && x_dir_upd == n_dir {
                                (face_width - 1, x, HEAD_LEFT)
                            } else if y_dir_upd == (x_dir + 3) % 6 && x_dir_upd == (n_dir + 3) % 6 {
                                (0, face_width - 1 - x, HEAD_RIGHT)
                            } else {
                                unreachable!(
                                    "down: {}, {}, _{}_ --> _{}_, {}, {}",
                                    n_dir, x_dir, y_dir, n_dir_upd, x_dir_upd, y_dir_upd
                                )
                            }
                        } else if n_dir_upd == (y_dir + 3) % 6 {
                            // move up
                            if x_dir_upd == x_dir && y_dir_upd == n_dir {
                                (x, face_width - 1, HEAD_UP)
                            } else if x_dir_upd == (x_dir + 3) % 6 && y_dir_upd == (n_dir + 3) % 6 {
                                (face_width - 1 - x, 0, HEAD_DOWN)
                            } else if y_dir_upd == x_dir && x_dir_upd == (n_dir + 3) % 6 {
                                (0, x, HEAD_RIGHT)
                            } else if y_dir_upd == (x_dir + 3) % 6 && x_dir_upd == n_dir {
                                (face_width - 1, face_width - 1 - x, HEAD_LEFT)
                            } else {
                                unreachable!(
                                    "up: {}, {}, _{}_ --> _{}_, {}, {}",
                                    n_dir, x_dir, y_dir, n_dir_upd, x_dir_upd, y_dir_upd
                                )
                            }
                        } else {
                            unreachable!()
                        };

                        (x_upd, y_upd, d_upd, col_upd, row_upd, x_dir_upd, y_dir_upd)
                    } else {
                        // update within a face
                        (x_upd, y_upd, d, col, row, x_dir, y_dir)
                    };

                    if data.grid
                        [x_upd + col_upd * face_width + (y_upd + row_upd * face_width) * data.width]
                        == b'#'
                    {
                        // wall, can't move any further
                        break;
                    }

                    // update
                    x = x_upd;
                    y = y_upd;
                    d = d_upd;
                    col = col_upd;
                    row = row_upd;
                    n_dir = n_dir_upd;
                    x_dir = x_dir_upd;
                    y_dir = y_dir_upd;

                    #[cfg(feature = "print-path")]
                    path.insert(
                        x + col * face_width + (y + row * face_width) * data.width,
                        d,
                    );
                }
            }
            Step::Left => d = (d + 3) % 4,
            Step::Right => d = (d + 1) % 4,
        }

        #[cfg(feature = "print-path")]
        path.insert(
            x + col * face_width + (y + row * face_width) * data.width,
            d,
        );
    }

    #[cfg(feature = "print-path")]
    for y in 0..data.grid.len() / data.width {
        for x in 0..data.width {
            let p = x + y * data.width;
            match (path.get(&p), data.grid[p]) {
                (Some(0), b'.') => {
                    print!(">");
                }
                (Some(1), b'.') => {
                    print!("v");
                }
                (Some(2), b'.') => {
                    print!("<");
                }
                (Some(3), b'.') => {
                    print!("^");
                }
                (None, b'#') => {
                    print!("#");
                }
                (None, b'.') => {
                    print!(".");
                }
                (None, b' ') => {
                    print!(" ");
                }
                (a, b) => panic!("Unexpected {a:?}, {}", b as char),
            }
        }
        println!();
    }

    (y + row * face_width + 1) * 1000 + (x + col * face_width + 1) * 4 + d
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(6_032, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(5_031, star_2(&data, 4));

        let data = PuzzleData::from(include_str!("../tests_1.txt"));
        assert_eq!(1_033, star_2(&data, 5));

        let data = PuzzleData::from(include_str!("../tests_2.txt"));
        assert_eq!(1_033, star_2(&data, 5));

        let data = PuzzleData::from(include_str!("../tests_3.txt"));
        assert_eq!(12_013, star_2(&data, 5));

        let data = PuzzleData::from(include_str!("../tests_4.txt"));
        assert_eq!(3_028, star_2(&data, 5));
    }
}
// end::tests[]
