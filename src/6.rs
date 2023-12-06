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

fn find_bound(start: i64, time: i64, distance: i64, slope: i64) -> i64 {
    // can't pass lambda
    let f = |x: i64| -x * x + x * time - distance; // f
    let d = |x: i64| -2 * x + time; // f'
                                    // newthon method
    let mut prev = -1;
    let mut curr = start;
    while prev != curr {
        prev = curr;
        curr = curr - f(curr) / d(curr); // newton method, more stability
    }
    loop {
        if f(curr) > 0 {
            return curr;
        }
        curr += slope;
    }
}

fn get_range((time, distance): (i64, i64)) -> (i64, i64) {
    (
        find_bound(0, time, distance, 1),
        find_bound(time, time, distance, -1),
    )
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

        assert_eq!((14, 71516), get_range((71530, 940200)));

        assert_eq!((8, 37), get_range((45, 295)));
        assert_eq!((24, 74), get_range((98, 1734)));
        assert_eq!((21, 62), get_range((83, 1278)));
        assert_eq!((26, 47), get_range((73, 1210)));

        assert_eq!((7711543, 38276830), get_range((45988373, 295173412781210)));
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
