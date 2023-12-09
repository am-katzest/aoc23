use std::fs::read_to_string;

use itertools::Itertools;
use std::iter;
fn parse_line(l: &str) -> Vec<i64> {
    l.split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

fn diff(x: Vec<i64>) -> Vec<i64> {
    // woo, we're doing diffrentiation
    x.into_iter().tuple_windows().map(|(a0, a1)| a1 - a0).collect()
}

fn make_diffs(x: Vec<i64>) -> impl Iterator<Item = Vec<i64>> {
    iter::successors(Some(x), move |x| {
        let a = diff(x.clone());
        match x.iter().all(|&x| x == 0) {
            false => Some(a),
            true => None
        }
    })
}

fn forwards(x: Vec<i64>) -> i64 {
    make_diffs(x).map(|x| *x.last().unwrap_or(&0)).sum() // this is what it comes down to
}

fn backwards(x: Vec<i64>) -> i64 {
    make_diffs(x).map(|x| *x.first().unwrap_or(&0)).reduce(|dx, x| x - dx).unwrap() // this is what it comes down to
}

fn part1(f: & str) -> i64 {
    read_to_string(f).unwrap().lines().map(parse_line).map(forwards).sum()
}

fn part2(f: & str) -> i64 {
    read_to_string(f).unwrap().lines().map(parse_line).map(backwards).sum()
}

fn main() {
    println!("part 1: {:?}", part1("inputs/9b"));
    println!("part 1: {:?}", part2("inputs/9a"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn diff_test() {
        assert_eq!(vec![0, 2, 4, 6, 8], diff(vec![3, 3, 5, 9, 15, 23]));
        assert_eq!(vec![2], diff(vec![0, 2]));
    }
    #[test]
    fn tpart1() {
        assert_eq!(114, part1("inputs/9a"));
        assert_eq!(1684566095, part1("inputs/9b"));
    }
    #[test]
    fn tpart2() {
        assert_eq!(2, part2("inputs/9a"));
        assert_eq!(-2, part2("inputs/9b"));
    }
}
