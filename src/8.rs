use std::{collections::HashMap, fs::read_to_string};

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

fn walking<'a>(
    start: Node,
    nodes: HashMap<Node, (Node, Node)>,
    directions: Vec<Dir>,
) -> impl Iterator<Item = Node> + 'a {
    directions.into_iter().cycle().scan(start, move |x, dir| {
        *x = follow(dir, *nodes.get(x)?);
        Some(*x)
    })
}

fn count_steps(
    (directions, nodes): (Vec<Dir>, HashMap<Node, (Node, Node)>),
    start: Node,
    end: Node,
) -> usize {
    walking(start, nodes, directions)
        .find_position(|&x| x == end)
        .unwrap()
        .0
        + 1
}

fn ending_node(n: Node) -> bool {
    n.2 == 'Z'
}
fn start_node(n: &Node) -> bool {
    n.2 == 'A'
}

fn count_period(
    start: Node,
    nodes: HashMap<Node, (Node, Node)>,
    directions: Vec<Dir>,
) -> (usize, usize) {
    // returns the start, and period of the first found cycle
    let mut visited: HashMap<(usize, Node), usize> = HashMap::new();
    // node -> absolutei
    //
    let len = directions.len();
    for (current, node) in walking(start, nodes, directions).enumerate() {
        let local = current % len;
        if ending_node(node) {
            // every time we roll over the directions,
            // check if we visited that node before
            match visited.get(&(local, node)) {
                None => {
                    visited.insert((local, node), current);
                }
                Some(previous) => return (*previous, current - previous),
            }
        }
    }
    (0, 0)
}

fn part2((directions, nodes): (Vec<Dir>, HashMap<Node, (Node, Node)>)) -> i64 {
    let keys: Vec<_> = nodes.keys().copied().filter(start_node).collect();
    let _x = keys
        .into_iter()
        .map(move |start| count_period(start, nodes.to_owned(), directions.to_owned()))
        .inspect(|x| println!("{:?}", x))
        .collect_vec();
    return 3;
}

fn parse(f: &str) -> (Vec<Dir>, HashMap<Node, (Node, Node)>) {
    let file_content = read_to_string(f).unwrap();
    let (dirs, nodes) = file_content.split("\n\n").collect_tuple().unwrap();
    (
        dirs.chars().map(parse_dir).collect(),
        nodes.lines().map(parse_line).collect(),
    )
}

fn main() {
    let start = encode_node("AAA");
    let end = encode_node("ZZZ");
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
}
