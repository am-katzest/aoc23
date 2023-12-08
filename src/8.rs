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

//type Node<'a> = &'a str;
// should fit in u16, cuz log2(26**3)~=14
type Node = u32;

fn encode_node(x: &str) -> Node {
    x.as_bytes().into_iter().for_each(|x| println!("{x}"));
    x.as_bytes()
        .into_iter()
        .fold(0, |acc, &x| (acc << 8) | (x as u32))
}

fn parse_line(l: &str) -> (Node, (Node, Node)) {
    let get = |o| encode_node(l.substring(o, o + 3));
    (get(0), (get(7), get(12)))
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
    println!("part 1: {:?}", parse("inputs/8a"));
    //println!("part 2: {:?}", solve(parse("inputs/7b", Card::Joker)));
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
