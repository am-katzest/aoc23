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
    // maps what it can, returns up to three MappingResults
    let rs = r.start;
    let re = r.start + r.len;
    let ms = m.src;
    let me = m.src + m.len;
    let transform: i64 = m.dest - m.src;
    // could possibly be done by always creating three ranges, then filtering i think
    if rs >= me || ms >= re {
        // range entirely outside map
        return vec![Res::Unmapped(r)];
    } else if re <= me && rs >= ms {
        // range entirely fits in map
        return vec![Res::Mapped(Range {
            start: rs + transform,
            len: r.len,
        })];
    } else if rs < ms && re > me {
        // range exceeds mapping on both ends
        let left = Range {start: rs, len: ms - rs};
        let middle = Range {start: m.dest, len: m.len};
        let right = Range {start: me, len: r.len - (ms-rs) - m.len };
        return vec![Res::Unmapped(left),
                    Res::Mapped(middle),
                    Res::Unmapped(right)];
    } else if re <= me {
        // range exceeds mapping on the left
        let left = Range {start: rs, len: ms - rs};
        let middle = Range {start: m.dest, len: r.len - (ms - rs)};
        return vec![Res::Unmapped(left),
                    Res::Mapped(middle)];
    } else {
        // range exceeds mapping on the right
        let middle = Range {start: m.dest + rs - ms, len: me - rs};
        let right = Range {start: me, len: r.len - (me-rs)};
        return vec![Res::Mapped(middle),
                    Res::Unmapped(right)];
    }
}

fn translate_ranges(rs:Vec<Range>, ms:&Vec<MapLine>) -> Vec<Range> {
    let is = rs.iter().copied().map(|x| Res::Unmapped(x)).collect_vec();
    fn reducer(i:Vec<Res>, m:MapLine) -> Vec<Res> {
        i.iter().copied().map(|x| match x {
            Res::Unmapped(x) => try_translate_range(x, m),
            x => vec![x]} // don't touch already mapped values
        ).concat()
    }
    ms.iter()
      .copied()
      .fold(is, reducer)
      .iter()
      .copied()
      .map(|x| match x {
        Res::Unmapped(x)  => x,
        Res::Mapped(x)  => x
    }).collect_vec()
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

fn advance_ranges(mappings: Vec<Vec<MapLine>>, seed_ranges: Vec<Range>) -> i64 {
    mappings.iter().fold(seed_ranges, translate_ranges).iter().map(|x| x.start).min().unwrap()
}
fn solve(f: &str) -> i64 {
    let (s, mappings) = parse(f);
    let seed_ranges = s.iter().copied().map(|start| Range {start, len: 1}).collect_vec();
    advance_ranges(mappings, seed_ranges)
}
fn solve2(f: & str) -> i64 {
    let (s, mappings) = parse(f);
    let seed_ranges = s.iter().copied().tuples().map(|(start, len)| Range {start, len}).collect_vec();
    advance_ranges(mappings, seed_ranges)
}


fn main() {
    println!("part 1: {}", solve("inputs/5b"));
    println!("part 2: {}", solve2("inputs/5b"));
}

//part 2: 75221860 // too low ;-;
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn try_range_apply_test() {
        let ml = MapLine {
            dest: 200,
            src: 100,
            len: 10,
        };
        // wholly inside
        assert_eq!(
            vec![Res::Mapped(Range { start: 201, len: 2 })],
            try_translate_range(Range { start: 101, len: 2 }, ml)
        );
        // touching
        assert_eq!(
            vec![Res::Mapped(Range { start: 200, len: 10 })],
            try_translate_range(Range { start: 100, len: 10 }, ml)
        );
        // wholly outside
        assert_eq!(
            vec![Res::Unmapped(Range { start: 145, len: 5 })],
            try_translate_range(Range { start: 145, len: 5 }, ml)
        );
        assert_eq!(
            vec![Res::Unmapped(Range { start: 5, len: 5 })],
            try_translate_range(Range { start: 5, len: 5 }, ml)
        );
        // touching
        assert_eq!(
            vec![Res::Unmapped(Range { start: 95, len: 5 })],
            try_translate_range(Range { start: 95, len: 5 }, ml)
        );
        assert_eq!(
            vec![Res::Unmapped(Range { start: 110, len: 5 })],
            try_translate_range(Range { start: 110, len: 5 }, ml)
        );
        // exceeding
        assert_eq!(
            vec![Res::Unmapped(Range { start: 85, len: 15 }),
                 Res::Mapped(Range { start: 200, len: 10 }),
                 Res::Unmapped(Range { start: 110, len: 15 })],
            try_translate_range(Range { start: 85, len: 40 }, ml)
        );
        // exceeding left
        assert_eq!(
            vec![Res::Unmapped(Range { start: 90, len: 10 }),
                 Res::Mapped(Range { start: 200, len: 5 }),],
            try_translate_range(Range { start: 90, len: 15 }, ml)
        );
        // touching
        assert_eq!(
            vec![Res::Unmapped(Range { start: 90, len: 10 }),
                 Res::Mapped(Range { start: 200, len: 10 }),],
            try_translate_range(Range { start: 90, len: 20 }, ml)
        );
        // exceeding right
        assert_eq!(
            vec![Res::Mapped(Range { start: 205, len: 5 }),
                 Res::Unmapped(Range { start: 110, len: 30 })],
            try_translate_range(Range { start: 105, len: 35 }, ml)
        );
        // touching
        assert_eq!(
            vec![Res::Mapped(Range { start: 200, len: 10 }),
                 Res::Unmapped(Range { start: 110, len: 30 })],
            try_translate_range(Range { start: 100, len: 40 }, ml)
        );
    }
#[test]
    fn part1() {
        assert_eq!(35, solve("inputs/5a"));
    }
#[test]
    fn part2() {
        assert_eq!(46, solve2("inputs/5a"));
    }
}
