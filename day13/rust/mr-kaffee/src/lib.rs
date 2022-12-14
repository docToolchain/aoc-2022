use mr_kaffee_aoc::Puzzle;

#[cfg(not(feature = "no-heap"))]
pub fn puzzle() -> Puzzle<'static, tree::input::PuzzleData, usize, usize, usize, usize> {
    tree::puzzle()
}

#[cfg(feature = "no-heap")]
pub fn puzzle() -> Puzzle<'static, iter::input::PuzzleData<'static>, usize, usize, usize, usize> {
    iter::puzzle()
}

pub mod tree {
    use self::input::PuzzleData;
    use self::node::Node;
    use mr_kaffee_aoc::{Puzzle, Star};
    use std::cmp::Ordering;

    /// the puzzle
    pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
        Puzzle {
            year: 2022,
            day: 13,
            input: include_str!("../input.txt"),
            star1: Some(Star {
                name: "Star 1",
                f: &star_1,
                exp: Some(5_675),
            }),
            star2: Some(Star {
                name: "Star 2",
                f: &star_2,
                exp: Some(20_383),
            }),
        }
    }

    // tag::input[]
    pub mod input {
        use super::node::Node;

        pub struct PuzzleData {
            pub nodes: Vec<Node>,
        }

        impl<'a> From<&'a str> for PuzzleData {
            fn from(s: &'a str) -> Self {
                Self {
                    nodes: s
                        .lines()
                        .filter(|l| !l.is_empty())
                        .map(|l| Node::from(l))
                        .collect(),
                }
            }
        }
    }
    // end::input[]

    // tag::node[]
    pub mod node {
        //! Tree structure for recursive lists from puzzle
        use std::cmp::Ordering;

        #[derive(Debug, Eq, PartialEq, Clone)]
        pub enum Node {
            List(Box<[Node]>),
            Value(usize),
        }

        impl std::fmt::Display for Node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Node::List(list) => {
                        '['.fmt(f)?;
                        for n in list.iter().take(1) {
                            n.fmt(f)?;
                        }
                        for n in list.iter().skip(1) {
                            ','.fmt(f)?;
                            n.fmt(f)?;
                        }
                        ']'.fmt(f)?;
                    }
                    Node::Value(value) => value.fmt(f)?,
                }

                Ok(())
            }
        }

        impl<T> From<T> for Node
        where
            T: AsRef<[u8]>,
        {
            fn from(s: T) -> Self {
                Self::parse(s.as_ref(), 0).0
            }
        }

        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                match (self, other) {
                    (Node::List(lhs), Node::List(rhs)) => {
                        for k in 0..lhs.len().min(rhs.len()) {
                            let o = lhs[k].cmp(&rhs[k]);
                            if o != Ordering::Equal {
                                return o;
                            }
                        }
                        lhs.len().cmp(&rhs.len())
                    }
                    (Node::Value(lhs), Node::Value(rhs)) => lhs.cmp(rhs),
                    (_, Node::Value(rhs)) => self.cmp(&Node::List(vec![Node::Value(*rhs)].into())),
                    (Node::Value(lhs), _) => Node::List(vec![Node::Value(*lhs)].into()).cmp(other),
                }
            }
        }

        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Node {
            fn parse(s: &[u8], pos: usize) -> (Self, usize) {
                if s[pos] == b'[' && s[pos + 1] == b']' {
                    // handle empty list separately
                    (Self::List(vec![].into()), 2)
                } else if s[pos] == b'[' {
                    let mut v = Vec::new();
                    let mut len = 1;
                    loop {
                        let (n, l) = Self::parse(s, pos + len);
                        v.push(n);
                        len += l + 1;
                        if s[pos + len - 1] == b']' {
                            break;
                        }
                    }
                    (Self::List(v.into()), len)
                } else {
                    let mut v = 0;
                    let mut len = 0;
                    while pos + len < s.len() && s[pos + len] >= b'0' && s[pos + len] <= b'9' {
                        v = v * 10 + (s[pos + len] - b'0') as usize;
                        len += 1;
                    }
                    (Self::Value(v), len)
                }
            }
        }
    }
    // end::node[]

    // tag::star_1[]
    pub fn star_1(data: &PuzzleData) -> usize {
        data.nodes
            .iter()
            .step_by(2)
            .zip(data.nodes.iter().skip(1).step_by(2))
            .enumerate()
            .filter(|(_, (a, b))| a.cmp(b) != Ordering::Greater)
            .fold(0, |s, (k, _)| s + k + 1)
    }
    // end::star_1[]

    // tag::star_2[]
    pub fn star_2(data: &PuzzleData) -> usize {
        let mut v = data.nodes.clone();
        v.push(Node::from("[[2]]"));
        v.push(Node::from("[[6]]"));
        v.sort_unstable();

        let a = v.iter().position(|n| n == &Node::from("[[2]]")).unwrap();
        let b = v.iter().position(|n| n == &Node::from("[[6]]")).unwrap();

        (a + 1) * (b + 1)
    }
    // end::star_2[]

    // tag::tests[]
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::tree::node::Node;

        #[test]
        pub fn test_parse() {
            let s = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
            let n = Node::from(s);
            println!("{n:?}");
            assert_eq!(s, n.to_string());
        }

        #[test]
        pub fn test_cmp() {
            let nodes = PuzzleData::from(crate::tests::CONTENT).nodes;
            let cmp = nodes
                .iter()
                .step_by(2)
                .zip(nodes.iter().skip(1).step_by(2))
                .map(|(a, b)| a.cmp(b))
                .collect::<Vec<_>>();
            println!("{cmp:?}");
            assert_eq!(
                vec![
                    Ordering::Less,
                    Ordering::Less,
                    Ordering::Greater,
                    Ordering::Less,
                    Ordering::Greater,
                    Ordering::Less,
                    Ordering::Greater,
                    Ordering::Greater
                ],
                cmp
            );
        }

        #[test]
        pub fn test_star_1() {
            assert_eq!(13, star_1(&PuzzleData::from(crate::tests::CONTENT)))
        }

        #[test]
        pub fn test_star_2() {
            assert_eq!(140, star_2(&PuzzleData::from(crate::tests::CONTENT)))
        }
    }
    // end::tests[]
}

// tag::no-heap[]
pub mod iter {
    use self::node::List;
    use self::{input::PuzzleData, node::Node};
    use mr_kaffee_aoc::{Puzzle, Star};
    use std::cmp::Ordering;

    /// the puzzle
    pub fn puzzle() -> Puzzle<'static, PuzzleData<'static>, usize, usize, usize, usize> {
        Puzzle {
            year: 2022,
            day: 13,
            input: include_str!("../input.txt"),
            star1: Some(Star {
                name: "Star 1",
                f: &star_1,
                exp: Some(5_675),
            }),
            star2: Some(Star {
                name: "Star 2",
                f: &star_2,
                exp: Some(20_383),
            }),
        }
    }

    pub mod input {
        pub struct PuzzleData<'a> {
            pub input: &'a str,
        }

        impl<'a> From<&'a str> for PuzzleData<'a> {
            fn from(input: &'a str) -> Self {
                Self { input }
            }
        }
    }

    pub mod node {
        use std::{cmp::Ordering, iter::once};

        #[derive(Debug, Clone)]
        pub struct List<'a> {
            data: &'a [u8],
            pos: usize,
        }

        #[derive(Debug, Clone)]
        pub enum Node<'a> {
            List(List<'a>),
            Value(usize),
        }

        impl<'a> std::fmt::Display for Node<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Node::List(list) => list.fmt(f),
                    Node::Value(value) => value.fmt(f),
                }
            }
        }

        impl<'a> From<&'a str> for Node<'a> {
            fn from(s: &'a str) -> Self {
                if s.starts_with('[') {
                    Self::List(List::from(s))
                } else {
                    Self::Value(s.parse().unwrap())
                }
            }
        }

        impl<'a> std::fmt::Display for List<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut level = 1;
                '['.fmt(f)?;
                for &b in &self.data[self.pos..] {
                    level = match b {
                        b'[' => level + 1,
                        b']' => level - 1,
                        _ => level,
                    };
                    (b as char).fmt(f)?;
                    if level == 0 {
                        break;
                    }
                }
                Ok(())
            }
        }

        impl<'a> From<&'a [u8]> for List<'a> {
            fn from(s: &'a [u8]) -> Self {
                let data = s.as_ref();
                let pos = if data[0] == b'[' { 1 } else { 0 };
                Self { data, pos }
            }
        }

        impl<'a> From<&'a str> for List<'a> {
            fn from(s: &'a str) -> Self {
                Self::from(s.as_bytes())
            }
        }

        impl<'a> Iterator for List<'a> {
            type Item = Node<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.pos >= self.data.len() || self.data[self.pos] == b']' {
                    // exhausted
                    None
                } else if self.data[self.pos] == b'[' {
                    // parse list
                    let nxt = Self::Item::List(List::from(&self.data[self.pos..]));

                    // advance pos to after list
                    self.pos += 2 + self.data[self.pos + 1..]
                        .iter()
                        .scan(1usize, |level, b| {
                            match b {
                                b'[' => *level += 1,
                                b']' => *level -= 1,
                                _ => (),
                            };
                            Some(*level)
                        })
                        .position(|level| level == 0)
                        .unwrap();

                    // skip ',' if applicable
                    if self.pos < self.data.len() && self.data[self.pos] == b',' {
                        self.pos += 1;
                    }

                    // return list
                    Some(nxt)
                } else {
                    // parse value
                    let mut v = 0;
                    while (b'0'..=b'9').contains(&self.data[self.pos]) {
                        // parse digit
                        v = 10 * v + (self.data[self.pos] - b'0') as usize;
                        self.pos += 1;
                    }

                    // skip ',' if applicable
                    if self.pos < self.data.len() && self.data[self.pos] == b',' {
                        self.pos += 1;
                    }

                    // return value
                    Some(Self::Item::Value(v))
                }
            }
        }

        impl<'a> Ord for Node<'a> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                match (self, other) {
                    (Node::Value(lhs), Node::Value(rhs)) => lhs.cmp(rhs),
                    (Node::List(lhs), Node::List(rhs)) => lhs.clone().cmp(rhs.clone()),
                    (Node::List(lhs), rhs) => lhs.clone().cmp(once(rhs.clone())),
                    (lhs, Node::List(rhs)) => once(lhs.clone()).cmp(rhs.clone()),
                }
            }
        }

        impl<'a> PartialOrd for Node<'a> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<'a> PartialEq for Node<'a> {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == Ordering::Equal
            }
        }

        impl<'a> Eq for Node<'a> {}
    }

    pub fn star_1(data: &PuzzleData) -> usize {
        let mut lines = data.input.lines().filter(|l| !l.is_empty());
        let mut idx = 0;
        let mut result = 0;
        while let (Some(a), Some(b)) = (lines.next(), lines.next()) {
            idx += 1;
            if Node::from(a).cmp(&Node::from(b)) != Ordering::Greater {
                result += idx;
            }
        }

        result
    }

    pub fn star_2(data: &PuzzleData) -> usize {
        let n_1 = List::from("[[2]]");
        let n_2 = List::from("[[6]]");

        let (cnt_1, cnt_2) =
            data.input
                .lines()
                .filter(|l| !l.is_empty())
                .fold((1, 2), |(cnt_1, cnt_2), l| {
                    if n_1.clone().cmp(List::from(l)) == Ordering::Greater {
                        (cnt_1 + 1, cnt_2 + 1)
                    } else if n_2.clone().cmp(List::from(l)) == Ordering::Greater {
                        (cnt_1, cnt_2 + 1)
                    } else {
                        (cnt_1, cnt_2)
                    }
                });

        cnt_1 * cnt_2
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::iter::node::{List, Node};

        #[test]
        pub fn test_next() {
            let mut list = List::from("[10,[[9,10,0],2]]");
            assert_eq!(Some("10".into()), list.next().map(|n| n.to_string()));
            assert_eq!(
                Some("[[9,10,0],2]".into()),
                list.next().map(|n| n.to_string())
            );
            assert!(list.next().is_none());
        }

        #[test]
        pub fn test_cmp() {
            let nodes = crate::tests::CONTENT
                .lines()
                .filter(|l| !l.is_empty())
                .map(List::from)
                .map(Node::List)
                .collect::<Vec<_>>();

            let cmp = nodes
                .iter()
                .step_by(2)
                .zip(nodes.iter().skip(1).step_by(2))
                .map(|(a, b)| a.cmp(b))
                .collect::<Vec<_>>();
            println!("{cmp:?}");
            assert_eq!(
                vec![
                    Ordering::Less,
                    Ordering::Less,
                    Ordering::Greater,
                    Ordering::Less,
                    Ordering::Greater,
                    Ordering::Less,
                    Ordering::Greater,
                    Ordering::Greater
                ],
                cmp
            );
        }

        #[test]
        pub fn test_star_1() {
            assert_eq!(13, star_1(&PuzzleData::from(crate::tests::CONTENT)))
        }

        #[test]
        pub fn test_star_2() {
            assert_eq!(140, star_2(&PuzzleData::from(crate::tests::CONTENT)))
        }
    }
}
// end::no-heap[]

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_compare_tree_iter() {
        let content = include_str!("../input.txt");
        let mut lines = content.lines().filter(|l| !l.is_empty());
        while let (Some(a), Some(b)) = (lines.next(), lines.next()) {
            let c1 = tree::node::Node::from(a).cmp(&tree::node::Node::from(b));
            let c2 = iter::node::Node::from(a).cmp(&iter::node::Node::from(b));
            assert_eq!(c1, c2, "Some result for\n  {a}\n  {b}");
        }
    }

    pub const CONTENT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;
}
