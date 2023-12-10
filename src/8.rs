use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
use num_integer::Integer;
use substring::Substring;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Left,
    Right,
}

fn parse_dir(c: char) -> Dir {
    match c {
        'L' => Dir::Left,
        'R' => Dir::Right,
        _ => panic!("wrong direction {c}"),
    }
}
type Node = (char, char, char);

fn encode_node(x: &str) -> Node {
    x.chars().collect_tuple().unwrap()
}

fn parse_line(l: &str) -> (Node, (Node, Node)) {
    let get = |o| encode_node(l.substring(o, o + 3));
    (get(0), (get(7), get(12)))
}

fn follow(direction: Dir, (left, right): (Node, Node)) -> Node {
    match direction {
        Dir::Left => left,
        Dir::Right => right,
    }
}

#[derive(Clone)]
struct Data {
    nodes: HashMap<Node, (Node, Node)>,
    directions: Vec<Dir>,
}

fn walking<'a>(start: Node, data: Data) -> impl Iterator<Item = Node> + 'a {
    data.directions.into_iter().cycle().scan(start, move |x, dir| {
        *x = follow(dir, *data.nodes.get(x)?);
        Some(*x)
    })
}

fn count_steps(data: Data, start: Node, end: Node) -> usize {
    walking(start, data).find_position(|&x| x == end).unwrap().0 + 1
}

fn ending_node(n: Node) -> bool {
    n.2 == 'Z'
}
fn start_node(n: &Node) -> bool {
    n.2 == 'A'
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cycle {
    start: isize,
    period: isize,
}

fn merge_cycles(x: Cycle, y: Cycle) -> Option<Cycle> {
    let period = x.period.lcm(&y.period);
    if x.start == y.start {
        Some(Cycle { period, start: x.start })
    } else if x.period - x.start == y.period - y.start {
        Some(Cycle {
            period,
            start: period - (x.period - x.start),
        })
    } else {
        None
    }
}

fn count_period(start: Node, data: Data) -> Vec<Cycle> {
    // returns the start, and period of all found cycles
    #[derive(Debug)]
    enum Visited {
        Once(usize),
        Twice((usize, usize)),
    }
    let mut visited: HashMap<(usize, Node), Visited> = HashMap::new();
    // (ic, node) -> Visited
    let len = data.directions.len();
    for (current, node) in walking(start, data).enumerate() {
        let local = current % len;
        if ending_node(node) {
            // every time we roll over the directions,
            // check if we visited that node before this instruction
            // i assume that once i'll visit one zs thrice it means all other zs are visited twice
            let k = (local, node);
            match visited.get(&k) {
                None => {
                    visited.insert(k, Visited::Once(current));
                }
                Some(Visited::Once(previous)) => {
                    visited.insert(k, Visited::Twice((*previous, current)));
                }
                Some(Visited::Twice(_)) if current > 500000 => break,
                _ => (),
            }
        }
    }

    visited
        .values()
        .map(|x| match x {
            Visited::Twice((a, b)) => Cycle {
                start: *a as isize,
                period: (b - a) as isize,
            },
            x => panic!("unexpected {:?}", x),
        })
        .collect()
}
fn part2(data: Data) -> isize {
    let keys: Vec<_> = data.nodes.keys().copied().filter(start_node).collect();
    keys.into_iter()
        .map(move |start| count_period(start, data.to_owned()))
        .multi_cartesian_product()
        .filter_map(|sequence| {
            // sequence.into_iter()try_reduce(merge_cycles).map(|x| x.start)
            let mut i = sequence.into_iter();
            let first = i.next().unwrap();
            i.try_fold(first, merge_cycles).map(|x| x.start)
        })
        .min()
        .unwrap()
        + 1
}

fn parse(f: &str) -> Data {
    let file_content = read_to_string(f).unwrap();
    let (dirs, nodes) = file_content.split("\n\n").collect_tuple().unwrap();

    let directions = dirs.chars().map(parse_dir).collect();
    let nodes = nodes.lines().map(parse_line).collect();
    Data { nodes, directions }
}

fn part1(data: Data) -> usize {
    let start = encode_node("AAA");
    let end = encode_node("ZZZ");
    count_steps(data, start, end)
}

fn main() {
    println!("part 1: {:?}", part1(parse("inputs/8b")));
    println!("part 2: {:?}", part2(parse("inputs/8b")));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parse_line_test() {
        let a = (encode_node("AAA"), (encode_node("BBB"), encode_node("CCC")));
        let b = parse_line("AAA = (BBB, CCC)");
        assert_eq!(a, b);
        assert_ne!(a, parse_line("AAB = (BBB, CCC)"));
        assert_ne!(a, parse_line("ABA = (BBB, CCC)"));
        assert_ne!(a, parse_line("BAA = (BBB, CCC)"));
    }
    #[test]
    fn merging_test() {
        let a = Cycle { start: 0, period: 6 };
        let b = Cycle { start: 0, period: 4 };
        assert_eq!(Some(Cycle { start: 0, period: 12 }), merge_cycles(a, b));
        let a = Cycle { start: 5, period: 6 };
        let b = Cycle { start: 3, period: 4 };
        assert_eq!(Some(Cycle { start: 11, period: 12 }), merge_cycles(a, b));

        let a = Cycle { start: 12, period: 13 };
        let b = Cycle { start: 16, period: 17 };
        assert_eq!(Some(Cycle { start: 220, period: 221 }), merge_cycles(a, b));
    }
    #[test]
    fn part1_test() {
        assert_eq!(6, part1(parse("inputs/8a")));
        assert_eq!(18827, part1(parse("inputs/8b")));
    }
    #[test]
    fn part2_test() {
        assert_eq!(6, part2(parse("inputs/8c")));
        assert_eq!(20220305520997, part2(parse("inputs/8b")));
    }
}
