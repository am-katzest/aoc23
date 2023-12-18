use itertools::Itertools;
use std::fs::read_to_string;
use substring::Substring;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Axis {
    Horizontal,
    Vertical,
}

fn axis(d: Dir) -> Axis {
    match d {
        Dir::Up | Dir::Down => Axis::Vertical,
        Dir::Left | Dir::Right => Axis::Horizontal,
    }
}

fn on(a: Axis, c: Coord) -> isize {
    match a {
        Axis::Vertical => c.1,
        Axis::Horizontal => c.0,
    }
}

fn off(a: Axis, c: Coord) -> isize {
    match a {
        Axis::Vertical => c.0,
        Axis::Horizontal => c.1,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

type Coord = (isize, isize);

fn step(d: Dir, (x, y): Coord, n: isize) -> Coord {
    match d {
        Dir::Left => (x - n, y),
        Dir::Right => (x + n, y),
        Dir::Down => (x, y + n),
        Dir::Up => (x, y - n),
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Undug,
    Trench,
    Filling,
}

#[derive(Clone, Debug, PartialEq)]
struct Instruction {
    dir: Dir,
    length: isize,
}

fn parse_line(l: &str) -> Instruction {
    let (d, l, _c) = l.split_whitespace().collect_tuple().unwrap();
    let dir = match d {
        "R" => Dir::Right,
        "L" => Dir::Left,
        "D" => Dir::Down,
        "U" => Dir::Up,
        _ => panic!("wrong dir, {d}"),
    };
    let length = l.parse::<isize>().unwrap();
    Instruction { dir, length }
}

fn parse_line2(l: &str) -> Instruction {
    let (_, _, c) = l.split_whitespace().collect_tuple().unwrap();
    let l = c.substring(2, 7);
    let dir = match c.chars().nth(7).unwrap() {
        '0' => Dir::Right,
        '2' => Dir::Left,
        '1' => Dir::Down,
        '3' => Dir::Up,
        _ => panic!(";-;"),
    };
    let length = isize::from_str_radix(l, 16).unwrap();
    Instruction { dir, length }
}

fn parse(f: &str, parse_line: fn(&str) -> Instruction) -> Vec<Instruction> {
    read_to_string(f).unwrap().lines().map(parse_line).collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Segment {
    axis: Axis,
    start: isize,
    end: isize,
    altitude: isize,
}

fn solve(instr: Vec<Instruction>) -> isize {
    // a bit redundant, but i like it :3
    let segments = segmentize(instr);
    // we are scanning left to right
    let changes = breakpoints(&segments, Axis::Vertical);
    let bars = bars(&segments, Axis::Horizontal);
    println!("{:?}", bars);
    3
}

fn bars(segments: &Vec<Segment>, a:Axis) -> Vec<Segment> {
    let bars = segments
        .iter()
        .filter_map(|&s| match s.axis==a {
            false => None,
            true => Some(s),
        })
        .collect_vec();
    bars
}

fn breakpoints(segments: &Vec<Segment>, a: Axis) -> Vec<isize> {
    segments
        .iter()
        .filter_map(|&s| match s.axis == a {
            false => None,
            true => Some(s.altitude),
        })
        .sorted()
        .unique()
        .collect_vec()
}

fn segmentize(instr: Vec<Instruction>) -> Vec<Segment> {
    instr
        .into_iter()
        .scan((0, 0), |pos0, i| {
            let pos1 = step(i.dir, *pos0, i.length);
            let axis = axis(i.dir);
            let start = on(axis, pos1).min(on(axis, *pos0));
            let end = on(axis, pos1).max(on(axis, *pos0));
            let altitude = off(axis, *pos0);
            let s = Segment { axis, start, end, altitude };
            *pos0 = pos1;
            Some(s)
        })
        .collect_vec()
}

fn main() {
    println!("part 1: {:?}", solve(parse("inputs/18a", parse_line)));
    //    println!("part 2: {:?}", solve("inputs/18b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parsing_test() {
        assert_eq!(Instruction { dir: Dir::Right, length: 6 }, parse_line("R 6 (#70c710)"));
        assert_eq!(
            Instruction {
                dir: Dir::Right,
                length: 461937
            },
            parse_line2("R 6 (#70c710)")
        );
    }
}
