use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
struct Row {
    springs: Vec<Spring>,
    ecc: Vec<usize>,
}

fn parse_spring(s: char) -> Spring {
    match s {
        '?' => Spring::Unknown,
        '.' => Spring::Operational,
        '#' => Spring::Damaged,
        _ => panic!("wrong spring {s}"),
    }
}

fn parse_line(line: &str) -> Row {
    let (l, r) = line.split(' ').collect_tuple().unwrap();
    let springs = l.chars().map(parse_spring).collect_vec();
    let ecc = r.split(',').map(|x| x.parse().unwrap()).collect_vec();
    Row { springs, ecc }
}
fn possibilities(r: Row) -> usize {
    3
}

fn part1(f: &str) -> usize {
    read_to_string(f).unwrap().lines().map(parse_line).map(possibilities).sum()
}

fn main() {
    println!("part 1: {}", part1("inputs/1b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parser_test() {
        assert_eq!(
            Row {
                springs: vec![Spring::Operational, Spring::Operational, Spring::Unknown, Spring::Damaged],
                ecc: vec![2, 1]
            },
            parse_line("..?# 2,1")
        );
    }
    #[test]
    fn dumb_solver_test() {
        let r = parse_line(". 1");
        assert_eq!(1, possibilities(r));
    }
}
