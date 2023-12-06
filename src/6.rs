use std::fs::read_to_string;

use itertools::Itertools;

fn parse_line_of_numbers(l: &str) -> impl Iterator<Item = i64> + '_ {
    l.split_whitespace().filter_map(|x| x.parse::<i64>().ok())
}
fn parse(f: &str) -> impl Iterator<Item = (i64, i64)> + '_ {
    let (a, b) = read_to_string(f)
        .unwrap()
        .lines()
        .map(|x| parse_line_of_numbers(x).collect_vec())
        .collect_tuple()
        .unwrap();
    std::iter::zip(a, b)
}
fn parse2(f: &str) -> (i64, i64) {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}
// (t, min_distance)
// distance = (t - x) * x
// distance = x*t - x^2
// min_distance < x*t - x^2
// 0 < -x^2 + xt - min_distance
// we can assume there always are solutions (:clueless:)
// a = 1
// b = t
// c = -min_distance
// Δ = b^2 - 4ac
// x1 =  (-b - √Δ) /2a
// x2 =  (-b + √Δ) /2a
fn roots(a: i64, b: i64, c: i64) -> (f64, f64) {
    let delta = b * b - 4 * a * c;
    let sqrt_delta = (delta as f64).sqrt();
    let twoa = (2 * a) as f64;
    let minus_b = -b as f64;
    ((minus_b + sqrt_delta) / twoa, (minus_b - sqrt_delta) / twoa)
}
fn get_range((time, distance): (i64, i64)) -> (i64, i64) {
    let (a, b) = roots(-1, time, -distance);
    println!("{time} {distance} -> {a} {b}");
    (a as i64 + 1, (b - 1e-9) as i64) // HACK, will probably backfire
}

fn solve(f: &str) -> i64 {
    parse(f)
        .map(get_range)
        .map(|(x, y)| y - x + 1)
        .fold(1, |x, y| x * y)
}
fn solve2(f: &str) -> i64 {
    let (x, y) = get_range(parse2(f));
    y - x + 1
}
fn main() {
    println!("part 1: {:?}", solve("inputs/6b"));
    println!("part 2: {:?}", solve2("inputs/6b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn range_test() {
        assert_eq!((2, 5), get_range((7, 9)));
        assert_eq!((4, 11), get_range((15, 40)));
        assert_eq!((11, 19), get_range((30, 200)));
    }
    #[test]
    fn part1() {
        assert_eq!(288, solve("inputs/6a"));
    }
    #[test]
    fn part2() {
        assert_eq!(71503, solve2("inputs/6a"));
    }
}
