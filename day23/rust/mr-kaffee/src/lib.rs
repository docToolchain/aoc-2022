use grid::Grid;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, &'static str, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 23,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &(|s| star_1(s)),
            exp: Some(3_815),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &(|s| star_2(s)),
            exp: Some(893),
        }),
    }
}

// tag::grid[]
mod grid {
    #[derive(Clone)]
    pub struct Grid<T>
    where
        T: Clone + PartialEq,
    {
        data: Vec<T>,
        width: usize,
        default: T,
        bbox: (usize, usize, usize, usize),
    }

    impl From<&str> for Grid<u16> {
        fn from(value: &str) -> Self {
            let width = value.as_bytes().iter().position(|&b| b == b'\n').unwrap();
            let data: Vec<u16> = value
                .as_bytes()
                .iter()
                .filter(|&&b| b != b'\n')
                .map(|&b| (b == b'#') as u16)
                .collect();
            assert!(
                (data.len() / width) * width == data.len(),
                "All lines shall have equal length {width}"
            );
            let bbox = Self::get_bbox(&data, width, 0);
            let default = 0;
            Self {
                data,
                width,
                bbox,
                default,
            }
        }
    }

    impl std::fmt::Display for Grid<u16> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let (col_min, row_min, col_max, row_max) = self.bbox;
            for row in row_min..row_max {
                for col in col_min..col_max {
                    if self.data[col + row * self.width] & 255 > 0 {
                        '#'.fmt(f)?;
                    } else {
                        '.'.fmt(f)?;
                    }
                }
                '\n'.fmt(f)?;
            }
            Ok(())
        }
    }

    impl<T> Grid<T>
    where
        T: Copy + PartialEq,
    {
        fn get_bbox(data: &[T], width: usize, default: T) -> (usize, usize, usize, usize) {
            data.iter()
                .enumerate()
                .filter(|&(_, e)| e != &default)
                .map(|(pos, _)| (pos % width, pos / width))
                .fold((usize::MAX, usize::MAX, 0, 0), Self::add_to_bbox)
        }

        fn add_to_bbox(
            (col_min, row_min, col_max, row_max): (usize, usize, usize, usize),
            (col, row): (usize, usize),
        ) -> (usize, usize, usize, usize) {
            (
                col_min.min(col),
                row_min.min(row),
                col_max.max(col + 1),
                row_max.max(row + 1),
            )
        }

        /// make sure there is at least `gap` space between bounding
        /// box and end of grid
        pub fn increase_to_gap(
            &mut self,
            gap_min: usize,
            gap_max: usize,
        ) -> (usize, usize, usize, usize) {
            let (col_min, row_min, col_max, row_max) = self.bbox;
            let height = self.data.len() / self.width;

            if col_min < gap_min
                || row_min < gap_min
                || self.width - col_max < gap_min
                || height - row_max < gap_min
            {
                let (gap_c_l, gap_r_t, gap_c_r, gap_r_b) = (
                    gap_max - col_min.min(gap_max),
                    gap_max - row_min.min(gap_max),
                    gap_max - (self.width - col_max).min(gap_max),
                    gap_max - (height - row_max).min(gap_max),
                );

                let mut data = vec![
                    self.default;
                    (self.width + gap_c_l + gap_c_r) * (height + gap_r_t + gap_r_b)
                ];
                for row in 0..height {
                    let off = (row + gap_r_t) * (self.width + gap_c_l + gap_c_r) + gap_c_l;
                    data[off..off + self.width]
                        .clone_from_slice(&self.data[row * self.width..(row + 1) * self.width]);
                }

                self.data = data;
                self.width += gap_c_l + gap_c_r;
                self.bbox = (
                    self.bbox.0 + gap_c_l,
                    self.bbox.1 + gap_r_t,
                    self.bbox.2 + gap_c_l,
                    self.bbox.3 + gap_r_t,
                );

                (gap_c_l, gap_r_t, gap_c_r, gap_r_b)
            } else {
                (0, 0, 0, 0)
            }
        }

        /// set value and increase bbox if necessary
        pub fn set(&mut self, (col, row): (usize, usize), value: T) {
            self.data[col + row * self.width] = value;
            if value != self.default {
                self.bbox = Self::add_to_bbox(self.bbox, (col, row));
            }
        }

        pub fn get(&self, (col, row): (usize, usize)) -> T {
            self.data[col + row * self.width]
        }

        /// get current bbox
        pub fn bbox(&self) -> (usize, usize, usize, usize) {
            self.bbox
        }

        pub fn list<F>(&self, predicate: F) -> Vec<(usize, usize)>
        where
            F: Fn(&T) -> bool,
        {
            self.data
                .iter()
                .enumerate()
                .filter(|(_, v)| predicate(v))
                .map(|(pos, _)| (pos % self.width, pos / self.width))
                .collect()
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }
    }
}
// end::grid[]

// tag::star_1[]
/// helper function to get adjacent in given direction
fn adjacent((col, row): (usize, usize), dir: usize) -> (usize, usize) {
    let (d_col, d_row) = DELTAS[dir];
    (
        col.wrapping_add_signed(d_col),
        row.wrapping_add_signed(d_row),
    )
}

pub fn simulate_round(
    elves: &mut Vec<(usize, usize)>,
    grid: &mut Grid<u16>,
    targets: &mut Vec<(usize, (usize, usize))>,
    search_start: usize,
) -> bool {
    // make sure grid has enough space, require at least a gap of 1
    // in every direction, increase to gap of 10 if not satisfied
    let (col_off, row_off, _, _) = grid.increase_to_gap(1, 10);

    // clear buffer keeping capacity
    targets.truncate(0);

    // loop over elves
    for idx in 0..elves.len() {
        // apply offset
        elves[idx].0 += col_off;
        elves[idx].1 += row_off;

        // get coordinates
        let (col, row) = elves[idx];

        // if elf has no neighbors, don't move
        if (0..DELTAS.len()).all(|dir| grid.get(adjacent((col, row), dir)) & 255 == 0) {
            continue;
        }

        // find a direction to move to, starting from given search_start
        if let Some((tar_col, tar_row)) = (0..DIRS.len())
            .map(|pos| DIRS[(pos + search_start) % DIRS.len()])
            .find(|&dir| {
                [
                    (dir + DELTAS.len() - 1) % DELTAS.len(),
                    dir,
                    (dir + 1) % DELTAS.len(),
                ]
                .iter()
                .all(|&dir| grid.get(adjacent((col, row), dir)) & 255 == 0)
            })
            .map(|dir| adjacent((col, row), dir))
        {
            // increase 2nd byte by one (counting elves with the same target)
            let tar = grid.get((tar_col, tar_row)) + (1 << 8);
            grid.set((tar_col, tar_row), tar);

            // if first to have this target, push to targets
            if tar >> 8 == 1 {
                targets.push((idx, (tar_col, tar_row)));
            }
        }
    }

    // there are no targets -> no elves move anymore
    if targets.is_empty() {
        return false;
    }

    // update elves to targets where applicable
    for &(idx, (col_tar, row_tar)) in targets.iter() {
        if grid.get((col_tar, row_tar)) >> 8 == 1 {
            // can move -> move
            grid.set(elves[idx], 0);
            grid.set((col_tar, row_tar), 1);
            elves[idx] = (col_tar, row_tar);
        } else {
            // cannot move -> reset target count
            grid.set((col_tar, row_tar), 0);
        }
    }

    // some elves moved
    true
}

pub fn simulate(s: &str, rounds: usize) -> (usize, Vec<(usize, usize)>) {
    // build grid
    let mut grid = Grid::from(s);
    // build list of elves
    let mut elves = grid.list(|&v| v & 255 > 0);
    // buffer to store targets
    let mut targets = Vec::with_capacity(elves.len());

    // loop over rounds
    let mut round = 0;
    while round < rounds {
        if !simulate_round(&mut elves, &mut grid, &mut targets, round % 4) {
            // stop if no more elves move
            break;
        }
        round += 1;
    }

    (round + 1, elves)
}

pub const DELTAS: [(isize, isize); 8] = [
    (1, 1),   // SOUTH EAST
    (0, 1),   // SOUTH
    (-1, 1),  // SOUTH WEST
    (-1, 0),  // WEST
    (-1, -1), // NORTH WEST
    (0, -1),  // NORTH
    (1, -1),  // NORTH EAST
    (1, 0),   // EAST
];

/// NORTH, SOUTH, WEST, EAST
pub const DIRS: [usize; 4] = [5, 1, 3, 7];

pub fn star_1(data: &str) -> usize {
    let (_, elves) = simulate(data, 10);
    let (col_min, row_min, col_max, row_max) = elves.iter().fold(
        (usize::MAX, usize::MAX, 0, 0),
        |(col_min, row_min, col_max, row_max), &(col, row)| {
            (
                col_min.min(col),
                row_min.min(row),
                col_max.max(col + 1),
                row_max.max(row + 1),
            )
        },
    );

    (col_max - col_min) * (row_max - row_min) - elves.len()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &str) -> usize {
    simulate(data, usize::MAX).0
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_star_1() {
        assert_eq!(110, star_1(CONTENT));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(20, star_2(CONTENT));
    }

    #[test]
    pub fn test_increase_to_gap() {
        let mut grid = Grid::from("#....\n.#...\n..#..");
        let str_a = grid.to_string();

        // check initial conditions
        assert_eq!((0, 0, 3, 3), grid.bbox(), "incorrect bbox");
        assert_eq!(5, grid.width(), "unexpected width");
        assert_eq!(5 * 3, grid.len(), "unexpected length");

        // perform and verify increase
        assert_eq!(
            (2, 2, 0, 2),
            grid.increase_to_gap(1, 2),
            "unexpected amount of increase"
        );
        assert_eq!(
            (0, 0, 0, 0),
            grid.increase_to_gap(2, 10),
            "unexpected increase beyond min"
        );
        assert_eq!((2, 2, 5, 5), grid.bbox(), "unexpected bbox after increase");
        assert_eq!(7, grid.width(), "unexpected width after increase");
        assert_eq!(7 * 7, grid.len(), "unexpected length after increase");
        assert_eq!(
            str_a,
            grid.to_string(),
            "string representation changed after increase"
        );
    }

    #[test]
    pub fn test_simulate_round() {
        let mut grid = Grid::from(CONTENT_SMALL);
        let mut elves = grid.list(|v| v & 255 > 0);
        let mut targets = Vec::with_capacity(elves.len());

        println!("{grid}");
        simulate_round(&mut elves, &mut grid, &mut targets, 0);
        println!("{grid}");
        simulate_round(&mut elves, &mut grid, &mut targets, 1);
        println!("{grid}");
        simulate_round(&mut elves, &mut grid, &mut targets, 2);
        println!("{grid}");
    }

    const CONTENT: &str = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
"#;

    const CONTENT_SMALL: &str = r#".....
..##.
..#..
.....
..##.
.....
"#;
}
// end::tests[]
