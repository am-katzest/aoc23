use std::{fs::read_to_string};

use itertools::Itertools;

fn parse_line_of_numbers(l: &str) -> Vec<u32> {
    l.split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect()
}

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

fn apply_mappings(i:u32, ms: Vec<MapLine>) -> u32 {
    for m in ms {
        match try_apply_mapping(i, m) {
            Some(o) => return o,
            _ => (),
        }
    }
    i
}

fn parse_mapping(f: &str) -> MapLine {
    let (dest, src, len) = parse_line_of_numbers(f).iter().copied().collect_tuple().unwrap();
    MapLine {dest, src, len}
}

fn parse_section(f:&str) -> Vec<MapLine> {
   f.split("\n").skip(1).filter(|x| x.len() > 0).map(parse_mapping).collect()
}

fn parse(f: &str) -> (Vec<u32>, Vec<Vec<MapLine>>){
    let s = read_to_string(f).unwrap();
    let mut i = s.split("\n\n");
    let seeds = parse_line_of_numbers(i.next().unwrap());
    let mappings = i.map(parse_section).collect_vec();
    mappings.iter().for_each(|x| println!("{:?}", x));

    (seeds, mappings)
}

fn solve(f: &str) -> usize {
    parse(f);
    3
}

fn main() {
    println!("part 1: {}", solve("inputs/5a"));
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
