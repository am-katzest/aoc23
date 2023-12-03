use std::{convert::TryInto, fs::read_to_string};

use itertools::Itertools;

fn is_symbol(t: char) -> bool {
    t != '.' && !t.is_ascii_digit()
}

fn solve(f: &str, part2: bool) -> i32 {
    let lines: Vec<Vec<char>> = read_to_string(f)
        .unwrap()
        .lines()
        .map(|x| x.chars().collect_vec())
        .collect_vec();
    let sizey: i32 = lines.len().try_into().unwrap();
    let sizex: i32 = lines.first().unwrap().len().try_into().unwrap();
    let mut acc = 0;
    let mut current: i32 = 0;
    let mut current_counts = false;
    let mut reading = false;
    let get = |x: i32, y: i32| -> char {
        if x < 0 || y < 0 {
            return '.';
        }
        match lines.get(usize::try_from(y).unwrap()) {
            None => '.',
            Some(line) => match line.get(usize::try_from(x).unwrap()) {
                None => '.',
                Some(&c) => c,
            },
        }
    };

    let add_digit = |current: i32, dgt: char| -> i32 {
        let n = dgt.to_digit(10).unwrap();
        let a = current * 10 + n as i32;
        a
    };

    let adjectant_to_symbol = |x: i32, y: i32| {
        for xi in x - 1..=x + 1 {
            for yi in y - 1..=y + 1 {
                if !(xi == x && yi == x) {
                    if is_symbol(get(xi, yi)) {
                        return true;
                    }
                }
            }
        }
        false
    };
    let seek_to_start = |(x, y)| {
        if !get(x, y).is_digit(10) {
            return None;
        };
        let mut xi = x;
        while get(xi - 1, y).is_digit(10) {
            xi -= 1;
        }
        Some((xi, y))
    };

    let read_from_start = |(x, y)| {
        let mut xi = x;
        let mut acc = 0;
        while get(xi, y).is_digit(10) {
            acc = add_digit(acc, get(xi, y));
            xi += 1;
        } // no recursion :despair:
        acc
    };

    let calc_gear_ratio = |x: i32, y: i32| -> i32 {
        let mut indices: Vec<(i32, i32)> = Vec::new();

        for xi in x - 1..=x + 1 {
            for yi in y - 1..=y + 1 {
                if !(xi == x && yi == x) {
                    indices.push((xi, yi)); // not too pretty
                }
            }
        }
        let gear_thingies = indices
            .iter()
            .copied()
            .filter_map(seek_to_start)
            .unique()
            .map(read_from_start)
            .collect_tuple();
        match gear_thingies {
            Some((a, b)) => a * b,
            None => 0,
        }
    };

    for y in 0..sizey {
        for x in 0..=sizex {
            let c = get(x, y);
            if part2 {
                if c == '*' {
                    acc += calc_gear_ratio(x, y);
                }
            } else {
                // getting over the size of array to ensure that numbers are broken
                if c.is_digit(10) {
                    reading = true;
                    current = add_digit(current, c);
                    if !current_counts {
                        if adjectant_to_symbol(x, y) {
                            current_counts = true;
                        }
                    }
                // finish reading the number
                } else {
                    if reading {
                        if current_counts {
                            acc += current;
                        }
                        current_counts = false;
                        current = 0;
                        reading = false;
                    }
                }
            }
        }
    }
    acc
}

fn main() {
    println!("part 1: {}", solve("inputs/3b", false));
    println!("part 2: {}", solve("inputs/3b", true));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1() {
        assert_eq!(4361, solve("inputs/3a", false))
    }
    #[test]
    fn part2() {
        assert_eq!(467835, solve("inputs/3a", true))
    }
}
