use input::*;
use mr_kaffee_aoc::{Puzzle, Star};
use std::{
    collections::{BinaryHeap, HashMap},
    iter::once,
};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData<'static>, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 16,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(2_359),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(2_999),
        }),
    }
}

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData<'a> {
        valves: Vec<Valve<'a>>,
        root: usize,
    }

    fn parse_line(line: &str) -> (&str, Vec<&str>, usize) {
        let mut words = line.split_ascii_whitespace().skip(1);

        let name = words.next().unwrap();

        let mut words = words.skip(2);

        let flow = words.next().unwrap();
        let flow = flow[5..flow.len() - 1].parse().unwrap();

        let tunnels = words
            .skip(4)
            .map(|word| word.trim_end_matches(','))
            .collect();

        (name, tunnels, flow)
    }

    impl<'a> From<&'a str> for PuzzleData<'a> {
        /// parse the puzzle input
        fn from(s: &'a str) -> Self {
            let lines = s.lines().map(parse_line).collect::<Vec<_>>();
            let valves = lines
                .iter()
                .enumerate()
                .map(|(idx, (name, tunnels, flow))| Valve {
                    name,
                    idx,
                    flow: *flow,
                    tunnels: tunnels
                        .iter()
                        .map(|&tunnel| {
                            lines
                                .iter()
                                .position(|&(name, _, _)| name == tunnel)
                                .unwrap()
                        })
                        .collect(),
                })
                .collect();
            let root = lines.iter().position(|&(name, _, _)| name == "AA").unwrap();
            Self { valves, root }
        }
    }

    impl<'a> PuzzleData<'a> {
        pub fn get(&self, idx: usize) -> &Valve {
            &self.valves[idx]
        }

        pub fn root(&self) -> &Valve {
            &self.valves[self.root]
        }

        pub fn valves(&self) -> &[Valve] {
            &self.valves
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Valve<'a> {
        pub idx: usize,
        pub name: &'a str,
        pub flow: usize,
        pub tunnels: Vec<usize>,
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    // search state
    // - pressure potential
    // - pressure
    // - flow (valves opened so far)
    // - idx (position)
    // - opened (valves opened so far)
    // - timer (time left before eruption)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct State {
        potential: usize,
        pressure: usize,
        flow: usize,
        idx: usize,
        opened: u64,
        timer: usize,
    }

    impl State {
        fn create(
            pressure: usize,
            flow: usize,
            idx: usize,
            opened: u64,
            timer: usize,
            data: &PuzzleData,
            max_flow: usize,
        ) -> Self {
            let mut potential = pressure;
            let mut flow_ = 0;
            if timer >= 1 {
                // in next step, pressure will be increased by flow
                flow_ += flow;
                potential += flow_;
            }
            if timer >= 2 {
                // in the 2nd step, pressure will at most be increased by flow
                // of current valve. If it is not opened in the next step
                // (agent moves instead), the flow will not change
                if opened & 1 << idx == 0 {
                    flow_ += data.get(idx).flow;
                }
                potential += flow_;

                // upper bound for subsequent steps: all valves open
                potential += (timer - 2) * max_flow;
            }

            Self {
                potential,
                pressure,
                flow,
                idx,
                opened,
                timer,
            }
        }
    }

    // all valves opened / max flow
    let (all_opened, max_flow) = data
        .valves()
        .iter()
        .filter(|v| v.flow > 0)
        .fold((0, 0), |(o, f), v| (o | 1 << v.idx, f + v.flow));

    // max time
    let timer: usize = 30;

    // start at root, no valves open
    let start = State::create(0, 0, data.root().idx, 0, timer, data, max_flow);

    // the queue for searching
    let mut queue = BinaryHeap::new();
    queue.push(start);

    // do not visit the same spot with the same opened valves again
    let mut seen = HashMap::from([((start.idx, start.opened), max_flow * timer)]);

    while let Some(s) = queue.pop() {
        // do not explore further if timer elapsed or all valves open,
        // just see if there is something better in the queue
        if s.timer == 0 || s.opened == all_opened {
            return s.potential;
        }

        let v = data.get(s.idx);

        let can_o = (s.opened & 1 << v.idx) == 0 && v.flow > 0;

        for (o, adj) in once((true, v))
            .chain(v.tunnels.iter().map(|&idx| (false, data.get(idx))))
            .filter(|&(o, _)| !o || can_o)
        {
            let opened = s.opened | if o { 1 << adj.idx } else { 0 };
            let flow = s.flow + if o { adj.flow } else { 0 };
            let next = State::create(
                s.pressure + s.flow,
                flow,
                adj.idx,
                opened,
                s.timer - 1,
                data,
                max_flow,
            );
            let v = seen.entry((next.idx, next.opened)).or_insert(0);
            if next.potential.gt(v) {
                *v = next.potential;
                queue.push(next);
            }
        }
    }

    unreachable!();
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    // search state
    // - pressure potential
    // - pressure
    // - flow (valves opened so far)
    // - idx (positions, sorted)
    // - opened (valves opened so far)
    // - timer (time left before eruption)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct State {
        potential: usize,
        pressure: usize,
        flow: usize,
        idx: [usize; 2],
        opened: u64,
        timer: usize,
    }

    impl State {
        fn create(
            pressure: usize,
            flow: usize,
            idx: [usize; 2],
            opened: u64,
            timer: usize,
            data: &PuzzleData,
            max_flow: usize,
        ) -> Self {
            let mut potential = pressure;
            let mut flow_ = 0;
            if timer >= 1 {
                // in next step, pressure will be increased by flow
                flow_ += flow;
                potential += flow_;
            }
            if timer >= 2 {
                // in the 2nd step, pressure will at most be increased by flow
                // of current valves. If they are not opened in the next step
                // (agents move instead), the flow will not change
                for idx_ in idx {
                    if opened & 1 << idx_ == 0 {
                        flow_ += data.get(idx_).flow;
                    }
                }
                potential += flow_;

                // upper bound for subsequent steps: all valves open
                potential += (timer - 2) * max_flow;
            }

            Self {
                potential,
                pressure,
                flow,
                idx,
                opened,
                timer,
            }
        }
    }

    // all valves opened / max flow
    let (all_opened, max_flow) = data
        .valves()
        .iter()
        .filter(|v| v.flow > 0)
        .fold((0, 0), |(o, f), v| (o | 1 << v.idx, f + v.flow));

    // max time
    let timer: usize = 26;

    // start at root, no valves open
    let start = State::create(0, 0, [data.root().idx; 2], 0, timer, data, max_flow);

    // the queue for searching
    let mut queue = BinaryHeap::new();
    queue.push(start);

    // do not visit the same spot with the same opened valves again
    let mut seen = HashMap::from([((start.idx, start.opened), start.potential)]);

    while let Some(s) = queue.pop() {
        // do not explore further if timer elapsed or all valves open,
        // just see if there is something better in the queue
        if s.timer == 0 || s.opened == all_opened {
            return s.potential;
        }

        let v_1 = data.get(s.idx[0]);
        let v_2 = data.get(s.idx[1]);

        let can_o_1 = (s.opened & 1 << v_1.idx) == 0 && v_1.flow > 0;
        let can_o_2 = v_1.idx != v_2.idx && (s.opened & 1 << v_2.idx) == 0 && v_2.flow > 0;

        for (o_1, adj_1) in once((true, v_1))
            .chain(v_1.tunnels.iter().map(|&idx| (false, data.get(idx))))
            .filter(|&(o, _)| !o || can_o_1)
        {
            for (o_2, adj_2) in once((true, v_2))
                .chain(v_2.tunnels.iter().map(|&idx| (false, data.get(idx))))
                .filter(|&(o, _)| !o || can_o_2)
            {
                let opened = s.opened
                    | if o_1 { 1 << adj_1.idx } else { 0 }
                    | if o_2 { 1 << adj_2.idx } else { 0 };
                let flow =
                    s.flow + if o_1 { adj_1.flow } else { 0 } + if o_2 { adj_2.flow } else { 0 };
                let next = State::create(
                    s.pressure + s.flow,
                    flow,
                    [adj_1.idx.min(adj_2.idx), adj_1.idx.max(adj_2.idx)],
                    opened,
                    s.timer - 1,
                    data,
                    max_flow,
                );
                let v = seen.entry((next.idx, next.opened)).or_insert(0);
                if next.potential.gt(v) {
                    *v = next.potential;
                    queue.push(next);
                }
            }
        }
    }

    unreachable!();
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    const CONTENT: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        let root = data.root();
        assert_eq!("AA", root.name);
        assert_eq!(
            vec!["DD", "II", "BB"],
            root.tunnels
                .iter()
                .map(|&idx| data.get(idx).name)
                .collect::<Vec<_>>()
        );
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(1_651, star_1(&data));
    }

    #[test]
    pub fn test_example() {
        let data = PuzzleData::from(CONTENT);
        let map = data
            .valves()
            .iter()
            .map(|v| (v.name, v.idx))
            .collect::<HashMap<_, _>>();

        let steps = [
            (1, false, "DD"),
            (2, true, "DD"),
            (3, false, "CC"),
            (4, false, "BB"),
            (5, true, "BB"),
            (6, false, "AA"),
            (7, false, "II"),
            (8, false, "JJ"),
            (9, true, "JJ"),
            (10, false, "II"),
            (11, false, "AA"),
            (12, false, "DD"),
            (13, false, "EE"),
            (14, false, "FF"),
            (15, false, "GG"),
            (16, false, "HH"),
            (17, true, "HH"),
            (18, false, "GG"),
            (19, false, "FF"),
            (20, false, "EE"),
            (21, true, "EE"),
            (22, false, "DD"),
            (23, false, "CC"),
            (24, true, "CC"),
        ];

        let max_flow: usize = data.valves().iter().map(|v| v.flow).sum();
        let timer = 30;

        #[derive(Debug)]
        struct State {
            potential: usize,
            pressure: usize,
            flow: usize,
            idx: usize,
            opened: u64,
            timer: usize,
        }

        let mut s = State {
            potential: timer * max_flow,
            pressure: 0,
            flow: 0,
            idx: *map.get("AA").unwrap(),
            opened: 0,
            timer: timer,
        };

        let mut seen = HashSet::new();
        seen.insert((s.idx, s.opened));

        for (minute, open, valve) in steps {
            let idx = *map.get(valve).unwrap();
            let v = data.get(idx);
            assert!(idx == s.idx || !open);
            assert!(open || data.get(s.idx).tunnels.contains(&idx));
            s = State {
                potential: s.pressure + s.flow + (s.timer - 1) * max_flow,
                pressure: s.pressure + s.flow,
                flow: if open { s.flow + v.flow } else { s.flow },
                idx: v.idx,
                opened: if open { s.opened | 1 << idx } else { s.opened },
                timer: s.timer - 1,
            };
            assert!(seen.insert((s.idx, s.opened)));
            println!(
                "{minute}, {} {valve}, open valves: {:?} ({s:?})",
                if open { "opened" } else { "moved to" },
                data.valves()
                    .iter()
                    .filter(|v| (s.opened & 1 << v.idx) > 0)
                    .map(|v| v.name)
                    .collect::<Vec<_>>()
            );
        }
        println!(
            "{} + {} * {} = {}, {}",
            s.pressure,
            s.timer,
            s.flow,
            s.pressure + s.timer * s.flow,
            s.potential
        );

        assert_eq!(1_651, s.potential);
    }

    #[test]
    pub fn test_star_2() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(1_707, star_2(&data));
    }
}
// end::tests[]
