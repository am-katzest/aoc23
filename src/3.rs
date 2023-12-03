use std::{fs::read_to_string, iter};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Thing{
    Number(i32),
    Symbol(char),
    Padding
}

fn parse_thing(l:&str) -> Thing {
    match l.parse::<i32>(){
        Ok(r) => Thing::Number(r),
        Err(_) => Thing::Symbol(l.chars().next().unwrap()),
    }
}
fn parse_line(l:&str) -> Vec<Thing> {
    l.split('.').filter(|x| x.len() != 0).map(parse_thing).collect()
}

fn is_symbol(t:Thing) -> bool {
    match t {
        Thing::Symbol(_) => true,
        _ => false,
    }
}

fn sum_only_neighbouring(x:Vec<Thing>) -> i32 {
    iter::once(&Thing::Padding)
        .chain(x.iter())
        .chain(iter::once(&Thing::Padding)) // we prepend and append dummy "Padding" Things to the iterator
        .copied()
        .tuple_windows()
        .filter_map(|(l, x, r)| match x{
            Thing::Number(x) => if is_symbol(l) || is_symbol(r) {Some(x)}else{None}
            _ => None,
        })
        .sum()
}

fn solve(f:&str) -> i32 {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(parse_line)
        .map(sum_only_neighbouring)
        .sum()
}

fn main() {
    println!("part 1: {}", solve("inputs/3a"));
//    println!("part 2: {}", solve2("inputs/2b"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_test() {
        assert_eq!(Thing::Number(617),parse_thing("617"));
        assert_eq!(Thing::Symbol('#'),parse_thing("#"));
    }
    #[test]
    fn line_parsing_test() {
                assert_eq!(vec![Thing::Number(617),Thing::Symbol('*')],parse_line("617*......"));
    }
}
