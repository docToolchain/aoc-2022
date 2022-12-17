use input::*;
use mr_kaffee_aoc::{Puzzle, Star};
use std::collections::HashSet;

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 15,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(4_827_924),
        }),
        star2: Some(Star {
            name: "Star 2",
            #[cfg(feature = "scan")]
            f: &star_2_scan_lines,
            #[cfg(not(feature = "scan"))]
            f: &star_2_geometry,
            exp: Some(12_977_110_973_564),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        sensors: Vec<((isize, isize), (isize, isize))>,
        pub row: isize,
        pub width: isize,
    }

    fn parse_line(l: &str) -> ((isize, isize), (isize, isize)) {
        let words = l.split_ascii_whitespace();
        let mut words = words.skip(2); // Sensor at

        let x_s = words.next().unwrap();
        let x_s = x_s.strip_prefix("x=").unwrap().strip_suffix(",").unwrap();
        let x_s = x_s.parse().unwrap();

        let y_s = words.next().unwrap();
        let y_s = y_s.strip_prefix("y=").unwrap().strip_suffix(":").unwrap();
        let y_s = y_s.parse().unwrap();

        let mut words = words.skip(4); // closest beacon is at

        let x_b = words.next().unwrap();
        let x_b = x_b.strip_prefix("x=").unwrap().strip_suffix(",").unwrap();
        let x_b = x_b.parse().unwrap();

        let y_b = words.next().unwrap();
        let y_b = y_b.strip_prefix("y=").unwrap();
        let y_b = y_b.parse().unwrap();

        ((x_s, y_s), (x_b, y_b))
    }

    impl From<&str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &str) -> Self {
            Self {
                sensors: s.lines().map(parse_line).collect(),
                row: 2_000_000,
                width: 4_000_000,
            }
        }
    }

    impl PuzzleData {
        pub fn sensors(&self) -> &[((isize, isize), (isize, isize))] {
            &self.sensors
        }

        pub fn sensors_with_r(&self) -> Vec<((isize, isize), isize)> {
            self.sensors
                .iter()
                .map(|((x, y), (x_b, y_b))| ((*x, *y), (x - x_b).abs() + (y - y_b).abs()))
                .collect()
        }
    }
}
// end::input[]

// tag::star_1[]
/// determine ranges covered by sensors on given row
pub fn ranges(
    sensors: &[((isize, isize), (isize, isize))],
    mn: isize,
    mx: isize,
    row: isize,
) -> Vec<(isize, isize)> {
    sensors
        .iter()
        .map(|((x, y), (x_b, y_b))| (*x, (x - x_b).abs() + (y - y_b).abs() - (y - row).abs()))
        .map(|(x, d)| ((x - d).max(mn), (x + d).min(mx)))
        .filter(|(mn, mx)| mx >= mn)
        .fold(Vec::new(), |mut ranges, range| {
            ranges.push(range);
            let mut k_2 = ranges.len() - 1;
            for k_1 in (0..ranges.len() - 1).rev() {
                if ranges[k_1].0 <= ranges[k_2].0 && ranges[k_1].1 >= ranges[k_2].1 {
                    // k_2 contained in k_1
                    ranges.swap_remove(k_2);
                    break;
                } else if ranges[k_2].0 <= ranges[k_1].0 && ranges[k_2].1 >= ranges[k_1].1 {
                    // k_1 contained in k_2
                    ranges.swap_remove(k_1);
                    k_2 = if k_2 == ranges.len() { k_1 } else { k_2 };
                } else if ranges[k_2].0 >= ranges[k_1].0 && ranges[k_2].0 <= ranges[k_1].1 + 1 {
                    // k_2's min in k_1 or immediately after
                    ranges[k_1].1 = ranges[k_2].1;
                    ranges.swap_remove(k_2);
                    k_2 = k_1;
                } else if ranges[k_2].1 >= ranges[k_1].0 - 1 && ranges[k_2].1 <= ranges[k_1].1 {
                    // k_2's max in k_1 or immediately before
                    ranges[k_1].0 = ranges[k_2].0;
                    ranges.swap_remove(k_2);
                    k_2 = k_1;
                }
            }
            ranges
        })
}

pub fn star_1(data: &PuzzleData) -> usize {
    let ranges: Vec<(isize, isize)> = ranges(data.sensors(), isize::MIN, isize::MAX, data.row);
    let r = ranges
        .iter()
        .map(|(mn, mx)| (mx - mn + 1) as usize)
        .sum::<usize>();
    let s = data
        .sensors()
        .iter()
        .filter(|(_, (_, y))| *y == data.row)
        .map(|(_, b)| b)
        .collect::<HashSet<_>>()
        .len();
    r - s
}
// end::star_1[]

// tag::star_2_scan[]
pub fn star_2_scan_lines(data: &PuzzleData) -> usize {
    let (y, ranges) = (0..data.width)
        .map(|row| (row, ranges(data.sensors(), 0, data.width, row)))
        .find(|(_, r)| r.len() == 2)
        .unwrap();

    let x = ranges[0].0.max(ranges[1].0) - 1;

    (x * 4_000_000 + y) as _
}
// end::star_2_scan[]

// tag::star_2[]
pub fn is_distress_beacon(
    sensors: &[((isize, isize), isize)],
    p: &(isize, isize),
    width: isize,
) -> bool {
    (0..=width).contains(&p.0)
        && (0..=width).contains(&p.1)
        && sensors
            .iter()
            .all(|((x, y), r)| (x - p.0).abs() + (y - p.1).abs() > *r)
}

pub fn candidates(
    (((x_1, y_1), r_1), ((x_2, y_2), r_2)): (&((isize, isize), isize), &((isize, isize), isize)),
) -> HashSet<(isize, isize)> {
    let dx = x_2 - x_1;
    let dy = y_2 - y_1;
    let dr = r_2 - r_1;

    [
        // TL - RT
        // x1+(dx+dy-dr)/2,y1-r1-1+(dx+dy-dr)/2
        (
            x_1 + (dx + dy - dr + 1) / 2,
            y_1 - r_1 - 1 + (dx + dy - dr) / 2,
        ),
        (x_1 + (dx + dy - dr) / 2, y_1 - r_1 - 1 + (dx + dy - dr) / 2),
        // LB - BR
        // x1+(dx+dy+dr)/2,y1+r1+1+(dx+dy+dr)/2
        (
            x_1 + (dx + dy + dr - 1) / 2,
            y_1 + r_1 + 1 + (dx + dy + dr) / 2,
        ),
        (x_1 + (dx + dy + dr) / 2, y_1 + r_1 + 1 + (dx + dy + dr) / 2),
        // RT - TL
        // x1+(dx-dy+dr)/2,y1-r1-1-(dx-dy+dr)/2
        (
            x_1 + (dx - dy + dr - 1) / 2,
            y_1 - r_1 - 1 - (dx - dy + dr) / 2,
        ),
        (x_1 + (dx - dy + dr) / 2, y_1 - r_1 - 1 - (dx - dy + dr) / 2),
        // BR - LB
        // x1+(dx-dy-dr)/2,y1+r1+1-(dx-dy-dr)/2
        (
            x_1 + (dx - dy - dr + 1) / 2,
            y_1 + r_1 + 1 - (dx - dy - dr) / 2,
        ),
        (x_1 + (dx - dy - dr) / 2, y_1 + r_1 + 1 - (dx - dy - dr) / 2),
        // LB - TL
        // x1-r1-1+(dx+dy-dr)/2,y1+(dx+dy-dr)/2
        (
            x_1 - r_1 - 1 + (dx + dy - dr) / 2,
            y_1 + (dx + dy - dr + 1) / 2,
        ),
        (x_1 - r_1 - 1 + (dx + dy - dr) / 2, y_1 + (dx + dy - dr) / 2),
        // BR - RT
        // x1+r1+1+(dx-dy+dr)/2,y1-(dx-dy+dr)/2
        (
            x_1 + r_1 + 1 + (dx - dy + dr) / 2,
            y_1 - (dx - dy + dr - 1) / 2,
        ),
        (x_1 + r_1 + 1 + (dx - dy + dr) / 2, y_1 - (dx - dy + dr) / 2),
        // TL - LB
        // x1-r1-1+(dx-dy-dr)/2,y1-(dx-dy-dr)/2
        (
            x_1 - r_1 - 1 + (dx - dy - dr) / 2,
            y_1 - (dx - dy - dr + 1) / 2,
        ),
        (x_1 - r_1 - 1 + (dx - dy - dr) / 2, y_1 - (dx - dy - dr) / 2),
        // RT - BR
        // x2+r2+1+(dx+dy+dr)/2,y2+(dx+dy+dr)/2
        (
            x_1 + r_1 + 1 + (dx + dy + dr) / 2,
            y_1 + (dx + dy + dr - 1) / 2,
        ),
        (x_1 + r_1 + 1 + (dx + dy + dr) / 2, y_1 + (dx + dy + dr) / 2),
    ]
    .into_iter()
    .filter(|(x, y)| {
        (1..=2).contains(&((x - x_1).abs() + (y - y_1).abs() - r_1))
            && (1..=2).contains(&((x - x_2).abs() + (y - y_2).abs() - r_2))
    })
    .collect()
}

fn star_2_geometry(data: &PuzzleData) -> usize {
    let sensors = data.sensors_with_r();
    let (x, y) = sensors
        .iter()
        .zip(sensors.iter().skip(1))
        .map(candidates)
        .map(HashSet::into_iter)
        .flatten()
        .find(|p| is_distress_beacon(&sensors, p, data.width))
        .unwrap();

    (x * 4_000_000 + y) as _
}
// end::star_2[]

/// don't try this :)
pub fn star_2_brute_force(data: &PuzzleData) -> usize {
    let k = (0..data.width * data.width)
        .find(|k| {
            data.sensors().iter().all(|((x_s, y_s), (x_b, y_b))| {
                let d_b = (x_s - x_b).abs() + (y_s - y_b).abs();
                let d_k = (x_s - k % data.width).abs() + (y_s - k / data.width).abs();
                d_b < d_k
            })
        })
        .unwrap();
    ((k % data.width) * 4_000_000 + k / data.width) as _
}

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    pub fn test_is_distress_beacon() {
        let data = PuzzleData::from(CONTENT);
        let sensors = data.sensors_with_r();
        assert!(is_distress_beacon(&sensors, &(14, 11), 20));
    }

    #[test]
    pub fn test_candidates() {
        // 0...#..%...
        // 1..#.#%.%..
        // 2.#.A%#B.%.
        // 3..#.#%.%..
        // 4...#..%...
        //  0123456789
        let s1 = ((3, 2), 2);
        let s2 = ((6, 2), 2);
        let e = HashSet::from([(4, 0), (5, 0), (4, 4), (5, 4)]);
        assert_eq!(
            e,
            candidates((&s1, &s2)).intersection(&e).cloned().collect()
        );
        assert_eq!(
            e,
            candidates((&s2, &s1)).intersection(&e).cloned().collect()
        );

        // 0...#...%..
        // 1..#.#.%.%.
        // 2.#.A.$.B.%
        // 3..#.#.%.%.
        // 4...#...%..
        //  0123456789
        //
        // RT: [x1 + r1 + 1 - k1, y1 - k1];
        // TL: [x2 - k2, y2 - r2 - 1 + k2];
        // [x1+(dx+dy-dr)/2,y1-r1-1+(dx+dy-dr)/2];
        // --- [x1+(dx-dy+dr)/2,y1-r1-1-(dx-dy+dr)/2];
        //
        // BR: [x1 + k1, y1 + r1 + 1 - k1];
        // LB: [x2 - r2 - 1 + k2, y2 + k2];
        // [x1+(dx-dy-dr)/2,y1+r1+1-(dx-dy-dr)/2];
        // --- [x1+(dx+dy+dr)/2,y1+r1+1+(dx+dy+dr)/2];
        let s1 = ((3, 2), 2);
        let s2 = ((7, 2), 2);
        let e = HashSet::from([(5, 1), (5, 3)]);
        assert_eq!(
            e,
            candidates((&s1, &s2)).intersection(&e).cloned().collect()
        );
        assert_eq!(
            e,
            candidates((&s2, &s1)).intersection(&e).cloned().collect()
        );

        // 0...#....
        // 1..#.#...
        // 2.#.A%#..
        // 3..#%#%..
        // 4..%#..%.
        // 5.%..B..%
        // 6..%...%.
        // 7...%.%..
        // 8....%...
        //  01234567
        // LB: [x1 - r1 - 1 + k1, y1 + k1];
        // TL: [x2 - k2, y2 - r2 - 1 + k2];
        // [x1-r1-1+(dx+dy-dr)/2,y1+(dx+dy-dr)/2]
        // --- [x1-r1-1+(dx-dy-dr)/2,y1-(dx-dy-dr)/2]
        let s1 = ((3, 2), 2);
        let s2 = ((4, 5), 3);
        let e = HashSet::from([(1, 3), (1, 4), (6, 2), (6, 3)]);
        assert_eq!(
            e,
            candidates((&s1, &s2)).intersection(&e).cloned().collect()
        );
        assert_eq!(
            e,
            candidates((&s2, &s1)).intersection(&e).cloned().collect()
        );

        // 0...#...
        // 1..#.#..
        // 2.#.A.#.
        // 3..#.#..
        // 4...$...
        // 5.-%.%..
        // 6.%.B.%.
        // 7..%.%..
        // 8...%...
        //  0123456
        let s1 = ((3, 2), 2);
        let s2 = ((3, 6), 2);
        let e = HashSet::from([(2, 4), (4, 4)]);
        assert_eq!(
            e,
            candidates((&s1, &s2)).intersection(&e).cloned().collect()
        );
        assert_eq!(
            e,
            candidates((&s2, &s1)).intersection(&e).cloned().collect()
        );
    }

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let mut data = PuzzleData::from(CONTENT);
        data.row = 10;
        assert_eq!(26, star_1(&data));
    }

    #[test]
    pub fn test_solve_2() {
        let mut data = PuzzleData::from(CONTENT);
        data.width = 20;
        assert_eq!(56_000_011, star_2_geometry(&data));
        assert_eq!(56_000_011, star_2_scan_lines(&data));
        assert_eq!(56_000_011, star_2_brute_force(&data));
    }
}
// end::tests[]
