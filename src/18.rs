use itertools::Itertools;
use std::fs::read_to_string;
use substring::Substring;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Segment {
    altitude: isize,
    axis: Axis,
    start: isize,
    end: isize,
}
fn relevant(s: Segment, x: isize) -> bool {
    x >= s.start && x <= s.end
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    Inside,
    Outside,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Enter,
    Leave,
    Neither,
}

fn edge(s: Segment, x: isize) -> Side {
    if relevant(s, x) {
        if x == s.start {
            return Side::Left;
        }
        if x == s.end {
            return Side::Right;
        }
        return Side::Inside;
    }
    return Side::Outside;
}

fn height(ss: &Vec<Segment>, x: isize) -> isize {
    let intersections = ss.iter().copied().filter(|&s| relevant(s, x)).sorted().map(|s| (edge(s, x), s.altitude));
    let mut acc = 0;
    let mut state = Side::Outside;
    let mut entered = 0;
    for (edge, alt) in intersections {
        let a = match (state, edge) {
            (_, Side::Outside) | (Side::Left, Side::Inside) | (Side::Right, Side::Inside) => {
                panic!();
            }
            (Side::Outside, e) => {
                state = e; //TODO
                Action::Enter
            }
            // leaving side
            (Side::Left, Side::Left) | (Side::Right, Side::Right) => {
                state = Side::Outside;
                Action::Leave
            }
            // entering into main body (while being on side)
            (Side::Left, Side::Right) | (Side::Right, Side::Left) => {
                state = Side::Inside;
                Action::Neither
            }
            (Side::Inside, Side::Inside) => {
                // leaving
                state = Side::Outside;
                Action::Leave
            }
            (Side::Inside, Side::Left) => {
                // starting to leave
                state = Side::Right;
                Action::Neither
            }
            (Side::Inside, Side::Right) => {
                // starting to leave
                state = Side::Left;
                Action::Neither
            }
        };
        match a {
            Action::Enter => {
                entered = alt;
            }
            Action::Leave => {
                acc += alt - entered + 1;
            }
            Action::Neither => {}
        }
        //println!("{:?}, {} (am {:?})", edge, alt, state);
    }
    acc
}

fn solve(instr: Vec<Instruction>) -> isize {
    // a bit redundant, but i like it :3
    let segments = segmentize(instr);
    // we are scanning left to right
    let changes = breakpoints(&segments, Axis::Vertical);
    let bars = bars(&segments, Axis::Horizontal);
    let mut acc = 0;
    for (i,i1) in changes.iter().copied().tuple_windows() {
        acc += height(&bars, i); // at the edge
        acc += height(&bars, i+1) * (i1 - i - 1); // between
    }
    acc += height(&bars, *changes.last().unwrap());
    acc
}

fn bars(segments: &Vec<Segment>, a: Axis) -> Vec<Segment> {
    segments.iter().copied().filter(|x| x.axis == a).collect_vec()
}

fn breakpoints(segments: &Vec<Segment>, a: Axis) -> Vec<isize> {
    segments
        .iter()
        .filter(|x| x.axis == a)
        .map(|&s| s.altitude)
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
    println!("part 1: {:?}", solve(parse("inputs/18b", parse_line)));
    println!("part 2: {:?}", solve(parse("inputs/18b", parse_line2)));
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
