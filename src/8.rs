use std::{collections::HashMap, fs::read_to_string};

use num_integer::Integer;
use itertools::Itertools;
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
    data.directions
        .into_iter()
        .cycle()
        .scan(start, move |x, dir| {
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
    start: usize,
    period: usize,
}

fn merge_cycles(x: Cycle, y: Cycle) -> Option<Cycle> {
    let period = x.period.lcm(&y.period);
    let start = 0; // wrong
    Some(Cycle { period, start })
}

fn count_period(start: Node, data: Data) -> Cycle {
    // returns the start, and period of the first found cycle
    let mut visited: HashMap<(usize, Node), usize> = HashMap::new();
    // (ic, node) -> absolutei
    let len = data.directions.len();
    for (current, node) in walking(start, data).enumerate() {
        let local = current % len;
        if ending_node(node) {
            // every time we roll over the directions,
            // check if we visited that node before
            match visited.get(&(local, node)) {
                None => {
                    visited.insert((local, node), current);
                }
                Some(previous) => {
                    return Cycle {
                        start: *previous,
                        period: current - previous,
                    }
                }
            }
        }
    }
    // loop could get over more than one node, TODO -- fix it to output vector of periods
    panic!()
}

fn part2(data: Data) -> i64 {
    let keys: Vec<_> = data.nodes.keys().copied().filter(start_node).collect();
    let _x = keys
        .into_iter()
        .map(move |start| count_period(start, data.to_owned()))
        .inspect(|x| println!("{:?}", x))
        .collect_vec();
    return 3;
}

fn parse(f: &str) -> Data {
    let file_content = read_to_string(f).unwrap();
    let (dirs, nodes) = file_content.split("\n\n").collect_tuple().unwrap();

    let directions = dirs.chars().map(parse_dir).collect();
    let nodes = nodes.lines().map(parse_line).collect();
    Data { nodes, directions }
}

fn main() {
    //let start = encode_node("AAA");
    //let end = encode_node("ZZZ");
    //println!("part 1: {:?}", count_steps(parse("inputs/8b"), start, end));
    //println!("part 1: {:?}", count_steps(parse("inputs/8b"), start, end));
    println!("part 2: {:?}", part2(parse("inputs/8c")));
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
        let a = Cycle {start:0, period:6};
        let b = Cycle {start:0, period:4};
        assert_eq!(Some(Cycle{start: 0, period: 12}) ,merge_cycles(a, b));
        //        let a = Cycle {start:1, period:6};
        //        let b = Cycle {start:1, period:4};
        //assert_eq!(Some(Cycle{start: 1, period: 12}) ,merge_cycles(a, b));
    }
}
