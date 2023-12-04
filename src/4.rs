use std::{convert::TryInto, fs::read_to_string};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
struct Line {
    id: i32,
    left: Vec<i32>,
    right: Vec<i32>
}

fn parse_subline(l: &str) -> Vec<i32>{
    // can't use filter_map, sad
    l.split(' ').filter(|x| x.len() != 0).map(|x| x.parse::<i32>().unwrap()).collect()
}

fn parse_line(l: &str) -> Line {
    let (head, rest) = l.split(':').collect_tuple().unwrap();
    let (left, right) = rest.split('|').map(parse_subline).collect_tuple().unwrap();
    let id = head.split(' ').rev().next().unwrap().parse::<i32>().unwrap();
    Line { id, left, right }
}

fn solve(f: &str) -> i32 {
    3
}

fn main() {
    println!("part 1: {}", solve("inputs/3b"));
    println!("part 2: {}", solve("inputs/3b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parsing() {
        let expected = Line {id: 1, left: vec![41,48,83,86,17], right: vec![83,86,6,31,17,9,48,53]};
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(expected, parse_line(input));
    }
}
