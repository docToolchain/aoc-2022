use input::PuzzleData;
use mr_kaffee_aoc::{Puzzle, Star};
use node::Node;
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
mod input {
    use crate::node::Node;

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
mod node {
    //! Tree structure for recursive lists from puzzle
    use std::cmp::Ordering;

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Node {
        List(Box<Vec<Node>>),
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
            Self::parse(s.as_ref(), 0, "".to_string()).0
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
        fn parse(s: &[u8], pos: usize, prefix: String) -> (Self, usize) {
            if s[pos] == b'[' && s[pos + 1] == b']' {
                // handle empty list separately
                (Self::List(vec![].into()), 2)
            } else if s[pos] == b'[' {
                let mut v = Vec::new();
                let mut len = 1;
                loop {
                    let (n, l) = Self::parse(s, pos + len, format!("| {prefix}"));
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

    #[test]
    pub fn test_parse() {
        let s = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let n = Node::from(s);
        println!("{n:?}");
        assert_eq!(s, n.to_string());
    }

    #[test]
    pub fn test_cmp() {
        let nodes = PuzzleData::from(CONTENT).nodes;
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
    pub fn test_cmp_2() {
        let n = Node::from("[[[]]]");
        println!("{n:?}");
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(13, star_1(&PuzzleData::from(CONTENT)))
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(140, star_2(&PuzzleData::from(CONTENT)))
    }

    const CONTENT: &str = r#"[1,1,3,1,1]
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
// end::tests[]
