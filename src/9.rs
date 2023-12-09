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
    x.into_iter()
        .tuple_windows()
        .map(|(a0, a1)| a1 - a0)
        .collect()
}

fn make_diffs(x: Vec<i64>) -> impl Iterator<Item = Vec<i64>> {
    iter::successors(Some(x), move |x| {
        let a = diff(x.clone());
        match x.iter().all(|&x| x == 0) {
            false => Some(a),
            true => None,
        }
    })
}

fn forwards(x: Vec<i64>) -> i64 {
    make_diffs(x).map(|x| *x.last().unwrap_or(&0)).sum() // this is what it comes down to
}

fn backwards(x: Vec<i64>) -> i64 {
    make_diffs(x)
        .map(|x| *x.first().unwrap_or(&0))
        .collect_vec() // i love
        .into_iter()   //  rust
        .rev()         // so much
        .reduce(|dx, x| x - dx)
        .unwrap()
}

fn solve(f: &str, direction: fn(Vec<i64>) -> i64) -> i64 {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(parse_line)
        .map(direction)
        .sum()
}

fn main() {
    println!("part 1: {:?}", solve("inputs/9b", forwards));
    println!("part 1: {:?}", solve("inputs/9a", backwards));
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
    fn forward_test() {
        assert_eq!(1, forwards(vec![1]));
        assert_eq!(3, forwards(vec![1, 2]));
        assert_eq!(35, forwards(vec![5, 10, 20]));
    }

    #[test]
    fn backwards_test() {
        assert_eq!(1, backwards(vec![1]));
        // 1 1 1
        //  0 0
        assert_eq!(0, backwards(vec![1, 2]));
        assert_eq!(5, backwards(vec![5, 10, 20]));
        //0 \5 10/ 15
        // 5 \5 / 5
        //  0 \/ 0
    }
    #[test]
    fn tpart1() {
        assert_eq!(114, solve("inputs/9a", forwards));
        assert_eq!(1684566095, solve("inputs/9b", forwards));
    }

    #[test]
    fn tpart2() {
        assert_eq!(2, solve("inputs/9a", backwards));
        assert_eq!(1136, solve("inputs/9b", backwards));
    }
}
