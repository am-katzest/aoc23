use std::{fs::read_to_string};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
struct Line {
    id: usize,
    left: Vec<usize>,
    right: Vec<usize>,
}

fn parse_subline(l: &str) -> Vec<usize> {
    l.split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

fn parse_line(l: &str) -> Line {
    let (head, rest) = l.split(':').collect_tuple().unwrap();
    let (left, right) = rest.split('|').map(parse_subline).collect_tuple().unwrap();
    let id = head
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    Line { id, left, right }
}

fn calc_line_matches(l: Line) -> usize {
    l.right.iter().filter(|n| l.left.contains(n)).count()
}

fn line_scores(f: &str) -> Vec<usize> {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(parse_line)
        .map(calc_line_matches)
        .collect()
}

fn calc_line_points(i: usize) -> usize {
    match i {
        0 => 0,
        n => 1 << (n - 1),
    }
}

fn solve(f: &str) -> usize {
    line_scores(f).iter().copied().map(calc_line_points).sum()
}

fn solve2(f: &str) -> usize {
    let scores = line_scores(f);
    let mut counts = scores.iter().map(|_| 1).collect_vec();
    for i in 0..counts.len() {
        let s = scores[i];
        let c = counts[i];

        for j in i + 1..(i + s + 1) {
            counts[j] += c;
        }
    }
    counts.iter().sum()
}

fn main() {
    println!("part 1: {}", solve("inputs/4b"));
    println!("part 2: {}", solve2("inputs/4b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parsing() {
        let expected = Line {
            id: 1,
            left: vec![41, 48, 83, 86, 17],
            right: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(expected, parse_line(input));
    }
    #[test]
    fn point_calculation() {
        let l1 = Line {
            id: 1,
            left: vec![41, 48, 83, 86, 17],
            right: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        let l2 = Line {
            id: 1,
            left: vec![41, 48, 83, 86, 17],
            right: vec![0],
        };
        assert_eq!(4, calc_line_matches(l1));
        assert_eq!(0, calc_line_matches(l2));
        assert_eq!(8, calc_line_points(4));
        assert_eq!(0, calc_line_points(0));
    }
    #[test]
    fn part1() {
        assert_eq!(13, solve("inputs/4a"));
    }
    #[test]
    fn part2() {
        assert_eq!(30, solve2("inputs/4a"));
    }
}
