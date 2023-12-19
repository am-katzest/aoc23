use itertools::Itertools;
use std::fs::read_to_string;
use substring::Substring;

fn horizontal(d: Dir) -> bool {
    match d {
        Dir::Up | Dir::Down => false,
        Dir::Left | Dir::Right => true,
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
    start: isize,
    end: isize,
}

fn relevant(s: Segment, x: isize) -> bool {
    x >= s.start && x <= s.end
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    Left,
    Right,
}
fn other(s: Side) -> Side {
    match s {
        Side::Left => Side::Right,
        Side::Right => Side::Left,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Inside,
    Outside,
    Side(Side),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Enter,
    Leave,
    Neither,
}

fn edge(s: Segment, x: isize) -> State {
    if relevant(s, x) {
        if x == s.start {
            return State::Side(Side::Left);
        }
        if x == s.end {
            return State::Side(Side::Right);
        }
        return State::Inside;
    }
    return State::Outside;
}

// sums up height in one vertical line
fn height(segments: &Vec<Segment>, x: isize) -> isize {
    let mut acc = 0;
    let mut state = State::Outside;
    let mut alt_entered = 0;
    for &segment in segments {
        let a = match (state, edge(segment, x)) {
            (_, State::Outside) => Action::Neither,
            (State::Outside, e) => {
                state = e;
                Action::Enter
            }
            (State::Side(x), State::Side(y)) if x == y => {
                state = State::Outside;
                Action::Leave
            }
            (State::Side(_), State::Side(_)) => {
                state = State::Inside;
                Action::Neither
            }
            (State::Inside, State::Inside) => {
                state = State::Outside;
                Action::Leave
            }
            (State::Inside, State::Side(s)) => {
                state = State::Side(other(s));
                Action::Neither
            }
            (State::Side(_), State::Inside) => panic!(), // can't happen for geometry reasons
        };
        match a {
            Action::Enter => {
                alt_entered = segment.altitude;
            }
            Action::Leave => {
                acc += segment.altitude - alt_entered + 1;
            }
            Action::Neither => {}
        }
    }
    acc
}

fn solve(instr: Vec<Instruction>) -> isize {
    // a bit redundant, but i like it :3
    let segments = segmentize(instr);
    // we are scanning left to right
    let changes = breakpoints(&segments);
    let bars = segments;
    let mut acc = 0;
    for (i, i1) in changes.iter().copied().tuple_windows() {
        acc += height(&bars, i); // at the edge
        acc += height(&bars, i + 1) * (i1 - i - 1); // between
    }
    acc += height(&bars, *changes.last().unwrap());
    acc
}

// returns places where segments start or end
fn breakpoints(segments: &Vec<Segment>) -> Vec<isize> {
    segments.iter().flat_map(|s| vec![s.start, s.end]).sorted().unique().collect_vec()
}

// returns horizontal segments, sorted ascending by altitute
fn segmentize(instr: Vec<Instruction>) -> Vec<Segment> {
    instr
        .into_iter()
        .scan((0, 0), |pos0, i| {
            let pos1 = step(i.dir, *pos0, i.length);
            let start = pos0.0.min(pos1.0);
            let end = pos0.0.max(pos1.0);
            let altitude = pos0.1;
            let s = Segment { start, end, altitude };
            *pos0 = pos1;
            if horizontal(i.dir) {
                Some(Some(s))
            } else {
                Some(None)
            }
        })
        .filter_map(|x| x)
        .sorted()
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
    #[test]
    fn solver_test() {
        assert_eq!(62, solve(parse("inputs/18a", parse_line)));
        assert_eq!(49061, solve(parse("inputs/18b", parse_line)));

        assert_eq!(952408144115, solve(parse("inputs/18a", parse_line2)));
        assert_eq!(92556825427032, solve(parse("inputs/18b", parse_line2)));
    }
}
