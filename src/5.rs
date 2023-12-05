use std::{fs::read_to_string, ptr::null};

use itertools::Itertools;

fn parse_line_of_numbers(l: &str) -> Vec<u64> {
    l.split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MapLine {
    dest: u64,
    src: u64,
    len: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Range {
    start: u64,
    len: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Res {
    Mapped(Range),
    Unmapped(Range),
}

fn try_apply_mapping_range(i: Range, m: MapLine) -> Vec<Res> {
    // maps what it can, returns up to three MappingResults
    vec![]
}

fn try_apply_mapping(i: u64, m: MapLine) -> Option<u64> {
    if m.src <= i && i < m.src + m.len {
        Some(i + m.dest - m.src)
    } else {
        None
    }
}

fn apply_mappings(i: u64, ms: &Vec<MapLine>) -> u64 {
    ms.iter()
        .find_map(|m| try_apply_mapping(i, *m))
        .unwrap_or(i)
}

fn parse_mapping(f: &str) -> MapLine {
    let (dest, src, len) = parse_line_of_numbers(f)
        .iter()
        .copied()
        .collect_tuple()
        .unwrap();
    MapLine { dest, src, len }
}

fn parse_section(f: &str) -> Vec<MapLine> {
    f.split("\n")
        .skip(1)
        .filter(|x| x.len() > 0)
        .map(parse_mapping)
        .collect()
}

fn parse(f: &str) -> (Vec<u64>, Vec<Vec<MapLine>>) {
    let s = read_to_string(f).unwrap();
    let mut i = s.split("\n\n");
    let seeds = parse_line_of_numbers(i.next().unwrap());
    let mappings = i.map(parse_section).collect_vec();

    (seeds, mappings)
}

fn solve(f: &str) -> u64 {
    let (seeds, mappings) = parse(f);
    seeds
        .iter()
        .copied()
        .map(|seed| {
            mappings
                .iter()
                .fold(seed, |s, maps| apply_mappings(s, maps))
        })
        .min()
        .unwrap()
}

fn main() {
    println!("part 1: {}", solve("inputs/5b"));
    //println!("part 2: {}", solve2("inputs/4b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn try_apply_test() {
        let ml = MapLine {
            dest: 40,
            src: 50,
            len: 5,
        };

        assert_eq!(None, try_apply_mapping(3, ml));
        assert_eq!(None, try_apply_mapping(49, ml));
        assert_eq!(Some(40), try_apply_mapping(50, ml));
        assert_eq!(Some(44), try_apply_mapping(54, ml));
        assert_eq!(None, try_apply_mapping(55, ml));
    }
    #[test]
    fn try_range_apply_test() {
        let ml = MapLine {
            dest: 40,
            src: 50,
            len: 5,
        };
        let fully_inside = Range {start: 41, len: 2};
        let mrfi = vec![Res::Mapped(Range {start: 51, len:2})];
        assert_eq!(mrfi, try_apply_mapping_range(fully_inside, ml));
    }
    #[test]
    fn part1() {
        assert_eq!(35, solve("inputs/5a"));
    }
}
