use std::fs::read_to_string;

use itertools::Itertools;

fn parse_line_of_numbers(l: &str) -> Vec<i64> {
    l.split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MapLine {
    dest: i64,
    src: i64,
    len: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Range {
    start: i64,
    len: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Res {
    Mapped(Range),
    Unmapped(Range),
}

fn try_translate_range(r: Range, m: MapLine) -> Vec<Res> {
    let rs = r.start;
    let re = r.start + r.len;
    let ms = m.src;
    let me = m.src + m.len;
    let transform: i64 = m.dest - m.src;
    if rs > me || ms > re {
        // range entirely outside map
        return vec![Res::Unmapped(r)];
    } else if re < me && rs > ms {
        // range entirely fits in map
        return vec![Res::Mapped(Range {
            start: rs + transform,
            len: r.len,
        })];
    }
    // maps what it can, returns up to three MappingResults
    vec![]
}

fn try_translate(i: i64, m: MapLine) -> Option<i64> {
    if m.src <= i && i < m.src + m.len {
        Some(i + m.dest - m.src)
    } else {
        None
    }
}

fn translate(i: i64, ms: &Vec<MapLine>) -> i64 {
    ms.iter().find_map(|m| try_translate(i, *m)).unwrap_or(i)
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

fn parse(f: &str) -> (Vec<i64>, Vec<Vec<MapLine>>) {
    let s = read_to_string(f).unwrap();
    let mut i = s.split("\n\n");
    let seeds = parse_line_of_numbers(i.next().unwrap());
    let mappings = i.map(parse_section).collect_vec();

    (seeds, mappings)
}

fn solve(f: &str) -> i64 {
    let (seeds, mappings) = parse(f);
    seeds
        .iter()
        .copied()
        .map(|seed| mappings.iter().fold(seed, |s, maps| translate(s, maps)))
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

        assert_eq!(None, try_translate(3, ml));
        assert_eq!(None, try_translate(49, ml));
        assert_eq!(Some(40), try_translate(50, ml));
        assert_eq!(Some(44), try_translate(54, ml));
        assert_eq!(None, try_translate(55, ml));
    }
    #[test]
    fn try_range_apply_test() {
        let ml = MapLine {
            dest: 40,
            src: 50,
            len: 5,
        };
        let fully_inside = Range { start: 41, len: 2 };
        let mrfi = vec![Res::Mapped(Range { start: 51, len: 2 })];
        assert_eq!(mrfi, try_translate_range(fully_inside, ml));
    }
    #[test]
    fn part1() {
        assert_eq!(35, solve("inputs/5a"));
    }
}
