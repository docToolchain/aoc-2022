use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 14,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(614),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(26_170),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        paths: Vec<Vec<(isize, isize)>>,
    }

    impl<'a> From<&'a str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &'a str) -> Self {
            Self {
                paths: s
                    .lines()
                    .map(|l| {
                        l.split(" -> ")
                            .map(|c| c.split_once(',').unwrap())
                            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                            .collect()
                    })
                    .collect(),
            }
        }
    }

    impl<'a> PuzzleData {
        pub fn bbox(&self) -> (isize, isize, isize, isize) {
            self.paths.iter().fold((500, 0, 501, 1), |bbox, v| {
                v.iter().fold(bbox, |(x_mn, y_mn, x_mx, y_mx), (x, y)| {
                    (x_mn.min(*x), y_mn.min(*y), x_mx.max(x + 1), y_mx.max(y + 1))
                })
            })
        }

        /// get a grid as flat list of chars, the width of the grid, and the point of sand inflow
        pub fn grid(&self) -> (Vec<char>, usize, (usize, usize)) {
            let (x_mn, y_mn, x_mx, y_mx) = self.bbox();
            let width = (x_mx - x_mn) as usize;
            let height = (y_mx - y_mn) as usize;
            let mut grid = vec!['.'; width * height];

            for path in &self.paths {
                for k in 1..path.len() {
                    let (dx, dy, len) = if path[k].0 > path[k - 1].0 {
                        assert!(path[k].1 == path[k - 1].1);
                        (1, 0, path[k].0 - path[k - 1].0)
                    } else if path[k].0 < path[k - 1].0 {
                        assert!(path[k].1 == path[k - 1].1);
                        (-1, 0, path[k - 1].0 - path[k].0)
                    } else if path[k].1 > path[k - 1].1 {
                        assert!(path[k].0 == path[k - 1].0);
                        (0, 1, path[k].1 - path[k - 1].1)
                    } else if path[k].1 < path[k - 1].1 {
                        assert!(path[k].0 == path[k - 1].0);
                        (0, -1, path[k - 1].1 - path[k].1)
                    } else {
                        unreachable!()
                    };

                    let x0 = path[k - 1].0 - x_mn;
                    let y0 = path[k - 1].1 - y_mn;
                    for k in 0..len + 1 {
                        grid[(x0 + dx * k) as usize + (y0 + dy * k) as usize * width] = '#';
                    }
                }
            }

            (grid, width, ((500 - x_mn) as _, (0 - y_mn) as _))
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    let (mut grid, width, (x_0, y_0)) = data.grid();
    let height = grid.len() / width;

    let mut cnt = 0;
    'inflow: loop {
        // first candidate is spot directly on top of something solid
        let mut x = x_0;
        let mut y = match (y_0..height).find(|y| grid[x + y * width] != '.') {
            Some(y) => y - 1,
            None => unreachable!("Nothing solid found to start with"),
        };

        loop {
            if y == height - 1 {
                // go to void below
                break 'inflow;
            }

            if grid[x + (y + 1) * width] == '.' {
                // bubble down
                y += 1;
            } else if x > 0 && grid[x - 1 + (y + 1) * width] == '.' {
                // bubble down-left
                y += 1;
                x -= 1;
            } else if x < width - 1 && grid[x + 1 + (y + 1) * width] == '.' {
                // bubble down-right
                y += 1;
                x += 1;
            } else if x == 0 || x == width - 1 {
                // go to void left/right
                break 'inflow;
            } else {
                grid[x + y * width] = 'o';
                cnt += 1;
                break;
            }
        }
    }

    cnt
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    let (grid_0, width_0, (x_0_0, y_0)) = data.grid();
    let height_0 = grid_0.len() / width_0;

    // wrap the grid in a bigger grid (append height columns to the left and right and an additional row below)
    let width = width_0 + 2 * height_0;
    let height = height_0 + 1;
    let mut grid = vec!['.'; width * height];
    for y in 0..height_0 {
        for x in 0..width_0 {
            grid[x + height_0 + y * width] = grid_0[x + y * width_0];
        }
    }
    let x_0 = x_0_0 + height_0;

    let mut cnt = 0;
    loop {
        // first candidate is spot directly on top of something solid
        let mut x = x_0;
        let mut y = match (y_0..height).find(|y| grid[x + y * width] != '.') {
            Some(y) => {
                if y == y_0 {
                    // No more sand can enter
                    break;
                }
                y - 1
            }
            None => unreachable!("Nothing solid found to start with"),
        };

        loop {
            if y == height - 1 {
                // floor reached
                grid[x + y * width] = 'o';
                cnt += 1;
                break;
            }

            if grid[x + (y + 1) * width] == '.' {
                // bubble down
                y += 1;
            } else if x > 0 && grid[x - 1 + (y + 1) * width] == '.' {
                // bubble down-left
                y += 1;
                x -= 1;
            } else if x < width - 1 && grid[x + 1 + (y + 1) * width] == '.' {
                // bubble down-right
                y += 1;
                x += 1;
            } else if x == 0 || x == width - 1 {
                // the grid should not be too small
                unreachable!("Grid is too small!");
            } else {
                // cannot move any further
                grid[x + y * width] = 'o';
                cnt += 1;
                break;
            }
        }
    }

    cnt
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");

        let bbox = data.bbox();
        assert_eq!((494, 0, 504, 10), bbox);

        let (grid, width, (x0, y0)) = data.grid();
        assert_eq!((6, 0), (x0, y0));
        assert_eq!(10, width);
        let exp = "............................................#...##....#...#...###...#.........#.........#.#########.";
        assert_eq!(exp.chars().collect::<Vec<_>>(), grid);
        for row in 0..grid.len() / width {
            for col in 0..width {
                print!("{}", grid[col + row * width]);
            }
            println!();
        }
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(24, star_1(&data));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(93, star_2(&data));
    }
}
// end::tests[]
