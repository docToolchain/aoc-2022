use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, isize, isize, isize, isize> {
    Puzzle {
        year: 2022,
        day: 20,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &(|data| mix(&data.numbers, 1)),
            exp: Some(7_004),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(17_200_008_919_529),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        pub numbers: Vec<isize>,
    }

    impl From<&str> for PuzzleData {
        /// parse the puzzle input
        fn from(s: &str) -> Self {
            Self {
                numbers: s.trim().lines().map(|l| l.parse().unwrap()).collect(),
            }
        }
    }
}
// end::input[]

// tag::step[]
pub fn mix_step(indices: &mut Vec<(usize, usize)>, numbers: &[isize], k: usize) {
    let v = numbers[k];
    let steps = v % (numbers.len() as isize - 1);

    // move in direction of lower number of steps
    let steps = if steps > (numbers.len() as isize - 1) / 2 {
        steps - (numbers.len() as isize - 1)
    } else if steps < -(numbers.len() as isize - 1) / 2 {
        steps + (numbers.len() as isize - 1)
    } else {
        steps
    };

    if steps != 0 {
        if steps > 0 {
            // k_pre k k_post .. idx idx_post => k_pre k_post .. idx k idx_post
            let idx = (0..steps).fold(k, |k, _| indices[k].1);
            let (k_pre, k_post) = indices[k];
            let (_, idx_post) = indices[idx];
            indices[k_post].0 = k_pre;
            indices[k_pre].1 = k_post;
            indices[k] = (idx, idx_post);
            indices[idx].1 = k;
            indices[idx_post].0 = k;
        } else {
            // idx_pre idx .. k_pre k k_post => idx_pre k idx .. k_pre k_post
            let idx = (steps..0).fold(k, |k, _| indices[k].0);
            let (k_pre, k_post) = indices[k];
            let (idx_pre, _) = indices[idx];
            indices[k_post].0 = k_pre;
            indices[k_pre].1 = k_post;
            indices[k] = (idx_pre, idx);
            indices[idx].0 = k;
            indices[idx_pre].1 = k;
        };
    }
}
// end::step[]

// tag::mix[]
pub fn mix(numbers: &[isize], times: usize) -> isize {
    let n = numbers.len();
    let mut indices = (0..n)
        .map(|k| ((k + n - 1) % n, (k + 1) % n))
        .collect::<Vec<_>>();

    let k0 = numbers.iter().position(|&v| v == 0).unwrap();

    for _ in 0..times {
        for k in 0..n {
            mix_step(&mut indices, numbers, k);
        }
    }

    let k1 = (0..1000).fold(k0, |k, _| indices[k].1);
    let k2 = (0..1000).fold(k1, |k, _| indices[k].1);
    let k3 = (0..1000).fold(k2, |k, _| indices[k].1);

    numbers[k1] + numbers[k2] + numbers[k3]
}
// end::mix[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> isize {
    mix(
        &data
            .numbers
            .iter()
            .map(|v| v * 811_589_153)
            .collect::<Vec<_>>(),
        10,
    )
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const CONTENT: &str = r#"1
2
-3
3
-2
0
4
"#;

    #[test]
    pub fn test_mix_step() {
        let n = 21;
        let numbers: &[isize] = &[
            0,
            -1,
            -2,
            1,
            2,
            n - 2,
            n - 1,
            n,
            2 * n - 4,
            2 * n - 3,
            2 * n - 2,
            2 * n - 1,
            2 * n,
            -n - 1,
            -n,
            -n + 1,
            -2 * n - 3,
            -2 * n - 2,
            -2 * n - 1,
            -2 * n,
            -2 * n + 1,
        ];

        println!(
            "{:?}",
            numbers
                .iter()
                .map(|v| (v, v % (n - 1), v.rem_euclid(n - 1)))
                .collect::<Vec<_>>()
        );

        let exp = vec![
            (-2 * n + 1, -1),       // 0
            (-2 * n + 1, 0),        // -1
            (-2 * n + 1, 0),        // -2
            (2, n - 2),             // 1
            (n - 1, n),             // 2
            (1, 2),                 // n - 2 = 19
            (n - 2, n),             // n - 1 = 20
            (2 * n - 4, 2 * n - 3), // n = 21
            (n - 2, n - 1),         // 2 n - 4 = 38
            (n, 2 * n - 4),         // 2 n - 3 = 39
            (2 * n - 3, 2 * n - 1), // 2 n - 2 = 40
            (2 * n, -n - 1),        // 2 n - 1 = 41
            (-n, -n + 1),           // 2 n = 42
            (2 * n - 2, 2 * n - 1), // -n-1 = -22
            (2 * n, -n - 1),        // -n = -21
            (-n, -2 * n - 3),       // -n+1 = -20
        ];
        let n = n as usize;

        let to_vec = |indices: &Vec<(usize, usize)>| {
            (0..n)
                .scan(0, |k, _| {
                    let v = numbers[*k];
                    *k = indices[*k].1;
                    Some(v)
                })
                .collect::<Vec<isize>>()
        };

        let indices = (0..n)
            .map(|k| ((k + n - 1) % n, (k + 1) % n))
            .collect::<Vec<_>>();
        assert_eq!(numbers, to_vec(&indices));

        for k in 0..n {
            let mut indices = (0..n)
                .map(|k| ((k + n - 1) % n, (k + 1) % n))
                .collect::<Vec<_>>();
            mix_step(&mut indices, numbers, k);
            println!("numbers[{k}] = {} => {:?}", numbers[k], to_vec(&indices));

            if k < exp.len() {
                assert_eq!(
                    exp[k],
                    (numbers[indices[k].0], numbers[indices[k].1]),
                    "at k = {k}"
                );
            }

            let mut k_fwd = 0;
            let mut k_rev = 0;
            let mut fwds = HashSet::from([0]);
            let mut revs = HashSet::from([0]);
            for _ in 0..n {
                k_fwd = indices[k_fwd].1;
                k_rev = indices[k_rev].1;
                fwds.insert(k_fwd);
                revs.insert(k_rev);
            }
            // full cycle in both directions
            assert_eq!(
                0, k_fwd,
                "incomplete forward cycle\nk: {k}\n numbers: {numbers:?}\n indices: {indices:?}"
            );
            assert_eq!(
                0, k_rev,
                "incomplete backwards cycle\nk: {k}\n numbers: {numbers:?}\n indices: {indices:?}"
            );
            assert_eq!(
                n,
                fwds.len(),
                "items not in forward cycle\nk: {k}\n numbers: {numbers:?}\n indices: {indices:?}"
            );
            assert_eq!(
                n,
                revs.len(),
                "items not in backwards cycle\nk: {k}\n numbers: {numbers:?}\n indices: {indices:?}"
            );
        }
    }

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(3, mix(&data.numbers, 1));
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(1_623_178_306, star_2(&data));
    }
}
// end::tests[]
