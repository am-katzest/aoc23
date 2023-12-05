use std::{fs::read_to_string};

#[derive(Clone, Debug, PartialEq)]
struct MapLine {
    dest: u32,
    src: u32,
    len: u32,
}

fn try_apply_mapping(i: u32, m: MapLine) -> Option<u32> {
    if m.src <= i && i < m.src + m.len {
        Some(i + m.dest - m.src)
    } else {
        None
    }
}

fn parse_subline(l: &str) -> Vec<usize> {
    l.split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

fn parse(f: &str) {
    read_to_string(f)
        .unwrap()
        .lines()
        .for_each(|x| println!("{}", x))
}

fn solve(f: &str) -> usize {
    3
}

fn main() {
    println!("part 1: {}", solve("inputs/4b"));
    //println!("part 2: {}", solve2("inputs/4b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn try_apply_test() {
        assert_eq!(None, try_apply_mapping(3, MapLine {dest: 40, src: 50, len: 5}));
        assert_eq!(None, try_apply_mapping(49, MapLine {dest: 40, src: 50, len: 5}));
        assert_eq!(Some(40), try_apply_mapping(50, MapLine {dest: 40, src: 50, len: 5}));
        assert_eq!(Some(44), try_apply_mapping(54, MapLine {dest: 40, src: 50, len: 5}));
        assert_eq!(None, try_apply_mapping(55, MapLine {dest: 40, src: 50, len: 5}));
    }
}
